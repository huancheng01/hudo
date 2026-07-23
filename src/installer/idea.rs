use anyhow::Result;
use async_trait::async_trait;
use std::path::PathBuf;

use super::{DetectResult, EnvAction, InstallContext, InstallResult, Installer, ToolInfo};
use crate::config::HudoConfig;
use crate::download;

pub struct IdeaInstaller;

const IDEA_VERSION_DEFAULT: &str = "2025.3";

#[async_trait]
impl Installer for IdeaInstaller {
    fn info(&self) -> ToolInfo {
        ToolInfo {
            id: "idea",
            name: "IntelliJ IDEA",
            description: "IntelliJ IDEA Community IDE",
        }
    }

    async fn detect_installed(&self, ctx: &InstallContext<'_>) -> Result<DetectResult> {
        let idea_exe = ctx.config.ide_dir().join("idea").join("bin").join("idea64.exe");
        if idea_exe.exists() {
            // IDEA 没有简单的 --version，从 product-info.json 读
            let info_file = ctx.config.ide_dir().join("idea").join("product-info.json");
            if let Ok(content) = std::fs::read_to_string(&info_file) {
                if let Ok(val) = serde_json::from_str::<serde_json::Value>(&content) {
                    if let Some(ver) = val.get("version").and_then(|v| v.as_str()) {
                        return Ok(DetectResult::InstalledByHudo(format!("IDEA {}", ver)));
                    }
                }
            }
            return Ok(DetectResult::InstalledByHudo("已安装".to_string()));
        }

        if let Ok(out) = std::process::Command::new("where").arg("idea64").output() {
            if out.status.success() {
                return Ok(DetectResult::InstalledExternal("已安装".to_string()));
            }
        }

        Ok(DetectResult::NotInstalled)
    }

    fn resolve_download(&self, config: &HudoConfig) -> (String, String) {
        let version = config.versions.idea.as_deref().unwrap_or(IDEA_VERSION_DEFAULT);
        (
            apply_mirror(&constructed_url(version), config),
            format!("idea-{}.win.zip", version),
        )
    }

    async fn install(&self, ctx: &InstallContext<'_>) -> Result<InstallResult> {
        let config = ctx.config;
        let install_dir = config.ide_dir().join("idea");

        // 下载链接以 JetBrains API 返回为准：2025.3 起社区版并入统一发行版，
        // 文件名从 ideaIC-*.win.zip 换成 idea-*.win.zip，手拼跨代不可靠
        let (version, url) = match &config.versions.idea {
            Some(v) => {
                let link = crate::version::idea_release_link(v).await;
                match link {
                    Some(l) => (v.clone(), l),
                    None => {
                        crate::ui::print_warning("查询指定版本下载链接失败，按命名规则直接拼接");
                        (v.clone(), constructed_url(v))
                    }
                }
            }
            None => {
                crate::ui::print_action("查询 IntelliJ IDEA 最新版本...");
                match crate::version::idea_latest().await {
                    Some((v, l)) => {
                        crate::ui::print_info(&format!("最新版本: {}", v));
                        (v, l)
                    }
                    None => {
                        crate::ui::print_warning(&format!(
                            "获取最新版本失败，使用内置默认版本 {}",
                            IDEA_VERSION_DEFAULT
                        ));
                        (
                            IDEA_VERSION_DEFAULT.to_string(),
                            constructed_url(IDEA_VERSION_DEFAULT),
                        )
                    }
                }
            }
        };

        let url = apply_mirror(&url, config);
        let filename = format!("idea-{}.win.zip", version);

        let zip_path = download::download(&url, &config.cache_dir(), &filename).await?;

        crate::ui::print_action("解压 IntelliJ IDEA...");
        let tmp_dir = config.cache_dir().join("idea-extract");
        if tmp_dir.exists() {
            std::fs::remove_dir_all(&tmp_dir).ok();
        }
        download::extract_zip(&zip_path, &tmp_dir)?;

        // zip 可能带一层版本号子目录，也可能是平铺结构，两者都兼容
        let inner = download::find_single_subdir(&tmp_dir).unwrap_or(tmp_dir.clone());
        if install_dir.exists() {
            std::fs::remove_dir_all(&install_dir).ok();
        }
        std::fs::rename(&inner, &install_dir).ok();
        std::fs::remove_dir_all(&tmp_dir).ok();

        Ok(InstallResult {
            install_path: install_dir,
            version,
        })
    }

    fn env_actions(&self, install_path: &PathBuf, _config: &HudoConfig) -> Vec<EnvAction> {
        vec![EnvAction::AppendPath {
            path: install_path.join("bin").to_string_lossy().to_string(),
        }]
    }
}

/// 按命名规则拼接下载 URL（API 不可达时的回退）：
/// 2025.3 起统一发行版为 idea-{v}.win.zip，此前社区版为 ideaIC-{v}.win.zip（均实测在源上有效）
fn constructed_url(version: &str) -> String {
    let mut nums = version.split('.').filter_map(|s| s.parse::<u32>().ok());
    let (major, minor) = (nums.next().unwrap_or(0), nums.next().unwrap_or(0));
    let prefix = if (major, minor) >= (2025, 3) { "idea" } else { "ideaIC" };
    format!(
        "https://download.jetbrains.com/idea/{}-{}.win.zip",
        prefix, version
    )
}

/// mirrors.idea 替换 JetBrains 官方下载域（镜像站保持相同路径结构）
fn apply_mirror(url: &str, config: &HudoConfig) -> String {
    match config.mirrors.idea.as_deref() {
        Some(m) => url.replace("https://download.jetbrains.com", m.trim_end_matches('/')),
        None => url.to_string(),
    }
}
