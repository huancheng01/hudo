use anyhow::{Context, Result};
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

/// 进程级代理地址，启动时从 config 注入一次，作用于所有 reqwest 客户端
static PROXY: OnceLock<String> = OnceLock::new();

/// 设置全局代理（config.proxy）。地址无效时告警并忽略，不中断流程
pub fn set_proxy(url: &str) {
    match reqwest::Proxy::all(url) {
        Ok(_) => {
            let _ = PROXY.set(url.to_string());
        }
        Err(e) => crate::ui::print_warning(&format!("代理地址无效，已忽略: {} ({})", url, e)),
    }
}

/// 统一的 reqwest 客户端构建入口：带上全局代理；未设置时 reqwest 默认仍读系统代理环境变量
pub fn client_builder() -> reqwest::ClientBuilder {
    let mut builder = reqwest::Client::builder();
    if let Some(p) = PROXY.get() {
        if let Ok(proxy) = reqwest::Proxy::all(p.as_str()) {
            builder = builder.proxy(proxy);
        }
    }
    builder
}

/// 异步下载文件到 cache_dir，返回本地文件路径
/// 如果文件已存在则跳过下载
pub async fn download(url: &str, cache_dir: &Path, filename: &str) -> Result<PathBuf> {
    let dest = cache_dir.join(filename);

    // 缓存命中，跳过下载
    if dest.exists() {
        println!("  {} 使用缓存: {}", console::style("↓").cyan(), filename);
        return Ok(dest);
    }

    std::fs::create_dir_all(cache_dir)
        .with_context(|| format!("无法创建缓存目录: {}", cache_dir.display()))?;

    println!("  {} {}", console::style("↓").cyan(), console::style(url).dim());

    let client = client_builder()
        .connect_timeout(std::time::Duration::from_secs(10))
        .build()
        .unwrap_or_else(|_| reqwest::Client::new());
    let resp = client
        .get(url)
        .send()
        .await
        .with_context(|| format!("请求失败: {}", url))?
        .error_for_status()
        .with_context(|| format!("HTTP 错误: {}", url))?;

    // 写入临时文件，下载完成后再重命名，避免中断导致损坏
    let tmp_dest = cache_dir.join(format!("{}.tmp", filename));
    let result = download_to_tmp(&tmp_dest, resp).await;

    if let Err(e) = result {
        std::fs::remove_file(&tmp_dest).ok();
        return Err(e);
    }

    // 重命名为正式文件
    std::fs::rename(&tmp_dest, &dest)
        .with_context(|| format!("重命名临时文件失败: {}", tmp_dest.display()))?;

    println!("  {} {}", console::style("✓").green(), filename);
    Ok(dest)
}

/// 下载内容到临时文件
async fn download_to_tmp(tmp_dest: &Path, resp: reqwest::Response) -> Result<()> {
    let total_size = resp.content_length().unwrap_or(0);

    // 已知总大小走进度条；服务器不返回 Content-Length 时降级为 spinner + 已下载字节
    let pb = if total_size > 0 {
        let pb = ProgressBar::new(total_size);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("  {bar:40.cyan/blue}  {percent:>3}%  {bytes}/{total_bytes}  {bytes_per_sec}  {eta}")
                .unwrap()
                .progress_chars("━╸─"),
        );
        pb
    } else {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("  {spinner:.cyan} 已下载 {bytes}  {bytes_per_sec}")
                .unwrap(),
        );
        pb.enable_steady_tick(std::time::Duration::from_millis(100));
        pb
    };

    let mut file = std::fs::File::create(tmp_dest)
        .with_context(|| format!("无法创建临时文件: {}", tmp_dest.display()))?;

    let mut stream = resp.bytes_stream();
    loop {
        // 单块 60 秒读超时：连接建立后中途断流不再让进度条永久冻结
        let chunk = tokio::time::timeout(std::time::Duration::from_secs(60), stream.next())
            .await
            .map_err(|_| anyhow::anyhow!("下载超时（60 秒未收到数据），请检查网络后重试"))?;
        let Some(chunk) = chunk else { break };
        let chunk = chunk.context("下载数据流错误")?;
        std::io::Write::write_all(&mut file, &chunk).context("写入文件失败")?;
        pb.inc(chunk.len() as u64);
    }

    pb.finish_and_clear();
    Ok(())
}

