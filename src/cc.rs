use anyhow::{Context, Result};
use dialoguer::{Password, Select, theme::ColorfulTheme};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::ui;

// ── Provider 配置 ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CcProvider {
    pub name: String,
    pub base_url: String,
    pub api_key: String,
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub reasoning_model: Option<String>,
    #[serde(default)]
    pub haiku_model: Option<String>,
    #[serde(default)]
    pub sonnet_model: Option<String>,
    #[serde(default)]
    pub opus_model: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CcProviders {
    #[serde(default)]
    pub providers: Vec<CcProvider>,
}

impl CcProviders {
    fn path() -> Result<PathBuf> {
        let home = dirs::home_dir().context("无法获取用户主目录")?;
        Ok(home.join(".hudo").join("cc-providers.toml"))
    }

    pub fn load() -> Result<Self> {
        let path = Self::path()?;
        if !path.exists() {
            return Ok(Self::default());
        }
        let s = std::fs::read_to_string(&path)
            .with_context(|| format!("读取 {} 失败", path.display()))?;
        toml::from_str(&s).with_context(|| format!("解析 {} 失败", path.display()))
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::path()?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let s = toml::to_string_pretty(self).context("序列化 providers 失败")?;
        std::fs::write(&path, s)
            .with_context(|| format!("写入 {} 失败", path.display()))
    }
}

// ── Claude settings.json ──────────────────────────────────────────────────────

fn claude_settings_path() -> Result<PathBuf> {
    let home = dirs::home_dir().context("无法获取用户主目录")?;
    // claude 使用 ~/.claude/settings.json
    Ok(home.join(".claude").join("settings.json"))
}

/// 读取 ~/.claude/settings.json，不存在则返回空对象
fn read_settings() -> Result<serde_json::Value> {
    let path = claude_settings_path()?;
    if !path.exists() {
        return Ok(serde_json::json!({}));
    }
    let s = std::fs::read_to_string(&path)
        .with_context(|| format!("读取 {} 失败", path.display()))?;
    serde_json::from_str(&s).with_context(|| format!("解析 {} 失败", path.display()))
}

/// 将修改后的 settings 写回
fn write_settings(val: &serde_json::Value) -> Result<()> {
    let path = claude_settings_path()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let s = serde_json::to_string_pretty(val).context("序列化 settings.json 失败")?;
    std::fs::write(&path, s)
        .with_context(|| format!("写入 {} 失败", path.display()))
}

/// 将 provider 写入 claude settings.json，并确保 onboarding 已标记完成
fn apply_provider(p: &CcProvider) -> Result<()> {
    let mut settings = read_settings()?;

    // 确保 env 对象存在
    if settings.get("env").is_none() {
        settings["env"] = serde_json::json!({});
    }

    settings["env"]["ANTHROPIC_AUTH_TOKEN"] = serde_json::Value::String(p.api_key.clone());
    settings["env"]["ANTHROPIC_BASE_URL"] = serde_json::Value::String(p.base_url.clone());

    // 写入模型配置（有值则设置，无值则清除）
    let model_fields: &[(&str, &Option<String>)] = &[
        ("ANTHROPIC_MODEL", &p.model),
        ("ANTHROPIC_REASONING_MODEL", &p.reasoning_model),
        ("ANTHROPIC_DEFAULT_HAIKU_MODEL", &p.haiku_model),
        ("ANTHROPIC_DEFAULT_SONNET_MODEL", &p.sonnet_model),
        ("ANTHROPIC_DEFAULT_OPUS_MODEL", &p.opus_model),
    ];
    for (key, val) in model_fields {
        match val {
            Some(v) => {
                settings["env"][key] = serde_json::Value::String(v.clone());
            }
            None => {
                if let Some(env) = settings["env"].as_object_mut() {
                    env.remove(*key);
                }
            }
        }
    }

    write_settings(&settings)?;

    // 使用第三方 API 时，需要在 ~/.claude.json 中标记 onboarding 已完成
    // 否则 Claude Code 会卡在引导流程
    ensure_onboarding_completed()
}

/// 确保 ~/.claude.json 中 hasCompletedOnboarding = true
fn ensure_onboarding_completed() -> Result<()> {
    let home = dirs::home_dir().context("无法获取用户主目录")?;
    let path = home.join(".claude.json");

    let mut val = if path.exists() {
        let s = std::fs::read_to_string(&path)
            .with_context(|| format!("读取 {} 失败", path.display()))?;
        serde_json::from_str(&s)
            .with_context(|| format!("解析 {} 失败", path.display()))?
    } else {
        serde_json::json!({})
    };

    if val.get("hasCompletedOnboarding") == Some(&serde_json::Value::Bool(true)) {
        return Ok(());
    }

    val["hasCompletedOnboarding"] = serde_json::Value::Bool(true);

    let s = serde_json::to_string_pretty(&val).context("序列化 .claude.json 失败")?;
    std::fs::write(&path, s)
        .with_context(|| format!("写入 {} 失败", path.display()))?;

    Ok(())
}

