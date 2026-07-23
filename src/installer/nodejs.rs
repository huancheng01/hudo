use anyhow::{Context, Result};
use async_trait::async_trait;
use std::path::PathBuf;

use super::{
    clean_fnm_profile_lines, force_remove_dir_all, is_legacy_fnm_dir, DetectResult, EnvAction,
    InstallContext, InstallResult, Installer, ToolInfo,
};
use crate::config::HudoConfig;
use crate::download;

pub struct NodejsInstaller;

const NODEJS_VERSION_DEFAULT: &str = "24.14.1";

#[async_trait]
impl Installer for NodejsInstaller {
    fn info(&self) -> ToolInfo {
        ToolInfo {
            id: "nodejs",
            name: "Node.js",
            description: "Node.js 运行时",
        }
    }

    async fn detect_installed(&self, ctx: &InstallContext<'_>) -> Result<DetectResult> {
        // 检查 hudo 安装的 node
        let node_exe = ctx.config.lang_dir().join("node").join("node.exe");
        if node_exe.exists() {
            if let Ok(out) = std::process::Command::new(&node_exe).arg("--version").output() {
                if out.status.success() {
                    let version = String::from_utf8_lossy(&out.stdout).trim().to_string();
                    return Ok(DetectResult::InstalledByHudo(version));
                }
            }
        }

        // 兼容旧版 hudo：fnm 用 `nodejs` id 记录，install_path 指向 tools/fnm
        if let Some(ver) = legacy_fnm_version(ctx.config) {
            return Ok(DetectResult::InstalledByHudo(format!("legacy fnm {}", ver)));
        }

        // 检查系统 PATH 上的 node
        if let Ok(out) = std::process::Command::new("node").arg("--version").output() {
            if out.status.success() {
                let version = String::from_utf8_lossy(&out.stdout).trim().to_string();
                return Ok(DetectResult::InstalledExternal(version));
            }
        }

        Ok(DetectResult::NotInstalled)
    }

    fn resolve_download(&self, config: &HudoConfig) -> (String, String) {
        let version = config
            .versions
            .nodejs
            .as_deref()
            .unwrap_or(NODEJS_VERSION_DEFAULT);
        let filename = format!("node-v{}-win-x64.zip", version);
        let default_base = format!("https://nodejs.org/dist/v{}", version);
        let base = config.mirrors.nodejs.as_deref().unwrap_or(&default_base);
        let url = format!("{}/{}", base.trim_end_matches('/'), filename);
        (url, filename)
    }

    async fn install(&self, ctx: &InstallContext<'_>) -> Result<InstallResult> {
        let config = ctx.config;
        let install_dir = config.lang_dir().join("node");

        // 旧版 fnm 残留自动迁移：征得同意后清理 lang/node/、tools/fnm/、PowerShell profile
        if is_legacy_fnm_dir(&install_dir) || legacy_fnm_version(config).is_some() {
            crate::ui::print_warning("检测到旧版 fnm 残留（由 hudo 0.2.12 及更早版本安装）");
            let confirm = crate::ui::confirm_proceed("是否清理旧版 fnm 残留后继续安装 Node.js？", true)?;
            if !confirm {
                anyhow::bail!("已取消，请先运行 `hudo uninstall nodejs` 清理旧版 fnm 再重试");
            }
            cleanup_legacy_fnm(config)?;
        }

        // 解析版本: config > API > hardcoded
        let version = match &config.versions.nodejs {
            Some(v) => v.clone(),
            None => {
                crate::ui::print_action("查询 Node.js 最新 LTS 版本...");
                match crate::version::nodejs_lts_latest().await {
                    Some(v) => {
                        crate::ui::print_info(&format!("最新 LTS 版本: {}", v));
                        v
                    }
                    None => {
                        crate::ui::print_warning(&format!(
                            "获取最新版本失败，使用内置默认版本 {}",
                            NODEJS_VERSION_DEFAULT
                        ));
                        NODEJS_VERSION_DEFAULT.to_string()
                    }
                }
            }
        };

        let filename = format!("node-v{}-win-x64.zip", version);
        let default_base = format!("https://nodejs.org/dist/v{}", version);
        let base = config.mirrors.nodejs.as_deref().unwrap_or(&default_base);
        let url = format!("{}/{}", base.trim_end_matches('/'), filename);

        // 下载 zip（回退 npmmirror）
        let fallback_url = format!(
            "https://npmmirror.com/mirrors/node/v{}/{}",
            version, filename
        );
        let zip_path = download::download_with_fallback(&url, &fallback_url, &config.cache_dir(), &filename).await?;

        // 解压到临时目录，再移到 lang/node/
        crate::ui::print_action("解压 Node.js...");
        let tmp_dir = config.cache_dir().join("node-extract");
        if tmp_dir.exists() {
            force_remove_dir_all(&tmp_dir).ok();
        }
        download::extract_zip(&zip_path, &tmp_dir)?;

        // zip 内有 node-v{VERSION}-win-x64/ 子目录
        if install_dir.exists() {
            if let Err(e) = force_remove_dir_all(&install_dir) {
                anyhow::bail!(
                    "无法清理已存在的目录 {}。\n\
                     可能有进程占用文件，关闭所有 node/npm 终端后重试。\n\
                     底层错误: {}",
                    install_dir.display(),
                    e
                );
            }
        }
        let inner = tmp_dir.join(format!("node-v{}-win-x64", version));
        let source = if inner.exists() {
            inner
        } else if let Some(sub) = download::find_single_subdir(&tmp_dir) {
            sub
        } else {
            tmp_dir.clone()
        };
        std::fs::rename(&source, &install_dir).with_context(|| {
            format!(
                "移动 Node.js 文件失败: {} -> {}",
                source.display(),
                install_dir.display()
            )
        })?;
        force_remove_dir_all(&tmp_dir).ok();

        let installed_version =
            get_node_version(&install_dir).unwrap_or_else(|| format!("v{}", version));

        Ok(InstallResult {
            install_path: install_dir,
            version: installed_version,
        })
    }

