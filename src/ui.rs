use std::io::Write;
use anyhow::Result;
use console::{measure_text_width, pad_str, style, Alignment, Style};
use dialoguer::{theme::ColorfulTheme, Confirm, Input};

/// 硬编码 ASCII art（FIGlet "Small Slant" 风格，宽度 < 50 列）
const BANNER_LINES: &[&str] = &[
    r"    __              __",
    r"   / /_  __  ______/ /___",
    r"  / __ \/ / / / __  / __ \",
    r" / / / / /_/ / /_/ / /_/ /",
    r"/_/ /_/\__,_/\__,_/\____/",
];

/// 渐变色序列（蓝 → 青 → 紫）
const GRADIENT_COLORS: &[(u8, u8, u8)] = &[
    (59, 130, 246),  // blue-500
    (56, 152, 236),
    (99, 102, 241),  // indigo-500
    (124, 92, 239),
    (139, 92, 246),  // violet-500
];

/// 打印 hudo 品牌 Banner（硬编码 ASCII art + 逐行渐变）
/// 终端不支持颜色（重定向 / CLICOLOR=0 / 旧终端）时输出纯文本，避免裸转义乱码
pub fn print_banner() {
    let stdout = std::io::stdout();
    let mut w = std::io::BufWriter::new(stdout.lock());
    let colors = console::colors_enabled();
    for (i, line) in BANNER_LINES.iter().enumerate() {
        if colors {
            let (r, g, b) = GRADIENT_COLORS[i % GRADIENT_COLORS.len()];
            let _ = writeln!(w, "\x1b[1m\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, line);
        } else {
            let _ = writeln!(w, "{}", line);
        }
    }
    let _ = writeln!(
        w,
        "  {} {}",
        style(format!("v{}", env!("CARGO_PKG_VERSION"))).dim(),
        style("混沌 — 开发环境一键引导工具").dim()
    );
    let _ = writeln!(w);
}

/// 清屏（经 console 的终端能力检测：ANSI 不可用时走 WinAPI，保留滚动缓冲区）
pub fn clear_screen() {
    let _ = console::Term::stdout().clear_screen();
}

/// 打印标题行（单行分隔线，避免悬空的半开边框）
pub fn print_title(text: &str) {
    let text_width = measure_text_width(text);
    let fill = text_width.max(38) + 2 - text_width;
    let s = Style::new().cyan();
    println!();
    println!("{}", s.apply_to(format!("── {} {}", text, "─".repeat(fill))));
}

/// 打印分类标题（用于 list / setup 中的分组）
pub fn print_section(text: &str) {
    println!();
    println!("  {} {}", style("■").cyan(), style(text).bold());
}

/// 打印进度步骤
pub fn print_step(step: u32, total: u32, text: &str) {
    println!(
        "  {} {}",
        style(format!("[{}/{}]", step, total)).cyan().bold(),
        style(text).bold()
    );
}

pub fn print_success(text: &str) {
    println!("  {} {}", style("✓").green().bold(), text);
}

pub fn print_warning(text: &str) {
    println!("  {} {}", style("⚠").yellow().bold(), text);
}

#[allow(dead_code)]
pub fn print_error(text: &str) {
    println!("  {} {}", style("✗").red().bold(), text);
}

pub fn print_info(text: &str) {
    println!("  {}", style(text).dim());
}

/// 打印需要用户执行的关键下一步操作（醒目，与 dim 的过程信息区分）
pub fn print_next_step(text: &str) {
    println!("  {} {}", style(">").cyan().bold(), style(text).cyan());
}

/// 打印正在进行的操作（树形连接符）
pub fn print_action(text: &str) {
    println!("  {} {}", style("├─").dim(), text);
}

/// 打印最后一步操作（树形结束符）
#[allow(dead_code)]
pub fn print_action_last(text: &str) {
    println!("  {} {}", style("└─").dim(), text);
}

/// 打印 boxed 面板
pub fn print_box(lines: &[String]) {
    let max_width = lines
        .iter()
        .map(|l| measure_text_width(l))
        .max()
        .unwrap_or(0)
        .max(36);
    let inner_width = max_width + 2;

    let s = Style::new().cyan();
    println!();
    println!("{}", s.apply_to(format!("  ╭{}╮", "─".repeat(inner_width))));
    for line in lines {
        let text_width = measure_text_width(line);
        let padding = inner_width - text_width - 1;
        println!(
            "  {} {}{}{}",
            s.apply_to("│"),
            line,
            " ".repeat(padding),
            s.apply_to("│")
        );
    }
    println!("{}", s.apply_to(format!("  ╰{}╯", "─".repeat(inner_width))));
}

/// 用点线连接名称和版本（list 视图）
pub fn dotfill(name: &str, version: &str, total_width: usize) -> String {
    let name_w = measure_text_width(name);
    let ver_w = measure_text_width(version);
    let dots_needed = if name_w + ver_w + 2 < total_width {
        total_width - name_w - ver_w - 2
    } else {
        2
    };
    format!(
        "{} {} {}",
        style(name).bold(),
        style("·".repeat(dots_needed)).dim(),
        version
    )
}

/// 将文本填充到指定显示宽度（处理中文双宽字符）
pub fn pad(text: &str, width: usize) -> String {
    pad_str(text, width, Alignment::Left, None).to_string()
}

/// 工具分类
pub enum ToolCategory {
    Tool,
    Language,
    Database,
    Ide,
}

impl ToolCategory {
    pub fn label(&self) -> &'static str {
        match self {
            ToolCategory::Tool => "工具",
            ToolCategory::Language => "语言环境",
            ToolCategory::Database => "数据库",
            ToolCategory::Ide => "编辑器 / IDE",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            ToolCategory::Tool => "[T]",
            ToolCategory::Language => "[L]",
            ToolCategory::Database => "[D]",
            ToolCategory::Ide => "[E]",
        }
    }

    pub fn from_id(id: &str) -> Self {
        match id {
            "git" | "gh" | "claude-code" | "7zip" | "pwsh" | "powertoys" | "omp" => ToolCategory::Tool,
            "uv" | "nodejs" | "fnm" | "bun" | "miniconda" | "rust" | "go" | "jdk" | "c" | "maven" | "gradle" | "dotnet" => ToolCategory::Language,
            "mysql" | "pgsql" | "redis" => ToolCategory::Database,
            "vscode" | "pycharm" | "idea" | "chrome" => ToolCategory::Ide,
            _ => ToolCategory::Tool,
        }
    }
}

