use anyhow::{Context, Result};
use async_trait::async_trait;
use std::path::PathBuf;

use super::{DetectResult, EnvAction, InstallContext, InstallResult, Installer, ToolInfo};
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

        // 解析版本: config > API > hardcoded
        let version = match &config.versions.nodejs {
            Some(v) => v.clone(),
            None => {
                crate::ui::print_action("查询 Node.js 最新 LTS 版本...");
                crate::version::nodejs_lts_latest()
                    .await
                    .unwrap_or_else(|| NODEJS_VERSION_DEFAULT.to_string())
            }
        };

        let filename = format!("node-v{}-win-x64.zip", version);
        let default_base = format!("https://nodejs.org/dist/v{}", version);
        let base = config.mirrors.nodejs.as_deref().unwrap_or(&default_base);
        let url = format!("{}/{}", base.trim_end_matches('/'), filename);

        // 下载 zip
        let zip_path = download::download(&url, &config.cache_dir(), &filename).await?;

        // 解压到临时目录，再移到 lang/node/
        crate::ui::print_action("解压 Node.js...");
        let tmp_dir = config.cache_dir().join("node-extract");
        if tmp_dir.exists() {
            std::fs::remove_dir_all(&tmp_dir).ok();
        }
        download::extract_zip(&zip_path, &tmp_dir)?;

        // zip 内有 node-v{VERSION}-win-x64/ 子目录
        if install_dir.exists() {
            std::fs::remove_dir_all(&install_dir).ok();
        }
        let inner = tmp_dir.join(format!("node-v{}-win-x64", version));
        if inner.exists() {
            std::fs::rename(&inner, &install_dir).context("移动 Node.js 文件失败")?;
        } else {
            // 尝试找唯一子目录
            if let Some(sub) = download::find_single_subdir(&tmp_dir) {
                std::fs::rename(&sub, &install_dir).context("移动 Node.js 文件失败")?;
            } else {
                std::fs::rename(&tmp_dir, &install_dir).context("移动 Node.js 文件失败")?;
            }
        }
        std::fs::remove_dir_all(&tmp_dir).ok();

        let installed_version =
            get_node_version(&install_dir).unwrap_or_else(|| format!("v{}", version));

        Ok(InstallResult {
            install_path: install_dir,
            version: installed_version,
        })
    }

    fn env_actions(&self, install_path: &PathBuf, _config: &HudoConfig) -> Vec<EnvAction> {
        vec![EnvAction::AppendPath {
            path: install_path.to_string_lossy().to_string(),
        }]
    }
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
