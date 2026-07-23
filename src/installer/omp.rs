use anyhow::{Context, Result};
use async_trait::async_trait;
use std::path::{Path, PathBuf};

use super::{DetectResult, EnvAction, InstallContext, InstallResult, Installer, ToolInfo};
use crate::config::HudoConfig;
use crate::download;

pub struct OmpInstaller;

const PROFILE_MARKER: &str = "# oh-my-posh (hudo)";
const INIT_LINE: &str =
    "oh-my-posh init pwsh --config \"$env:POSH_THEMES_PATH\\jandedobbeleer.omp.json\" | Invoke-Expression";

#[async_trait]
impl Installer for OmpInstaller {
    fn info(&self) -> ToolInfo {
        ToolInfo {
            id: "omp",
            name: "Oh My Posh",
            description: "终端主题引擎（捆绑 Nerd Font 字体）",
        }
    }

    async fn detect_installed(&self, ctx: &InstallContext<'_>) -> Result<DetectResult> {
        let omp_exe = ctx.config.tools_dir().join("oh-my-posh").join("oh-my-posh.exe");
        if omp_exe.exists() {
            if let Some(v) = omp_version(&omp_exe) {
                return Ok(DetectResult::InstalledByHudo(v));
            }
            return Ok(DetectResult::InstalledByHudo("已安装".to_string()));
        }

        if let Ok(out) = std::process::Command::new("oh-my-posh").arg("--version").output() {
            if out.status.success() {
                let v = String::from_utf8_lossy(&out.stdout).trim().to_string();
                return Ok(DetectResult::InstalledExternal(v));
            }
        }

        Ok(DetectResult::NotInstalled)
    }

    fn resolve_download(&self, config: &HudoConfig) -> (String, String) {
        match config.versions.omp.as_deref() {
            Some(v) => (
                format!(
                    "https://github.com/JanDeDobbeleer/oh-my-posh/releases/download/v{}/posh-windows-amd64.exe",
                    v
                ),
                format!("posh-windows-amd64-{}.exe", v),
            ),
            None => (
                "https://github.com/JanDeDobbeleer/oh-my-posh/releases/latest/download/posh-windows-amd64.exe"
                    .to_string(),
                "posh-windows-amd64.exe".to_string(),
            ),
        }
    }

    async fn install(&self, ctx: &InstallContext<'_>) -> Result<InstallResult> {
        let config = ctx.config;
        let install_dir = config.tools_dir().join("oh-my-posh");
        let (url, filename) = self.resolve_download(config);

        // 未锁定版本时文件名不含版本号，清缓存避免永远装旧版
        if config.versions.omp.is_none() {
            let cached = config.cache_dir().join(&filename);
            if cached.exists() {
                std::fs::remove_file(&cached).ok();
            }
        }

        // 1. 主程序（单 exe）
        let exe_path = download::download(&url, &config.cache_dir(), &filename).await?;
        std::fs::create_dir_all(&install_dir)
            .with_context(|| format!("无法创建目录: {}", install_dir.display()))?;
        std::fs::copy(&exe_path, install_dir.join("oh-my-posh.exe"))
            .context("复制 oh-my-posh.exe 失败")?;

        // 2. 官方主题包
        let themes_url = match config.versions.omp.as_deref() {
            Some(v) => format!(
                "https://github.com/JanDeDobbeleer/oh-my-posh/releases/download/v{}/themes.zip",
                v
            ),
            None => "https://github.com/JanDeDobbeleer/oh-my-posh/releases/latest/download/themes.zip"
                .to_string(),
        };
        let themes_cache = "omp-themes.zip";
        let cached = config.cache_dir().join(themes_cache);
        if cached.exists() {
            std::fs::remove_file(&cached).ok();
        }
        let themes_zip = download::download(&themes_url, &config.cache_dir(), themes_cache).await?;
        let themes_dir = install_dir.join("themes");
        if themes_dir.exists() {
            std::fs::remove_dir_all(&themes_dir).ok();
        }
        crate::ui::print_action("解压主题包...");
        download::extract_zip(&themes_zip, &themes_dir)?;

        // 3. Nerd Font（oh-my-posh 官方硬要求，否则图标全是乱码方块）
        install_nerd_font(config).await?;

        let version = omp_version(&install_dir.join("oh-my-posh.exe"))
            .unwrap_or_else(|| "unknown".to_string());

        Ok(InstallResult {
            install_path: install_dir,
            version,
        })
    }

