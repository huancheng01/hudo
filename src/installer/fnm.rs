use anyhow::{Context, Result};
use async_trait::async_trait;
use std::path::PathBuf;

use super::{DetectResult, EnvAction, InstallContext, InstallResult, Installer, ToolInfo};
use crate::config::HudoConfig;
use crate::download;

pub struct FnmInstaller;

const FNM_VERSION_DEFAULT: &str = "1.38.1";

#[async_trait]
impl Installer for FnmInstaller {
    fn info(&self) -> ToolInfo {
        ToolInfo {
            id: "fnm",
            name: "fnm",
            description: "Node.js 版本管理器（多版本切换）",
        }
    }

    async fn detect_installed(&self, ctx: &InstallContext<'_>) -> Result<DetectResult> {
        let fnm_exe = ctx.config.tools_dir().join("fnm").join("fnm.exe");
        if fnm_exe.exists() {
            if let Ok(out) = std::process::Command::new(&fnm_exe).arg("--version").output() {
                if out.status.success() {
                    let version = String::from_utf8_lossy(&out.stdout).trim().to_string();
                    return Ok(DetectResult::InstalledByHudo(version));
                }
            }
        }

        if let Ok(out) = std::process::Command::new("fnm").arg("--version").output() {
            if out.status.success() {
                let version = String::from_utf8_lossy(&out.stdout).trim().to_string();
                return Ok(DetectResult::InstalledExternal(version));
            }
        }

        Ok(DetectResult::NotInstalled)
    }

    fn resolve_download(&self, config: &HudoConfig) -> (String, String) {
        let filename = "fnm-windows.zip".to_string();
        match &config.versions.fnm {
            Some(fnm_version) => {
                let default_base = format!(
                    "https://github.com/Schniz/fnm/releases/download/v{}",
                    fnm_version
                );
                let base = config.mirrors.fnm.as_deref().unwrap_or(&default_base);
                let url = format!("{}/{}", base.trim_end_matches('/'), filename);
                (url, filename)
            }
            None => {
                let base = config
                    .mirrors
                    .fnm
                    .as_deref()
                    .unwrap_or("https://github.com/Schniz/fnm/releases/latest/download");
                let url = format!("{}/{}", base.trim_end_matches('/'), filename);
                (url, filename)
            }
        }
    }

    async fn install(&self, ctx: &InstallContext<'_>) -> Result<InstallResult> {
        let config = ctx.config;
        let fnm_dir = config.tools_dir().join("fnm");
        let node_dir = fnm_node_dir(config);
        let (url, filename) = self.resolve_download(config);

        if config.versions.fnm.is_none() {
            let cached = config.cache_dir().join(&filename);
            if cached.exists() {
                std::fs::remove_file(&cached).ok();
            }
        }

        let zip_path = download::download(&url, &config.cache_dir(), &filename).await?;

        crate::ui::print_action("解压 fnm...");
        std::fs::create_dir_all(&fnm_dir).ok();
        download::extract_zip(&zip_path, &fnm_dir)?;

        std::fs::create_dir_all(&node_dir).ok();

        crate::ui::print_action("通过 fnm 安装 Node.js LTS...");
        let fnm_exe = fnm_dir.join("fnm.exe");
        let status = std::process::Command::new(&fnm_exe)
            .args(["install", "--lts"])
            .env("FNM_DIR", &node_dir)
            .status()
            .context("fnm install --lts 失败")?;

        if !status.success() {
            anyhow::bail!(
                "fnm install 失败，退出码: {}",
                status.code().unwrap_or(-1)
            );
        }

        std::process::Command::new(&fnm_exe)
            .args(["default", "lts-latest"])
            .env("FNM_DIR", &node_dir)
            .status()
            .ok();

        let version = get_fnm_version(&fnm_dir).unwrap_or_else(|| {
            config
                .versions
                .fnm
                .as_deref()
                .unwrap_or(FNM_VERSION_DEFAULT)
                .to_string()
        });

        Ok(InstallResult {
            install_path: fnm_dir,
            version,
        })
    }

    fn env_actions(&self, install_path: &PathBuf, config: &HudoConfig) -> Vec<EnvAction> {
        let node_dir = fnm_node_dir(config);
        vec![
            EnvAction::Set {
                name: "FNM_DIR".to_string(),
                value: node_dir.to_string_lossy().to_string(),
            },
            EnvAction::AppendPath {
                path: install_path.to_string_lossy().to_string(),
            },
        ]
    }

    async fn configure(&self, ctx: &InstallContext<'_>) -> Result<()> {
        let fnm_dir = ctx.config.tools_dir().join("fnm");
        let fnm_exe = fnm_dir.join("fnm.exe");

        if let Err(e) = write_powershell_profile(&fnm_exe) {
            crate::ui::print_warning(&format!("写入 PowerShell profile 失败: {}", e));
            crate::ui::print_info("请手动在 $PROFILE 中添加：");
            crate::ui::print_info("  fnm env --use-on-cd --shell power-shell | Out-String | Invoke-Expression");
        }

        Ok(())
    }
}

/// fnm 管理的 Node.js 版本目录，独立于纯 nodejs 安装器的 lang/node/
fn fnm_node_dir(config: &HudoConfig) -> PathBuf {
    config.lang_dir().join("node-fnm")
}

fn write_powershell_profile(fnm_exe: &std::path::Path) -> Result<()> {
    let output = std::process::Command::new("powershell")
        .args(["-NoProfile", "-Command", "$PROFILE"])
        .output()
        .context("无法获取 PowerShell profile 路径")?;

    let profile_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if profile_path.is_empty() {
        anyhow::bail!("PowerShell $PROFILE 路径为空");
    }
    let profile_path = std::path::Path::new(&profile_path);

    if let Some(parent) = profile_path.parent() {
        std::fs::create_dir_all(parent).ok();
    }

    let init_line = format!(
        "# fnm (Node.js version manager)\r\n& '{}' env --use-on-cd --shell power-shell | Out-String | Invoke-Expression",
        fnm_exe.display()
    );

    let existing = std::fs::read_to_string(profile_path).unwrap_or_default();
    if existing.contains("fnm env") {
        crate::ui::print_info("PowerShell profile 已包含 fnm 初始化，跳过");
        return Ok(());
    }

    use std::io::Write;
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(profile_path)
        .context("打开 PowerShell profile 失败")?;

    if !existing.is_empty() && !existing.ends_with('\n') {
        writeln!(file)?;
    }
    writeln!(file, "\r\n{}", init_line)?;

    crate::ui::print_success("已写入 PowerShell profile，重开终端后 node 命令即可使用");
    Ok(())
}

fn get_fnm_version(fnm_dir: &PathBuf) -> Option<String> {
    let fnm_exe = fnm_dir.join("fnm.exe");
    std::process::Command::new(fnm_exe)
        .arg("--version")
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
}