/// 页面头部：清屏 + Banner + 标题
/// 所有页面统一用这个：头部高度一致，切换页面时内容不跳行
pub fn page_header(title: &str) {
    clear_screen();
    print_banner();
    print_title(title);
}

/// 暂停等待用户按键（提示与读键走同一个终端，避免 stdout 重定向时提示丢失）
pub fn wait_for_key() {
    if assume_yes() {
        return;
    }
    let term = console::Term::stderr();
    let _ = term.write_line("");
    let _ = term.write_line(&format!("  {}", style("按任意键继续...").dim()));
    let _ = term.read_key();
}

use std::sync::atomic::{AtomicBool, Ordering};

/// 非交互模式开关（-y/--yes），进程启动时注入一次
static ASSUME_YES: AtomicBool = AtomicBool::new(false);

pub fn set_assume_yes(on: bool) {
    ASSUME_YES.store(on, Ordering::Relaxed);
}

pub fn assume_yes() -> bool {
    ASSUME_YES.load(Ordering::Relaxed)
}

/// 非交互模式下回显自动决策，保证脚本日志可审计
fn print_auto_answer(prompt: &str, answer: bool) {
    println!(
        "  {} {} {}",
        style("?").cyan(),
        prompt,
        style(if answer { "[自动: 是]" } else { "[自动: 否]" }).dim()
    );
}

/// 统一主题的确认框（Esc 视为取消，返回 false）
/// 用于可选分支（接管/重跑配置/顺带安装等）：非交互模式下取默认值，不会自动扩大动作范围
pub fn confirm(prompt: &str, default: bool) -> Result<bool> {
    if assume_yes() {
        print_auto_answer(prompt, default);
        return Ok(default);
    }
    let ans = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(default)
        .interact_opt()?;
    Ok(ans.unwrap_or(false))
}

/// 推进类确认（确认开始/确认卸载/确认导入等，"否"意味着整个命令中止）：
/// 非交互模式下自动"是"——用户敲下命令本身就是意图，交互默认值只服务于交互场景
pub fn confirm_proceed(prompt: &str, default: bool) -> Result<bool> {
    if assume_yes() {
        print_auto_answer(prompt, true);
        return Ok(true);
    }
    confirm(prompt, default)
}

/// 统一主题的文本输入（返回值已去除首尾空白）
/// allow_empty 为 true 时允许空输入，调用方可将空值视为取消
/// 非交互模式下直接采用默认值；无默认值且不允许空时报错，不静默编造输入
pub fn input_text(prompt: &str, default: Option<&str>, allow_empty: bool) -> Result<String> {
    if assume_yes() {
        let v = default.unwrap_or("").trim().to_string();
        if v.is_empty() && !allow_empty {
            anyhow::bail!("非交互模式下「{}」没有默认值，无法继续", prompt);
        }
        println!(
            "  {} {} {}",
            style("?").cyan(),
            prompt,
            style(format!("[自动: {}]", if v.is_empty() { "(空)" } else { &v })).dim()
        );
        return Ok(v);
    }
    let theme = ColorfulTheme::default();
    let mut input = Input::<String>::with_theme(&theme)
        .with_prompt(prompt)
        .allow_empty(allow_empty);
    if let Some(d) = default {
        input = input.default(d.to_string());
    }
    Ok(input.interact_text()?.trim().to_string())
}

/// 创建一个转圈 spinner（调用方负责 finish_and_clear）
pub fn spinner(msg: &str) -> indicatif::ProgressBar {
    let pb = indicatif::ProgressBar::new_spinner();
    pb.set_style(
        indicatif::ProgressStyle::default_spinner()
            .template("  {spinner:.cyan} {msg}")
            .unwrap(),
    );
    pb.set_message(msg.to_string());
    pb.enable_steady_tick(std::time::Duration::from_millis(100));
    pb
}
