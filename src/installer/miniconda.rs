use anyhow::{Context, Result};
use async_trait::async_trait;
use std::path::PathBuf;

use super::{DetectResult, EnvAction, InstallContext, InstallResult, Installer, ToolInfo};
use crate::config::HudoConfig;
use crate::download;

pub struct MinicondaInstaller;

#[async_trait]
impl Installer for MinicondaInstaller {
    fn info(&self) -> ToolInfo {
        ToolInfo {
            id: "miniconda",
            name: "Miniconda",
            description: "Conda 包管理器（最小安装）",
        }
    }

    async fn detect_installed(&self, ctx: &InstallContext<'_>) -> Result<DetectResult> {
        let conda_exe = ctx.config.tools_dir().join("miniconda").join("Scripts").join("conda.exe");
        if conda_exe.exists() {
            if let Ok(out) = std::process::Command::new(&conda_exe).arg("--version").output() {
                if out.status.success() {
                    let version = String::from_utf8_lossy(&out.stdout).trim().to_string();
                    return Ok(DetectResult::InstalledByHudo(version));
                }
            }
        }

        if let Ok(out) = std::process::Command::new("conda").arg("--version").output() {
            if out.status.success() {
                let version = String::from_utf8_lossy(&out.stdout).trim().to_string();
                return Ok(DetectResult::InstalledExternal(version));
            }
        }

        Ok(DetectResult::NotInstalled)
    }

    fn resolve_download(&self, _config: &HudoConfig) -> (String, String) {
        (
            "https://repo.anaconda.com/miniconda/Miniconda3-latest-Windows-x86_64.exe".to_string(),
            "Miniconda3-latest-Windows-x86_64.exe".to_string(),
        )
    }

    async fn install(&self, ctx: &InstallContext<'_>) -> Result<InstallResult> {
        let config = ctx.config;
        let install_dir = config.tools_dir().join("miniconda");
        let (url, filename) = self.resolve_download(config);

        // 下载（回退 TUNA 镜像）
        let fallback_url = "https://mirrors.tuna.tsinghua.edu.cn/anaconda/miniconda/Miniconda3-latest-Windows-x86_64.exe";
        let exe_path = download::download_with_fallback(&url, fallback_url, &config.cache_dir(), &filename).await?;

        // Miniconda 支持静默安装到指定目录
        crate::ui::print_action("安装 Miniconda（静默模式）...");
        let status = std::process::Command::new(&exe_path)
            .args([
                "/InstallationType=JustMe",                     // 仅当前用户，不写 HKLM
                "/RegisterPython=0",                            // 不注册为系统 Python
                "/AddToPath=0",                                 // 不自动加 PATH
                "/S",                                           // 静默
                &format!("/D={}", install_dir.display()),       // 指定安装目录（必须最后）
            ])
            .status()
            .context("启动 Miniconda 安装程序失败")?;

        if !status.success() {
            anyhow::bail!(
                "Miniconda 安装失败，退出码: {}",
                status.code().unwrap_or(-1)
            );
        }

        let version = get_conda_version(&install_dir).unwrap_or_else(|| "latest".to_string());

        Ok(InstallResult {
            install_path: install_dir,
            version,
        })
    }

    async fn configure(&self, ctx: &InstallContext<'_>) -> Result<()> {
        let install_dir = ctx.config.tools_dir().join("miniconda");
        let conda_exe = install_dir.join("Scripts").join("conda.exe");

        // 初始化 cmd.exe 和 PowerShell，使 conda activate 可用
        for shell in &["cmd.exe", "powershell"] {
            let status = std::process::Command::new(&conda_exe)
                .args(["init", shell])
                .status();
            match status {
                Ok(s) if s.success() => {
                    crate::ui::print_success(&format!("已初始化 conda ({})", shell));
                }
                _ => {
                    crate::ui::print_warning(&format!("conda init {} 失败，可手动执行", shell));
                }
            }
        }

        Ok(())
    }

    fn env_actions(&self, install_path: &PathBuf, _config: &HudoConfig) -> Vec<EnvAction> {
        vec![
            EnvAction::AppendPath {
                path: install_path.to_string_lossy().to_string(),
            },
            EnvAction::AppendPath {
                path: install_path.join("Scripts").to_string_lossy().to_string(),
            },
            EnvAction::AppendPath {
                path: install_path.join("Library").join("bin").to_string_lossy().to_string(),
            },
        ]
    }
}

fn get_conda_version(install_dir: &PathBuf) -> Option<String> {
    let conda = install_dir.join("Scripts").join("conda.exe");
    std::process::Command::new(conda)
        .arg("--version")
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
}