/// 从 claude settings.json 读取当前激活的 base_url
fn current_base_url() -> Option<String> {
    read_settings().ok().and_then(|s| {
        s["env"]["ANTHROPIC_BASE_URL"]
            .as_str()
            .map(|v| v.to_string())
    })
}

/// 清除 settings.json 中所有 hudo 写入的 env 变量，恢复官方默认
fn reset_to_default() -> Result<()> {
    let mut settings = read_settings()?;
    if let Some(env) = settings.get_mut("env").and_then(|e| e.as_object_mut()) {
        let keys_to_remove = [
            "ANTHROPIC_AUTH_TOKEN",
            "ANTHROPIC_BASE_URL",
            "ANTHROPIC_MODEL",
            "ANTHROPIC_REASONING_MODEL",
            "ANTHROPIC_DEFAULT_HAIKU_MODEL",
            "ANTHROPIC_DEFAULT_SONNET_MODEL",
            "ANTHROPIC_DEFAULT_OPUS_MODEL",
        ];
        for key in &keys_to_remove {
            env.remove(*key);
        }
        // 如果 env 为空则整个移除
        if env.is_empty() {
            if let Some(obj) = settings.as_object_mut() {
                obj.remove("env");
            }
        }
    }
    write_settings(&settings)
}

// ── 交互菜单 ──────────────────────────────────────────────────────────────────

pub fn cmd_cc() -> Result<()> {
    loop {
        ui::page_header("Claude Code API 来源管理");

        let mut store = CcProviders::load()?;
        let active_url = current_base_url();

        if store.providers.is_empty() {
            println!("  {}", console::style("暂无 Provider，请先添加").dim());
            println!();
            let items = ["[+] 添加 Provider", "[B] 返回"];
            let sel = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("请选择 (Esc 返回)")
                .items(&items)
                .default(0)
                .interact_opt()
                .unwrap_or(None);
            match sel {
                Some(0) => {
                    if add_provider(&mut store)? {
                        store.save()?;
                        offer_activate_last(&store)?;
                    }
                }
                _ => break,
            }
            continue;
        }

        // 构建列表项：当前激活的前面显示 *（名称按显示宽度对齐，兼容中文）
        let items: Vec<String> = store
            .providers
            .iter()
            .map(|p| {
                let active = active_url.as_deref() == Some(p.base_url.as_str());
                let mark = if active {
                    console::style("* ").green().to_string()
                } else {
                    "  ".to_string()
                };
                format!("{}{}  {}", mark, ui::pad(&p.name, 20), console::style(&p.base_url).dim())
            })
            .chain(std::iter::once("  [+] 添加 Provider".to_string()))
            .chain(std::iter::once("  [R] 恢复默认（清除自定义配置）".to_string()))
            .chain(std::iter::once("  [B] 返回".to_string()))
            .collect();

        let n = store.providers.len();
        let sel = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("选择 Provider 查看详情（* = 当前激活，Esc 返回）")
            .items(&items)
            .default(0)
            .interact_opt()
            .unwrap_or(None);

        match sel {
            None => break,
            Some(i) if i < n => {
                provider_menu(&mut store, i)?;
            }
            Some(i) if i == n => {
                if add_provider(&mut store)? {
                    store.save()?;
                    offer_activate_last(&store)?;
                }
            }
            Some(i) if i == n + 1 => {
                if ui::confirm("确认清除所有自定义 API 配置，恢复官方默认？", false)? {
                    reset_to_default()?;
                    ui::print_success("已恢复默认，自定义 API 配置已清除");
                    ui::print_next_step("重启终端或 Claude Code 后生效");
                    ui::wait_for_key();
                }
            }
            _ => break,
        }
    }

    Ok(())
}

