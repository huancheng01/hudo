use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

use crate::cc::{CcProvider, CcProviders};
use crate::config::HudoConfig;
use crate::installer::{DetectResult, InstallContext, Installer};
use crate::registry;

/// Profile 文件头信息
#[derive(Debug, Serialize, Deserialize)]
pub struct HudoMeta {
    pub version: String,
    pub exported_at: String,
}

/// Profile 中的 settings 段
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProfileSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub java_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub go_version: Option<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub mirrors: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub versions: BTreeMap<String, String>,
}

/// 完整的 Profile 数据结构
#[derive(Debug, Serialize, Deserialize)]
pub struct HudoProfile {
    pub hudo: HudoMeta,
    #[serde(default)]
    pub settings: ProfileSettings,
    #[serde(default)]
    pub tools: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub tool_config: BTreeMap<String, BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cc_providers: Vec<CcProvider>,
}

impl HudoProfile {
    /// 从当前系统状态构建 profile
    pub async fn build_from_current(
        config: &HudoConfig,
        installers: &[Box<dyn Installer>],
    ) -> Result<Self> {
        let ctx = InstallContext { config };
        let mut tools = BTreeMap::new();
        let mut tool_config = BTreeMap::new();

        for inst in installers {
            let info = inst.info();
            let detect = inst.detect_installed(&ctx).await;

            // 记录所有已安装工具（无论 hudo 还是系统安装）
            let version = match &detect {
                Ok(DetectResult::InstalledByHudo(ver)) => Some(ver.clone()),
                Ok(DetectResult::InstalledExternal(ver)) => Some(ver.clone()),
                _ => None,
            };

            if let Some(ver) = version {
                // 提取纯版本号（去掉 "git version " 等前缀）
                let short = extract_version(&ver);
                tools.insert(info.id.to_string(), short);

                // 收集工具配置
                let entries = inst.export_config(&ctx);
                if !entries.is_empty() {
                    let mut cfg_map = BTreeMap::new();
                    for (k, v) in entries {
                        cfg_map.insert(k, v);
                    }
                    tool_config.insert(info.id.to_string(), cfg_map);
                }
            }
        }

        // 收集 settings（遍历 KEYS：加新配置键时导出自动覆盖，不再手抄字段表）
        let mut mirrors = BTreeMap::new();
        for &key in crate::config::MirrorConfig::KEYS {
            if let Some(v) = config.mirrors.get(key) {
                mirrors.insert(key.to_string(), v.clone());
            }
        }

        let mut versions = BTreeMap::new();
        for &key in crate::config::VersionConfig::KEYS {
            if let Some(v) = config.versions.get(key) {
                versions.insert(key.to_string(), v.clone());
            }
        }

        let settings = ProfileSettings {
            java_version: Some(config.java.version.clone()),
            go_version: Some(config.go.version.clone()),
            mirrors,
            versions,
        };

        Ok(HudoProfile {
            hudo: HudoMeta {
                version: env!("CARGO_PKG_VERSION").to_string(),
                exported_at: registry::current_timestamp(),
            },
            settings,
            tools,
            tool_config,
            cc_providers: CcProviders::load().unwrap_or_default().providers,
        })
    }

    /// 保存 profile 到文件
    pub fn save_to_file(&self, path: &Path) -> Result<()> {
        let content = toml::to_string_pretty(self).context("序列化 profile 失败")?;
        let content = format!("# hudo profile\n{}", content);
        std::fs::write(path, content)
            .with_context(|| format!("无法写入 profile: {}", path.display()))?;
        Ok(())
    }

    /// 从文件加载 profile
    pub fn load_from_file(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("无法读取 profile: {}", path.display()))?;
        let profile: HudoProfile =
            toml::from_str(&content).with_context(|| format!("profile 格式错误: {}", path.display()))?;
        Ok(profile)
    }
}

/// 从版本字符串中提取纯版本号
fn extract_version(ver: &str) -> String {
    let trimmed = ver.trim();
    // 尝试找到以数字开头的 token（如 "git version 2.47.1" → "2.47.1"）
    trimmed
        .split_whitespace()
        .find(|s| s.starts_with(|c: char| c.is_ascii_digit()))
        .unwrap_or(trimmed)
        .to_string()
}
