use anyhow::{Context, Result};
use async_trait::async_trait;
use std::path::PathBuf;

use super::{DetectResult, EnvAction, InstallContext, InstallResult, Installer, ToolInfo};
use crate::config::HudoConfig;
use crate::download;

pub struct SevenzipInstaller;

const SEVENZIP_VERSION_DEFAULT: &str = "26.02";

#[async_trait]
impl Installer for SevenzipInstaller {
    fn info(&self) -> ToolInfo {
        ToolInfo {
            id: "7zip",
            name: "7-Zip",
            description: "压缩/解压工具（含命令行 7z 与图形界面 7zFM）",
        }
    }

    async fn detect_installed(&self, ctx: &InstallContext<'_>) -> Result<DetectResult> {
        let seven_exe = ctx.config.tools_dir().join("7zip").join("7z.exe");
        if seven_exe.exists() {
            if let Some(v) = banner_version(&seven_exe) {
                return Ok(DetectResult::InstalledByHudo(v));
            }
            return Ok(DetectResult::InstalledByHudo("已安装".to_string()));
        }

        // 只查 PATH 上的 7z：官方安装器装在 Program Files 且默认不进 PATH，
        // 与 hudo 便携版可无冲突共存，不提供一个注定失败的接管路径
        if let Ok(out) = std::process::Command::new("where").arg("7z").output() {
            if out.status.success() {
                let path = String::from_utf8_lossy(&out.stdout).lines().next().unwrap_or("").to_string();
                if !path.is_empty() {
                    if let Some(v) = banner_version(std::path::Path::new(&path)) {
                        return Ok(DetectResult::InstalledExternal(v));
                    }
                    return Ok(DetectResult::InstalledExternal("已安装".to_string()));
                }
            }
        }

        Ok(DetectResult::NotInstalled)
    }

    fn resolve_download(&self, config: &HudoConfig) -> (String, String) {
        let version = config.versions.sevenzip.as_deref().unwrap_or(SEVENZIP_VERSION_DEFAULT);
        let filename = installer_filename(version);
        (
            format!("https://www.7-zip.org/a/{}", filename),
            filename,
        )
    }

    async fn install(&self, ctx: &InstallContext<'_>) -> Result<InstallResult> {
        let config = ctx.config;
        let install_dir = config.tools_dir().join("7zip");

        let version = match &config.versions.sevenzip {
            Some(v) => v.clone(),
            None => {
                crate::ui::print_action("查询 7-Zip 最新版本...");
                match crate::version::sevenzip_latest().await {
                    Some(v) => {
                        crate::ui::print_info(&format!("最新版本: {}", v));
                        v
                    }
                    None => {
                        crate::ui::print_warning(&format!(
                            "获取最新版本失败，使用内置默认版本 {}",
                            SEVENZIP_VERSION_DEFAULT
                        ));
                        SEVENZIP_VERSION_DEFAULT.to_string()
                    }
                }
            }
        };

        // 便携方案：官方安装器本体是 7z SFX，用官方 7zr.exe 解出全部载荷，
        // 免管理员、不写注册表（代价：无资源管理器右键集成，那需要注册 DLL）
        let sevenzr = download::download_with_fallback(
            "https://www.7-zip.org/a/7zr.exe",
            &format!(
                "https://github.com/ip7z/7zip/releases/download/{}/7zr.exe",
                version
            ),
            &config.cache_dir(),
            "7zr.exe",
        )
        .await?;

        let filename = installer_filename(&version);
        let exe_path = download::download_with_fallback(
            &format!("https://www.7-zip.org/a/{}", filename),
            &format!(
                "https://github.com/ip7z/7zip/releases/download/{}/{}",
                version, filename
            ),
            &config.cache_dir(),
            &filename,
        )
        .await?;

        let tmp_dir = config.cache_dir().join("7zip-extract");
        if tmp_dir.exists() {
            std::fs::remove_dir_all(&tmp_dir).ok();
        }

        let sp = crate::ui::spinner("解包 7-Zip...");
        let output = std::process::Command::new(&sevenzr)
            .arg("x")
            .arg(&exe_path)
            .arg(format!("-o{}", tmp_dir.display()))
            .arg("-y")
            .output();
        sp.finish_and_clear();
        let output = output.context("启动 7zr.exe 失败")?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("7-Zip 解包失败: {}", stderr.trim());
        }

        if !tmp_dir.join("7z.exe").exists() {
            anyhow::bail!("解包后未找到 7z.exe，安装可能失败");
        }

        if install_dir.exists() {
            std::fs::remove_dir_all(&install_dir).ok();
        }
        std::fs::rename(&tmp_dir, &install_dir).context("移动 7-Zip 文件失败")?;

        let version = banner_version(&install_dir.join("7z.exe")).unwrap_or(version);

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

/// "26.02" → "7z2602-x64.exe"（官方文件名用去点的紧凑版本号）
fn installer_filename(version: &str) -> String {
    format!("7z{}-x64.exe", version.replace('.', ""))
}

/// 无参运行 7z 输出横幅 "7-Zip 26.02 (x64) : ..."
fn banner_version(exe: &std::path::Path) -> Option<String> {
    let out = std::process::Command::new(exe).output().ok()?;
    let text = String::from_utf8_lossy(&out.stdout);
    text.lines().find_map(parse_banner_line)
}

/// 版本 = 首个纯数字点号 token（产品名 "7-Zip" 也是数字开头，不能只按首字符判）
fn parse_banner_line(line: &str) -> Option<String> {
    if !line.contains("7-Zip") {
        return None;
    }
    line.split_whitespace()
        .find(|t| t.chars().all(|c| c.is_ascii_digit() || c == '.') && t.contains('.'))
        .map(|s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_banner_line() {
        assert_eq!(
            parse_banner_line("7-Zip 26.02 (x64) : Copyright (c) 1999-2026 Igor Pavlov : 2026-06-25"),
            Some("26.02".to_string())
        );
        assert_eq!(parse_banner_line("Usage: 7z <command>"), None);
    }
}