/// 单个 Provider 的详情页与操作子菜单（查看 / 切换 / 编辑 / 删除）
fn provider_menu(store: &mut CcProviders, idx: usize) -> Result<()> {
    loop {
        let active_url = current_base_url();
        let p = &store.providers[idx];
        let active = active_url.as_deref() == Some(p.base_url.as_str());

        ui::page_header(&format!("Provider — {}", p.name));
        print_provider_detail(p, active);

        let items = ["[>] 切换到此来源", "[E] 编辑", "[X] 删除", "[B] 返回"];
        let sel = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("请选择操作 (Esc 返回)")
            .items(&items)
            .default(0)
            .interact_opt()
            .unwrap_or(None);

        match sel {
            Some(0) => {
                let p = &store.providers[idx];
                apply_provider(p)?;
                ui::print_success(&format!("已切换到 [{}]  {}", p.name, p.base_url));
                ui::print_next_step("重启终端或 Claude Code 后生效");
                ui::wait_for_key();
                return Ok(());
            }
            Some(1) => {
                if edit_provider(&mut store.providers[idx])? {
                    store.save()?;
                    ui::print_success("已保存修改");
                    // 编辑的是当前激活的来源时，需重新应用才会写入 Claude 配置
                    if active && ui::confirm("该来源当前已激活，是否立即应用修改后的配置？", true)? {
                        apply_provider(&store.providers[idx])?;
                        ui::print_next_step("重启终端或 Claude Code 后生效");
                    }
                }
                ui::wait_for_key();
            }
            Some(2) => {
                let name = store.providers[idx].name.clone();
                if !ui::confirm(&format!("确认删除 [{}]？", name), false)? {
                    continue;
                }
                if active {
                    ui::print_warning("该来源当前处于激活状态，仅删除记录的话 Claude Code 仍会使用旧配置");
                    if ui::confirm("是否同时恢复官方默认配置？", true)? {
                        reset_to_default()?;
                        ui::print_info("已恢复官方默认配置");
                    }
                }
                store.providers.remove(idx);
                store.save()?;
                ui::print_success(&format!("已删除 [{}]", name));
                ui::wait_for_key();
                return Ok(());
            }
            Some(3) | None => return Ok(()),
            _ => return Ok(()),
        }
    }
}

/// 打印 Provider 详情面板（API Key 脱敏显示）
fn print_provider_detail(p: &CcProvider, active: bool) {
    let mut lines = Vec::new();
    lines.push(format!(" {}  {}", ui::pad("名称", 12), p.name));
    lines.push(format!(" {}  {}", ui::pad("Base URL", 12), p.base_url));
    lines.push(format!(" {}  {}", ui::pad("API Key", 12), mask_key(&p.api_key)));
    let models: &[(&str, &Option<String>)] = &[
        ("默认模型", &p.model),
        ("推理模型", &p.reasoning_model),
        ("Haiku 模型", &p.haiku_model),
        ("Sonnet 模型", &p.sonnet_model),
        ("Opus 模型", &p.opus_model),
    ];
    for (label, val) in models {
        if let Some(v) = val {
            lines.push(format!(" {}  {}", ui::pad(label, 12), v));
        }
    }
    lines.push(String::new());
    let status = if active {
        console::style("已激活（当前来源）").green().to_string()
    } else {
        console::style("未激活").dim().to_string()
    };
    lines.push(format!(" {}  {}", ui::pad("状态", 12), status));
    ui::print_box(&lines);
    println!();
}

/// API Key 脱敏：保留前 8 位与后 4 位
fn mask_key(key: &str) -> String {
    let chars: Vec<char> = key.chars().collect();
    if chars.len() <= 12 {
        "****".to_string()
    } else {
        let head: String = chars[..8].iter().collect();
        let tail: String = chars[chars.len() - 4..].iter().collect();
        format!("{}****{}", head, tail)
    }
}

/// 交互式添加 Provider，返回是否添加成功（必填项留空回车即取消）
fn add_provider(store: &mut CcProviders) -> Result<bool> {
    println!();
    ui::print_info("任意必填项留空回车即可取消");

    let name = ui::input_text("名称（如: 官方 / 中转）", None, true)?;
    if name.is_empty() {
        ui::print_info("已取消");
        return Ok(false);
    }
    if store.providers.iter().any(|p| p.name == name) {
        ui::print_warning(&format!("已存在同名 Provider [{}]，请换个名称或先删除旧的", name));
        ui::wait_for_key();
        return Ok(false);
    }

    let base_url = match input_base_url(None)? {
        Some(url) => url,
        None => {
            ui::print_info("已取消");
            return Ok(false);
        }
    };

    let api_key = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("API Key（输入不回显，留空取消）")
        .allow_empty_password(true)
        .interact()?
        .trim()
        .to_string();
    if api_key.is_empty() {
        ui::print_info("已取消");
        return Ok(false);
    }

    // 可选：配置自定义模型
    let (model, reasoning_model, haiku_model, sonnet_model, opus_model) =
        if ui::confirm("是否配置自定义模型？（第三方 API 通常需要）", false)? {
            (
                ask_model("默认模型 (ANTHROPIC_MODEL，回车跳过)")?,
                ask_model("推理模型 (ANTHROPIC_REASONING_MODEL，回车跳过)")?,
                ask_model("Haiku 模型 (ANTHROPIC_DEFAULT_HAIKU_MODEL，回车跳过)")?,
                ask_model("Sonnet 模型 (ANTHROPIC_DEFAULT_SONNET_MODEL，回车跳过)")?,
                ask_model("Opus 模型 (ANTHROPIC_DEFAULT_OPUS_MODEL，回车跳过)")?,
            )
        } else {
            (None, None, None, None, None)
        };

    store.providers.push(CcProvider {
        name,
        base_url,
        api_key,
        model,
        reasoning_model,
        haiku_model,
        sonnet_model,
        opus_model,
    });

    ui::print_success("Provider 已添加");
    Ok(true)
}

