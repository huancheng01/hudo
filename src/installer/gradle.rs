use anyhow::Result;
use async_trait::async_trait;
use std::path::PathBuf;

use super::{DetectResult, EnvAction, InstallContext, InstallResult, Installer, ToolInfo};
use crate::config::HudoConfig;
use crate::download;

pub struct GradleInstaller;

const GRADLE_VERSION_DEFAULT: &str = "8.12.1";

#[async_trait]
impl Installer for GradleInstaller {
    fn info(&self) -> ToolInfo {
        ToolInfo {
            id: "gradle",
            name: "Gradle",
            description: "Gradle 构建工具 (Java/Android)",
        }
    }

    async fn detect_installed(&self, ctx: &InstallContext<'_>) -> Result<DetectResult> {
        // 检查 hudo 安装目录（gradle.bat 需通过 cmd /c 执行）
        let gradle_bat = ctx.config.tools_dir().join("gradle").join("bin").join("gradle.bat");
        if gradle_bat.exists() {
            if let Ok(out) = std::process::Command::new("cmd")
                .args(["/c", &gradle_bat.to_string_lossy(), "--version"])
                .output()
            {
                if out.status.success() {
                    let version = String::from_utf8_lossy(&out.stdout)
                        .lines()
                        .find(|l| l.starts_with("Gradle "))
                        .unwrap_or("已安装")
                        .to_string();
                    return Ok(DetectResult::InstalledByHudo(version));
                }
            }
        }

        // 检查系统 PATH（gradle 是 .bat，通过 cmd /c 调用）
        if let Ok(out) = std::process::Command::new("cmd")
            .args(["/c", "gradle", "--version"])
            .output()
        {
            if out.status.success() {
                let version = String::from_utf8_lossy(&out.stdout)
                    .lines()
                    .find(|l| l.starts_with("Gradle "))
                    .unwrap_or("已安装")
                    .to_string();
                return Ok(DetectResult::InstalledExternal(version));
            }
        }

        Ok(DetectResult::NotInstalled)
    }

    fn resolve_download(&self, config: &HudoConfig) -> (String, String) {
        let version = config.versions.gradle.as_deref().unwrap_or(GRADLE_VERSION_DEFAULT);
        build_url(config, version)
    }

    async fn install(&self, ctx: &InstallContext<'_>) -> Result<InstallResult> {
        let config = ctx.config;
        let install_dir = config.tools_dir().join("gradle");

        // 检测 JDK 是否可用
        super::jdk::ensure_jdk(ctx, "Gradle").await?;

        let version = match &config.versions.gradle {
            Some(v) => v.clone(),
            None => {
                crate::ui::print_action("查询 Gradle 最新版本...");
                match crate::version::gradle_latest().await {
                    Some(v) => {
                        crate::ui::print_info(&format!("最新版本: {}", v));
                        v
                    }
                    None => {
                        crate::ui::print_warning(&format!(
                            "获取最新版本失败，使用内置默认版本 {}",
                            GRADLE_VERSION_DEFAULT
                        ));
                        GRADLE_VERSION_DEFAULT.to_string()
                    }
                }
            }
        };

        let (url, filename) = build_url(config, &version);
        // 下载（回退华为云）
        let fallback_url = format!(
            "https://mirrors.huaweicloud.com/gradle/gradle-{}-bin.zip",
            version
        );
        let zip_path = download::download_with_fallback(&url, &fallback_url, &config.cache_dir(), &filename).await?;

        crate::ui::print_action("解压 Gradle...");
        let tmp_dir = config.cache_dir().join("gradle-extract");
        if tmp_dir.exists() {
            std::fs::remove_dir_all(&tmp_dir).ok();
        }
        download::extract_zip(&zip_path, &tmp_dir)?;

        // zip 内有 gradle-{version}/ 子目录
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

    async fn configure(&self, _ctx: &InstallContext<'_>) -> Result<()> {
        // 生成 ~/.gradle/init.gradle 配置国内镜像仓库（已存在则跳过）
        if let Some(home) = dirs::home_dir() {
            let gradle_dir = home.join(".gradle");
            let init_path = gradle_dir.join("init.gradle");
            if !init_path.exists() {
                std::fs::create_dir_all(&gradle_dir).ok();
                let init_script = r#"allprojects {
    repositories {
        mavenLocal()
        maven { url 'https://maven.aliyun.com/repository/central' }
        maven { url 'https://maven.aliyun.com/repository/public' }
        mavenCentral()
    }
}
"#;
                if std::fs::write(&init_path, init_script).is_ok() {
                    crate::ui::print_success("已生成 Gradle 配置 (~/.gradle/init.gradle)，使用阿里云镜像");
                }
            } else {
                crate::ui::print_info("~/.gradle/init.gradle 已存在，跳过");
            }
        }
        Ok(())
    }

    fn env_actions(&self, install_path: &PathBuf, _config: &HudoConfig) -> Vec<EnvAction> {
        vec![
            EnvAction::Set {
                name: "GRADLE_HOME".to_string(),
                value: install_path.to_string_lossy().to_string(),
            },
            EnvAction::AppendPath {
                path: install_path.join("bin").to_string_lossy().to_string(),
            },
        ]
    }
}

fn build_url(config: &HudoConfig, version: &str) -> (String, String) {
    let base = config
        .mirrors
        .gradle
        .as_deref()
        .unwrap_or("https://services.gradle.org/distributions");
    let url = format!(
        "{}/gradle-{}-bin.zip",
        base.trim_end_matches('/'),
        version
    );
    let filename = format!("gradle-{}-bin.zip", version);
    (url, filename)
}