/// 带镜像回退的下载：先尝试原地址，连接失败自动回退镜像
/// 仅在网络连接失败时回退（超时/DNS/连接拒绝），HTTP 404 等不回退
pub async fn download_with_fallback(
    url: &str,
    fallback_url: &str,
    cache_dir: &Path,
    filename: &str,
) -> Result<PathBuf> {
    let dest = cache_dir.join(filename);

    if dest.exists() {
        println!("  {} 使用缓存: {}", console::style("↓").cyan(), filename);
        return Ok(dest);
    }

    std::fs::create_dir_all(cache_dir)
        .with_context(|| format!("无法创建缓存目录: {}", cache_dir.display()))?;

    // 先尝试原地址
    match try_download(url, cache_dir, filename).await {
        Ok(path) => return Ok(path),
        Err(e) => {
            // 仅在连接级别失败时回退，HTTP 错误（如 404）不回退
            if is_connection_error(&e) {
                crate::ui::print_warning(&format!("原地址连接失败，尝试镜像下载..."));
            } else {
                return Err(e);
            }
        }
    }

    // 回退到镜像
    try_download(fallback_url, cache_dir, filename).await
}

/// 尝试下载，不使用缓存检查（由调用方负责）
async fn try_download(url: &str, cache_dir: &Path, filename: &str) -> Result<PathBuf> {
    let dest = cache_dir.join(filename);

    println!("  {} {}", console::style("↓").cyan(), console::style(url).dim());

    let client = client_builder()
        .connect_timeout(std::time::Duration::from_secs(10))
        .build()
        .unwrap_or_else(|_| reqwest::Client::new());

    let resp = client
        .get(url)
        .send()
        .await
        .with_context(|| format!("请求失败: {}", url))?
        .error_for_status()
        .with_context(|| format!("HTTP 错误: {}", url))?;

    let tmp_dest = cache_dir.join(format!("{}.tmp", filename));
    let result = download_to_tmp(&tmp_dest, resp).await;

    if let Err(e) = result {
        std::fs::remove_file(&tmp_dest).ok();
        return Err(e);
    }

    std::fs::rename(&tmp_dest, &dest)
        .with_context(|| format!("重命名临时文件失败: {}", tmp_dest.display()))?;

    println!("  {} {}", console::style("✓").green(), filename);
    Ok(dest)
}

/// 判断是否为连接级别错误（超时/DNS/连接拒绝），这类错误值得回退镜像
fn is_connection_error(err: &anyhow::Error) -> bool {
    let msg = format!("{:?}", err);
    msg.contains("connect")
        || msg.contains("timeout")
        || msg.contains("dns")
        || msg.contains("timed out")
        || msg.contains("下载超时")
        || msg.contains("Connection refused")
        || msg.contains("No address")
        || msg.contains("request")
}

/// 解压 zip 文件到目标目录
#[allow(dead_code)]
pub fn extract_zip(zip_path: &Path, dest_dir: &Path) -> Result<()> {
    std::fs::create_dir_all(dest_dir)
        .with_context(|| format!("无法创建解压目录: {}", dest_dir.display()))?;

    let file = std::fs::File::open(zip_path)
        .with_context(|| format!("无法打开 zip 文件: {}", zip_path.display()))?;
    let mut archive = zip::ZipArchive::new(file)
        .with_context(|| format!("无效的 zip 文件: {}", zip_path.display()))?;

    // 按条目数显示进度：大包（MinGW/PyCharm 数万文件）解压可达数分钟，不能全程静默
    let total = archive.len();
    let pb = ProgressBar::new(total as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("  {bar:40.cyan/blue}  {pos}/{len} 文件")
            .unwrap()
            .progress_chars("━╸─"),
    );

    for i in 0..total {
        let mut entry = archive.by_index(i).context("读取 zip 条目失败")?;
        let name = entry.name().to_string();

        let out_path = dest_dir.join(&name);

        if entry.is_dir() {
            std::fs::create_dir_all(&out_path).ok();
        } else {
            if let Some(parent) = out_path.parent() {
                std::fs::create_dir_all(parent).ok();
            }
            let mut outfile = std::fs::File::create(&out_path)
                .with_context(|| format!("无法创建文件: {}", out_path.display()))?;
            std::io::copy(&mut entry, &mut outfile)
                .with_context(|| format!("解压文件失败: {}", name))?;
        }
        pb.inc(1);
    }

    pb.finish_and_clear();
    Ok(())
}

/// 找到目录下唯一的子目录（用于 zip 解压后有一层顶层目录的情况）
pub fn find_single_subdir(dir: &Path) -> Option<PathBuf> {
    let entries: Vec<_> = std::fs::read_dir(dir)
        .ok()?
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
        .collect();
    if entries.len() == 1 {
        Some(entries[0].path())
    } else {
        None
    }
}

/// 运行 exe 安装程序（如 rustup-init.exe）
pub fn run_installer(exe_path: &Path, args: &[&str]) -> Result<()> {
    let status = std::process::Command::new(exe_path)
        .args(args)
        .status()
        .with_context(|| format!("无法启动安装程序: {}", exe_path.display()))?;

    if !status.success() {
        anyhow::bail!(
            "安装程序退出码: {}",
            status.code().unwrap_or(-1)
        );
    }

    Ok(())
}