/// 输入并校验 Base URL（必须 http/https 开头，去除末尾斜杠）；空输入返回 None
fn input_base_url(current: Option<&str>) -> Result<Option<String>> {
    loop {
        let url = ui::input_text("Base URL（如: https://api.anthropic.com）", current, true)?;
        if url.is_empty() {
            return Ok(None);
        }
        if url.starts_with("http://") || url.starts_with("https://") {
            return Ok(Some(url.trim_end_matches('/').to_string()));
        }
        ui::print_warning("Base URL 需以 http:// 或 https:// 开头");
    }
}

/// 询问可选的模型名，空输入返回 None
fn ask_model(prompt: &str) -> Result<Option<String>> {
    let v = ui::input_text(prompt, None, true)?;
    Ok(if v.is_empty() { None } else { Some(v) })
}

/// 编辑用：回车保留原值，输入 "-" 清除
fn edit_model(label: &str, current: &Option<String>) -> Result<Option<String>> {
    let prompt = format!("{}（回车保留，输入 - 清除）", label);
    let v = ui::input_text(&prompt, current.as_deref(), true)?;
    Ok(match v.as_str() {
        "-" => None,
        "" => current.clone(),
        _ => Some(v),
    })
}

/// 逐字段编辑 Provider（回车保留原值），返回是否有修改
fn edit_provider(p: &mut CcProvider) -> Result<bool> {
    println!();
    ui::print_info("回车保留原值；模型字段输入 - 可清除");

    let name = ui::input_text("名称", Some(&p.name), false)?;
    let base_url = match input_base_url(Some(&p.base_url))? {
        Some(url) => url,
        None => p.base_url.clone(),
    };

    let key_input = Password::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("API Key（当前 {}，留空保留）", mask_key(&p.api_key)))
        .allow_empty_password(true)
        .interact()?
        .trim()
        .to_string();

    let model = edit_model("默认模型", &p.model)?;
    let reasoning_model = edit_model("推理模型", &p.reasoning_model)?;
    let haiku_model = edit_model("Haiku 模型", &p.haiku_model)?;
    let sonnet_model = edit_model("Sonnet 模型", &p.sonnet_model)?;
    let opus_model = edit_model("Opus 模型", &p.opus_model)?;

    let mut changed = false;
    if name != p.name {
        p.name = name;
        changed = true;
    }
    if base_url != p.base_url {
        p.base_url = base_url;
        changed = true;
    }
    if !key_input.is_empty() && key_input != p.api_key {
        p.api_key = key_input;
        changed = true;
    }
    if model != p.model {
        p.model = model;
        changed = true;
    }
    if reasoning_model != p.reasoning_model {
        p.reasoning_model = reasoning_model;
        changed = true;
    }
    if haiku_model != p.haiku_model {
        p.haiku_model = haiku_model;
        changed = true;
    }
    if sonnet_model != p.sonnet_model {
        p.sonnet_model = sonnet_model;
        changed = true;
    }
    if opus_model != p.opus_model {
        p.opus_model = opus_model;
        changed = true;
    }

    if !changed {
        ui::print_info("未做修改");
    }
    Ok(changed)
}

/// 添加成功后询问是否立即切换到新 Provider
fn offer_activate_last(store: &CcProviders) -> Result<()> {
    if let Some(p) = store.providers.last() {
        if ui::confirm(&format!("是否立即切换到 [{}]？", p.name), true)? {
            apply_provider(p)?;
            ui::print_success(&format!("已切换到 [{}]  {}", p.name, p.base_url));
            ui::print_next_step("重启终端或 Claude Code 后生效");
        } else {
            ui::print_info("已添加但未激活，可稍后在列表中选择启用");
        }
        ui::wait_for_key();
    }
    Ok(())
}