    fn env_actions(&self, install_path: &PathBuf, _config: &HudoConfig) -> Vec<EnvAction> {
        vec![
            EnvAction::AppendPath {
                path: install_path.to_string_lossy().to_string(),
            },
            EnvAction::Set {
                name: "POSH_THEMES_PATH".to_string(),
                value: install_path.join("themes").to_string_lossy().to_string(),
            },
        ]
    }

    async fn configure(&self, _ctx: &InstallContext<'_>) -> Result<()> {
        if crate::ui::confirm("是否写入 PowerShell profile 启用 oh-my-posh 提示符？", true)? {
            let mut written = false;
            for profile in profile_paths() {
                match append_init_to_profile(&profile) {
                    Ok(true) => {
                        crate::ui::print_success(&format!("已写入 {}", profile.display()));
                        written = true;
                    }
                    Ok(false) => {
                        crate::ui::print_info(&format!("{} 已配置过，跳过", profile.display()));
                        written = true;
                    }
                    Err(e) => crate::ui::print_warning(&format!(
                        "写入 {} 失败: {}",
                        profile.display(),
                        e
                    )),
                }
            }
            if !written {
                crate::ui::print_warning("未找到 PowerShell profile 路径，可手动配置");
                crate::ui::print_next_step(INIT_LINE);
            }
        } else {
            crate::ui::print_info("可稍后在 PowerShell profile 中手动添加:");
            crate::ui::print_next_step(INIT_LINE);
        }
        crate::ui::print_next_step(
            "请在终端设置中将字体切换为 CaskaydiaCove Nerd Font，图标才能正常显示",
        );
        Ok(())
    }

    async fn pre_uninstall(&self, _ctx: &InstallContext<'_>) -> Result<()> {
        // 逆向清理 profile 注入与用户级字体注册（POSH_THEMES_PATH/PATH 由通用卸载流程处理）
        for profile in profile_paths() {
            if remove_init_from_profile(&profile).unwrap_or(false) {
                crate::ui::print_info(&format!("已清理 {}", profile.display()));
            }
        }
        uninstall_nerd_font();
        Ok(())
    }
}

fn omp_version(exe: &Path) -> Option<String> {
    let out = std::process::Command::new(exe).arg("--version").output().ok()?;
    if !out.status.success() {
        return None;
    }
    let v = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if v.is_empty() {
        None
    } else {
        Some(v)
    }
}

/// Windows PowerShell 5.1 与 pwsh 7 的 $PROFILE 路径（存在的 shell 才返回，去重）
fn profile_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for shell in ["powershell", "pwsh"] {
        let Ok(out) = std::process::Command::new(shell)
            .args(["-NoProfile", "-Command", "$PROFILE"])
            .output()
        else {
            continue;
        };
        if !out.status.success() {
            continue;
        }
        let p = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if p.is_empty() {
            continue;
        }
        let p = PathBuf::from(p);
        if !paths.contains(&p) {
            paths.push(p);
        }
    }
    paths
}

/// 幂等追加 init 行；返回是否实际写入（已配置过返回 false）
fn append_init_to_profile(profile: &Path) -> Result<bool> {
    let content = if profile.exists() {
        std::fs::read_to_string(profile).unwrap_or_default()
    } else {
        if let Some(parent) = profile.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        String::new()
    };
    if content.contains(PROFILE_MARKER) || content.contains("oh-my-posh init") {
        return Ok(false);
    }
    let line_sep = if content.contains("\r\n") || content.is_empty() { "\r\n" } else { "\n" };
    let mut new_content = content;
    if !new_content.is_empty() && !new_content.ends_with('\n') {
        new_content.push_str(line_sep);
    }
    new_content.push_str(&format!("{}{}{}{}", PROFILE_MARKER, line_sep, INIT_LINE, line_sep));
    std::fs::write(profile, new_content)
        .with_context(|| format!("写入 profile 失败: {}", profile.display()))?;
    Ok(true)
}

