use anyhow::Result;
use async_trait::async_trait;
use std::path::PathBuf;

use super::{DetectResult, EnvAction, InstallContext, InstallResult, Installer, ToolInfo};
use crate::config::HudoConfig;
use crate::download;

pub struct PwshInstaller;

const PWSH_VERSION_DEFAULT: &str = "7.6.4";

#[async_trait]
impl Installer for PwshInstaller {
    fn info(&self) -> ToolInfo {
        ToolInfo {
            id: "pwsh",
            name: "PowerShell 7",
            description: "跨平台 PowerShell（系统仅预装 5.1）",
        }
    }

    async fn detect_installed(&self, ctx: &InstallContext<'_>) -> Result<DetectResult> {
        let pwsh_exe = ctx.config.tools_dir().join("pwsh").join("pwsh.exe");
        if pwsh_exe.exists() {
            if let Some(v) = pwsh_version(&pwsh_exe) {
                return Ok(DetectResult::InstalledByHudo(v));
            }
            return Ok(DetectResult::InstalledByHudo("已安装".to_string()));
        }

        // 系统的 pwsh（MSI 安装或 winget）：Windows 自带的 5.1 是 powershell.exe，不算
        if let Ok(out) = std::process::Command::new("pwsh").args(["-NoProfile", "-Version"]).output() {
            if out.status.success() {
                let text = String::from_utf8_lossy(&out.stdout).trim().to_string();
                return Ok(DetectResult::InstalledExternal(text));
            }
        }

        Ok(DetectResult::NotInstalled)
    }

    fn resolve_download(&self, config: &HudoConfig) -> (String, String) {
        let version = config.versions.pwsh.as_deref().unwrap_or(PWSH_VERSION_DEFAULT);
        (download_url(version), zip_filename(version))
    }

    async fn install(&self, ctx: &InstallContext<'_>) -> Result<InstallResult> {
        let config = ctx.config;
        let install_dir = config.tools_dir().join("pwsh");

        let version = match &config.versions.pwsh {
            Some(v) => v.clone(),
            None => {
                crate::ui::print_action("查询 PowerShell 最新版本...");
                match crate::version::pwsh_latest().await {
                    Some(v) => {
                        crate::ui::print_info(&format!("最新版本: {}", v));
                        v
                    }
                    None => {
                        crate::ui::print_warning(&format!(
                            "获取最新版本失败，使用内置默认版本 {}",
                            PWSH_VERSION_DEFAULT
                        ));
                        PWSH_VERSION_DEFAULT.to_string()
                    }
                }
            }
        };

        let filename = zip_filename(&version);
        let zip_path = download::download(&download_url(&version), &config.cache_dir(), &filename).await?;

        crate::ui::print_action("解压 PowerShell 7...");
        // zip 为平铺结构（pwsh.exe 在根目录），直接解到安装目录
        let tmp_dir = config.cache_dir().join("pwsh-extract");
        if tmp_dir.exists() {
            std::fs::remove_dir_all(&tmp_dir).ok();
        }
        download::extract_zip(&zip_path, &tmp_dir)?;

        if !tmp_dir.join("pwsh.exe").exists() {
            anyhow::bail!("解压后未找到 pwsh.exe，安装可能失败");
        }

        if install_dir.exists() {
            std::fs::remove_dir_all(&install_dir).ok();
        }
        std::fs::rename(&tmp_dir, &install_dir)
            .map_err(|e| anyhow::anyhow!("移动 PowerShell 文件失败: {}", e))?;

        Ok(InstallResult {
            install_path: install_dir,
            version,
        })
    }

    fn env_actions(&self, install_path: &PathBuf, _config: &HudoConfig) -> Vec<EnvAction> {
        vec![EnvAction::AppendPath {
            path: install_path.to_string_lossy().to_string(),
        }]
    }
}

fn download_url(version: &str) -> String {
    format!(
        "https://github.com/PowerShell/PowerShell/releases/download/v{}/PowerShell-{}-win-x64.zip",
        version, version
    )
}

fn zip_filename(version: &str) -> String {
    format!("PowerShell-{}-win-x64.zip", version)
}

/// `pwsh -NoProfile -Version` 输出 "PowerShell 7.6.4"
fn pwsh_version(exe: &std::path::Path) -> Option<String> {
    let out = std::process::Command::new(exe)
        .args(["-NoProfile", "-Version"])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if text.is_empty() {
        None
    } else {
        Some(text)
    }
}