    fn env_actions(&self, install_path: &PathBuf, config: &HudoConfig) -> Vec<EnvAction> {
        let mut actions = vec![EnvAction::AppendPath {
            path: install_path.to_string_lossy().to_string(),
        }];
        // 旧版 fnm 残留：当初还设了 FNM_DIR，一并清理
        if legacy_fnm_version(config).is_some() {
            actions.push(EnvAction::Set {
                name: "FNM_DIR".to_string(),
                value: config.lang_dir().join("node").to_string_lossy().to_string(),
            });
        }
        actions
    }

    async fn pre_uninstall(&self, ctx: &InstallContext<'_>) -> Result<()> {
        let config = ctx.config;
        // 旧版 fnm 残留：清 lang/node/、tools/fnm/、PowerShell profile
        if legacy_fnm_version(config).is_some() || is_legacy_fnm_dir(&config.lang_dir().join("node")) {
            crate::ui::print_action("清理旧版 fnm 残留...");
            let node_dir = config.lang_dir().join("node");
            if node_dir.exists() {
                if let Err(e) = force_remove_dir_all(&node_dir) {
                    crate::ui::print_warning(&format!("删除 {} 失败: {}", node_dir.display(), e));
                } else {
                    crate::ui::print_info(&format!("已删除 {}", node_dir.display()));
                }
            }
            let fnm_dir = config.tools_dir().join("fnm");
            if fnm_dir.exists() {
                force_remove_dir_all(&fnm_dir).ok();
                crate::ui::print_info(&format!("已删除 {}", fnm_dir.display()));
            }
            match clean_fnm_profile_lines() {
                Ok(true) => crate::ui::print_info("已清理 PowerShell profile 中的 fnm 初始化行"),
                Ok(false) => {}
                Err(e) => crate::ui::print_warning(&format!("清理 PowerShell profile 失败: {}", e)),
            }
        }
        Ok(())
    }
}

/// 若 state.json 的 `nodejs` 条目其实是旧版 fnm（install_path 含 fnm 或 version 以 `fnm ` 开头），
/// 返回记录的 version；否则 None
fn legacy_fnm_version(config: &HudoConfig) -> Option<String> {
    let reg = crate::registry::InstallRegistry::load(&config.state_path()).ok()?;
    let entry = reg.get("nodejs")?;
    let path_hint = entry.install_path.to_lowercase();
    let ver_hint = entry.version.to_lowercase();
    if path_hint.contains("fnm") || ver_hint.starts_with("fnm ") {
        Some(entry.version.clone())
    } else {
        None
    }
}

fn cleanup_legacy_fnm(config: &HudoConfig) -> Result<()> {
    crate::ui::print_action("清理旧版 fnm 残留...");
    let node_dir = config.lang_dir().join("node");
    if node_dir.exists() {
        force_remove_dir_all(&node_dir)
            .with_context(|| format!("删除 {} 失败（可能有进程占用）", node_dir.display()))?;
        crate::ui::print_info(&format!("已删除 {}", node_dir.display()));
    }
    let fnm_dir = config.tools_dir().join("fnm");
    if fnm_dir.exists() {
        force_remove_dir_all(&fnm_dir).ok();
        crate::ui::print_info(&format!("已删除 {}", fnm_dir.display()));
    }

    // 清 state.json 中旧的 nodejs 条目（若指向 fnm）
    if legacy_fnm_version(config).is_some() {
        let mut reg = crate::registry::InstallRegistry::load(&config.state_path())?;
        reg.remove("nodejs");
        reg.save(&config.state_path()).ok();
    }

    // 清 FNM_DIR 环境变量（若指向 lang/node）
    if let Ok(Some(val)) = crate::env::EnvManager::get_var("FNM_DIR") {
        let target = config.lang_dir().join("node").to_string_lossy().to_lowercase();
        if val.to_lowercase() == target {
            crate::env::EnvManager::delete_var("FNM_DIR").ok();
            crate::ui::print_info("已移除 FNM_DIR 环境变量");
        }
    }

    match clean_fnm_profile_lines() {
        Ok(true) => crate::ui::print_info("已清理 PowerShell profile 中的 fnm 初始化行"),
        Ok(false) => {}
        Err(e) => crate::ui::print_warning(&format!("清理 PowerShell profile 失败: {}", e)),
    }
    Ok(())
}

fn get_node_version(node_dir: &PathBuf) -> Option<String> {
    let node_exe = node_dir.join("node.exe");
    std::process::Command::new(node_exe)
        .arg("--version")
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
}