/// 移除带 marker 的注入行；返回是否实际修改
fn remove_init_from_profile(profile: &Path) -> Result<bool> {
    if !profile.exists() {
        return Ok(false);
    }
    let content = std::fs::read_to_string(profile).unwrap_or_default();
    let line_sep = if content.contains("\r\n") { "\r\n" } else { "\n" };
    let kept: Vec<&str> = content
        .split(line_sep)
        .filter(|l| {
            let t = l.trim();
            t != PROFILE_MARKER && t != INIT_LINE
        })
        .collect();
    let new_content = kept.join(line_sep);
    if new_content == content {
        return Ok(false);
    }
    std::fs::write(profile, new_content)
        .with_context(|| format!("写入 profile 失败: {}", profile.display()))?;
    Ok(true)
}

/// 用户级安装 CaskaydiaCove Nerd Font：
/// 复制 ttf 到 %LOCALAPPDATA%\Microsoft\Windows\Fonts + 写 HKCU Fonts 注册表（免管理员，Win10 1809+）
async fn install_nerd_font(config: &HudoConfig) -> Result<()> {
    let font_zip = download::download(
        "https://github.com/ryanoasis/nerd-fonts/releases/latest/download/CascadiaCode.zip",
        &config.cache_dir(),
        "nerd-CascadiaCode.zip",
    )
    .await?;

    let tmp_dir = config.cache_dir().join("nerdfont-extract");
    if tmp_dir.exists() {
        std::fs::remove_dir_all(&tmp_dir).ok();
    }
    crate::ui::print_action("安装 CaskaydiaCove Nerd Font（用户级）...");
    download::extract_zip(&font_zip, &tmp_dir)?;

    let fonts_dir = user_fonts_dir().context("无法定位用户字体目录")?;
    std::fs::create_dir_all(&fonts_dir).ok();

    use winreg::enums::*;
    use winreg::RegKey;
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (font_key, _) = hkcu
        .create_subkey("Software\\Microsoft\\Windows NT\\CurrentVersion\\Fonts")
        .context("打开用户字体注册表失败")?;

    let mut installed = 0u32;
    for entry in std::fs::read_dir(&tmp_dir)?.flatten() {
        let path = entry.path();
        let is_ttf = path
            .extension()
            .map(|e| e.eq_ignore_ascii_case("ttf"))
            .unwrap_or(false);
        if !is_ttf {
            continue;
        }
        let file_name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
        let dest = fonts_dir.join(&file_name);
        std::fs::copy(&path, &dest)
            .with_context(|| format!("复制字体失败: {}", file_name))?;
        let stem = path.file_stem().unwrap_or_default().to_string_lossy().to_string();
        font_key
            .set_value(
                format!("{} (TrueType)", stem),
                &dest.to_string_lossy().to_string(),
            )
            .with_context(|| format!("注册字体失败: {}", file_name))?;
        installed += 1;
    }
    std::fs::remove_dir_all(&tmp_dir).ok();

    if installed == 0 {
        anyhow::bail!("字体包中未找到 ttf 文件");
    }
    crate::ui::print_success(&format!("已安装 {} 个字体文件（新终端生效）", installed));
    Ok(())
}

/// 卸载 hudo 注册的 CaskaydiaCove 字体：删注册表值 + 删字体文件
fn uninstall_nerd_font() {
    use winreg::enums::*;
    use winreg::RegKey;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let Ok(font_key) =
        hkcu.open_subkey_with_flags("Software\\Microsoft\\Windows NT\\CurrentVersion\\Fonts", KEY_ALL_ACCESS)
    else {
        return;
    };

    use winreg::types::FromRegValue;
    let values: Vec<(String, String)> = font_key
        .enum_values()
        .flatten()
        .filter_map(|(name, val)| {
            let data = String::from_reg_value(&val).ok()?;
            Some((name, data))
        })
        .collect();

    let mut removed = 0u32;
    for (name, data) in values {
        if name.starts_with("CaskaydiaCove") {
            std::fs::remove_file(&data).ok();
            if font_key.delete_value(&name).is_ok() {
                removed += 1;
            }
        }
    }
    if removed > 0 {
        crate::ui::print_info(&format!("已移除 {} 个字体注册（文件同步删除）", removed));
    }
}

fn user_fonts_dir() -> Option<PathBuf> {
    let local = std::env::var("LOCALAPPDATA").ok()?;
    Some(PathBuf::from(local).join("Microsoft").join("Windows").join("Fonts"))
}
