use anyhow::Result;
use async_trait::async_trait;
use std::path::PathBuf;

use super::{DetectResult, EnvAction, InstallContext, InstallResult, Installer, ToolInfo};
use crate::config::HudoConfig;
use crate::download;

pub struct MavenInstaller;

const MAVEN_VERSION_DEFAULT: &str = "3.9.9";

#[async_trait]
impl Installer for MavenInstaller {
    fn info(&self) -> ToolInfo {
        ToolInfo {
            id: "maven",
            name: "Maven",
            description: "Apache Maven 构建工具 (Java)",
        }
    }

    async fn detect_installed(&self, ctx: &InstallContext<'_>) -> Result<DetectResult> {
        // 检查 hudo 安装目录（mvn.cmd 需通过 cmd /c 执行）
        let mvn_cmd = ctx.config.tools_dir().join("maven").join("bin").join("mvn.cmd");
        if mvn_cmd.exists() {
            if let Ok(out) = std::process::Command::new("cmd")
                .args(["/c", &mvn_cmd.to_string_lossy(), "--version"])
                .output()
            {
                if out.status.success() {
                    let version = String::from_utf8_lossy(&out.stdout)
                        .lines()
                        .next()
                        .unwrap_or("已安装")
                        .to_string();
                    return Ok(DetectResult::InstalledByHudo(version));
                }
            }
        }

        // 检查系统 PATH（mvn 是 .cmd，通过 cmd /c 调用）
        if let Ok(out) = std::process::Command::new("cmd")
            .args(["/c", "mvn", "--version"])
            .output()
        {
            if out.status.success() {
                let version = String::from_utf8_lossy(&out.stdout)
                    .lines()
                    .next()
                    .unwrap_or("已安装")
                    .to_string();
                return Ok(DetectResult::InstalledExternal(version));
            }
        }

        Ok(DetectResult::NotInstalled)
    }

    fn resolve_download(&self, config: &HudoConfig) -> (String, String) {
        let version = config.versions.maven.as_deref().unwrap_or(MAVEN_VERSION_DEFAULT);
        let (url, filename) = build_url(config, version);
        (url, filename)
    }

    async fn install(&self, ctx: &InstallContext<'_>) -> Result<InstallResult> {
        let config = ctx.config;
        let install_dir = config.tools_dir().join("maven");

        // 检测 JDK 是否可用
        super::jdk::ensure_jdk(ctx, "Maven").await?;

        let version = match &config.versions.maven {
            Some(v) => v.clone(),
            None => {
                crate::ui::print_action("查询 Maven 最新版本...");
                match crate::version::maven_latest().await {
                    Some(v) => {
                        crate::ui::print_info(&format!("最新版本: {}", v));
                        v
                    }
                    None => {
                        crate::ui::print_warning(&format!(
                            "获取最新版本失败，使用内置默认版本 {}",
                            MAVEN_VERSION_DEFAULT
                        ));
                        MAVEN_VERSION_DEFAULT.to_string()
                    }
                }
            }
        };

        let (url, filename) = build_url(config, &version);
        // 下载（回退华为云）
        let fallback_url = format!(
            "https://mirrors.huaweicloud.com/apache/maven/maven-3/{}/binaries/{}",
            version, filename
        );
        let zip_path = download::download_with_fallback(&url, &fallback_url, &config.cache_dir(), &filename).await?;

        crate::ui::print_action("解压 Maven...");
        let tmp_dir = config.cache_dir().join("maven-extract");
        if tmp_dir.exists() {
            std::fs::remove_dir_all(&tmp_dir).ok();
        }
        download::extract_zip(&zip_path, &tmp_dir)?;

        // zip 内有 apache-maven-{version}/ 子目录
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
        // 生成 ~/.m2/settings.xml 配置阿里云镜像（已存在则跳过）
        if let Some(home) = dirs::home_dir() {
            let m2_dir = home.join(".m2");
            let settings_path = m2_dir.join("settings.xml");
            if !settings_path.exists() {
                std::fs::create_dir_all(&m2_dir).ok();
                let settings = r#"<?xml version="1.0" encoding="UTF-8"?>
<settings xmlns="http://maven.apache.org/SETTINGS/1.2.0"
          xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
          xsi:schemaLocation="http://maven.apache.org/SETTINGS/1.2.0 https://maven.apache.org/xsd/settings-1.2.0.xsd">
  <mirrors>
    <mirror>
      <id>aliyun</id>
      <mirrorOf>central</mirrorOf>
      <name>Aliyun Maven Central Mirror</name>
      <url>https://maven.aliyun.com/repository/central</url>
    </mirror>
  </mirrors>
</settings>
"#;
                if std::fs::write(&settings_path, settings).is_ok() {
                    crate::ui::print_success("已生成 Maven 配置 (~/.m2/settings.xml)，使用阿里云镜像");
                }
            } else {
                crate::ui::print_info("~/.m2/settings.xml 已存在，跳过");
            }
        }
        Ok(())
    }

    fn env_actions(&self, install_path: &PathBuf, _config: &HudoConfig) -> Vec<EnvAction> {
        vec![
            EnvAction::Set {
                name: "MAVEN_HOME".to_string(),
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
        .maven
        .as_deref()
        .unwrap_or("https://downloads.apache.org/maven/maven-3");
    let url = format!(
        "{}/{}/binaries/apache-maven-{}-bin.zip",
        base.trim_end_matches('/'),
        version,
        version
    );
    let filename = format!("apache-maven-{}-bin.zip", version);
    (url, filename)
}
