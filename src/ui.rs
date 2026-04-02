use std::io::Write;
use console::{measure_text_width, pad_str, style, Alignment, Style};

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
pub fn print_banner() {
    let stdout = std::io::stdout();
    let mut w = std::io::BufWriter::new(stdout.lock());
    for (i, line) in BANNER_LINES.iter().enumerate() {
        let (r, g, b) = GRADIENT_COLORS[i % GRADIENT_COLORS.len()];
        let _ = writeln!(w, "\x1b[1m\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, line);
    }
    let _ = writeln!(
        w,
        "  {} {}",
        style(format!("v{}", env!("CARGO_PKG_VERSION"))).dim(),
        style("混沌 — 开发环境一键引导工具").dim()
    );
    let _ = writeln!(w);
}

/// 清屏
pub fn clear_screen() {
    let mut stdout = std::io::stdout().lock();
    let _ = write!(stdout, "\x1B[2J\x1B[3J\x1B[H");
    let _ = stdout.flush();
}

/// 打印标题行（带边框）
pub fn print_title(text: &str) {
    let text_width = measure_text_width(text);
    let total_width = text_width.max(38) + 4; // 两侧各留 2 字符
    let fill = total_width - text_width - 4; // 右侧填充
    let s = Style::new().cyan();
    println!();
    println!(
        "{}",
        s.apply_to(format!("╭─ {} {}╮", text, "─".repeat(fill)))
    );
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
            "git" | "gh" | "claude-code" => ToolCategory::Tool,
            "uv" | "nodejs" | "bun" | "miniconda" | "rust" | "go" | "jdk" | "c" | "maven" | "gradle" => ToolCategory::Language,
            "mysql" | "pgsql" | "redis" => ToolCategory::Database,
            "vscode" | "pycharm" | "chrome" => ToolCategory::Ide,
            _ => ToolCategory::Tool,
        }
    }
}

/// 页面头部：清屏 + Banner + 标题
pub fn page_header(title: &str) {
    clear_screen();
    print_banner();
    print_title(title);
}

/// 暂停等待用户按键
pub fn wait_for_key() {
    println!();
    println!("  {}", style("按任意键返回...").dim());
    let _ = console::Term::stderr().read_key();
}
