use anyhow::Result;
use async_trait::async_trait;
use std::path::PathBuf;

use super::{
    query_service_exists, query_service_state, run_as_admin, DetectResult, EnvAction,
    InstallContext, InstallResult, Installer, ServiceState, ToolInfo,
};
use crate::config::HudoConfig;
use crate::download;

pub struct RedisInstaller;

const REDIS_VERSION_DEFAULT: &str = "8.6.2";
const REDIS_SERVICE_NAME: &str = "Redis";

#[async_trait]
impl Installer for RedisInstaller {
    fn info(&self) -> ToolInfo {
        ToolInfo {
            id: "redis",
            name: "Redis",
            description: "Redis 内存数据库",
        }
    }

    async fn detect_installed(&self, ctx: &InstallContext<'_>) -> Result<DetectResult> {
        let redis_exe = ctx
            .config
            .tools_dir()
            .join("redis")
            .join("redis-server.exe");
        if redis_exe.exists() {
            if let Ok(out) = std::process::Command::new(&redis_exe).arg("--version").output() {
                if out.status.success() {
                    let version = parse_redis_version(&String::from_utf8_lossy(&out.stdout));
                    return Ok(DetectResult::InstalledByHudo(version));
                }
            }
        }

        if let Ok(out) = std::process::Command::new("redis-server")
            .arg("--version")
            .output()
        {
            if out.status.success() {
                let version = parse_redis_version(&String::from_utf8_lossy(&out.stdout));
                return Ok(DetectResult::InstalledExternal(version));
            }
        }

        Ok(DetectResult::NotInstalled)
    }

    fn resolve_download(&self, config: &HudoConfig) -> (String, String) {
        let version = config
            .versions
            .redis
            .as_deref()
            .unwrap_or(REDIS_VERSION_DEFAULT);
        let filename = format!(
            "Redis-{}-Windows-x64-msys2-with-Service.zip",
            version
        );
        let base = config.mirrors.redis.as_deref().unwrap_or(
            "https://github.com/redis-windows/redis-windows/releases/download",
        );
        // tag 直接用版本号（新版不再带 .1 后缀）
        let url = format!(
            "{}/{}/{}",
            base.trim_end_matches('/'),
            version,
            filename
        );
        (url, filename)
    }

    async fn install(&self, ctx: &InstallContext<'_>) -> Result<InstallResult> {
        let config = ctx.config;
        let install_dir = config.tools_dir().join("redis");

        let (tag, version) = match &config.versions.redis {
            Some(v) => (v.clone(), v.clone()),
            None => {
                crate::ui::print_action("查询 Redis 最新版本...");
                match crate::version::redis_latest().await {
                    Some(v) => {
                        crate::ui::print_info(&format!("最新版本: {}", v.0));
                        v
                    }
                    None => {
                        crate::ui::print_warning(&format!(
                            "获取最新版本失败，使用内置默认版本 {}",
                            REDIS_VERSION_DEFAULT
                        ));
                        (
                            REDIS_VERSION_DEFAULT.to_string(),
                            REDIS_VERSION_DEFAULT.to_string(),
                        )
                    }
                }
            }
        };

        let filename = format!(
            "Redis-{}-Windows-x64-msys2-with-Service.zip",
            version
        );
        let base = config.mirrors.redis.as_deref().unwrap_or(
            "https://github.com/redis-windows/redis-windows/releases/download",
        );
        let url = format!(
            "{}/{}/{}",
            base.trim_end_matches('/'),
            tag,
            filename
        );

        let zip_path = download::download(&url, &config.cache_dir(), &filename).await?;

        crate::ui::print_action("解压 Redis...");
        let tmp_dir = config.cache_dir().join("redis-extract");
        if tmp_dir.exists() {
            std::fs::remove_dir_all(&tmp_dir).ok();
        }
        download::extract_zip(&zip_path, &tmp_dir)?;

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
            path: install_path.to_string_lossy().to_string(),
        }]
    }

    async fn configure(&self, ctx: &InstallContext<'_>) -> Result<()> {
        let install_dir = ctx.config.tools_dir().join("redis");

        // 1. 生成 redis.conf
        crate::ui::print_action("生成 redis.conf...");
        let conf_path = write_redis_conf(&install_dir)?;
        crate::ui::print_info(&format!("配置文件: {}", conf_path.display()));

        // 2. 注册 Windows 服务（with-Service 版自带 RedisService.exe）
        let service_exe = install_dir.join("RedisService.exe");
        if !service_exe.exists() {
            crate::ui::print_warning("未找到 RedisService.exe，跳过服务注册");
            crate::ui::print_info("可手动启动: redis-server redis.conf");
            return Ok(());
        }

        if !query_service_exists(REDIS_SERVICE_NAME) {
            crate::ui::print_action("注册 Redis Windows 服务...");
            let service_str = service_exe.to_string_lossy().to_string();

            // 先直接尝试；捕获输出避免直通控制台
            let _ = std::process::Command::new(&service_str)
                .arg("install")
                .output();

            if !query_service_exists(REDIS_SERVICE_NAME) {
                crate::ui::print_info("需要管理员权限，请在弹出的 UAC 窗口中点击\"是\"...");
                run_as_admin(&service_str, &["install"])?;

                if !query_service_exists(REDIS_SERVICE_NAME) {
                    anyhow::bail!("Redis 服务注册失败，请以管理员身份运行 hudo 后重试");
                }
            }
            crate::ui::print_success("Redis 服务注册成功");
        } else {
            crate::ui::print_info("Redis 服务已存在，跳过注册");
        }

        // 3. 启动服务
        match query_service_state(REDIS_SERVICE_NAME) {
            ServiceState::Running => {
                crate::ui::print_success("Redis 服务已在运行");
            }
            ServiceState::Stopped => {
                // net start 是同步阻塞调用，用 spinner 显示等待状态；
                // 输出必须捕获，net 的本地化输出会与 spinner 重绘互相覆盖花屏
                let pb = crate::ui::spinner("Redis 服务启动中...");
                let start_output = tokio::task::spawn_blocking(|| {
                    std::process::Command::new("net")
                        .args(["start", REDIS_SERVICE_NAME])
                        .output()
                })
                .await
                .ok()
                .and_then(|r| r.ok());
                pb.finish_and_clear();

                let direct_ok = start_output
                    .as_ref()
                    .map(|o| o.status.success())
                    .unwrap_or(false);

                if direct_ok {
                    crate::ui::print_success("Redis 服务已启动");
                } else {
                    crate::ui::print_info("需要管理员权限，请在弹出的 UAC 窗口中点击\"是\"...");
                    match run_as_admin("net", &["start", REDIS_SERVICE_NAME]) {
                        Ok(_) => crate::ui::print_success("Redis 服务已启动"),
                        Err(_) => {
                            crate::ui::print_warning("Redis 服务未能自动启动");
                            // 提权进程的输出无法捕获，展示直接尝试时的错误摘要辅助排查
                            if let Some(out) = &start_output {
                                for line in output_summary(out) {
                                    crate::ui::print_info(&line);
                                }
                            }
                            crate::ui::print_next_step("请以管理员身份运行: net start Redis");
                        }
                    }
                }
            }
            ServiceState::NotFound => {
                crate::ui::print_warning("Redis 服务未找到，请重新安装");
                return Ok(());
            }
        }

        crate::ui::print_box(&[
            format!(" {}", console::style("Redis 连接信息").bold()),
            String::new(),
            " 地址   127.0.0.1:6379（仅本机，无密码）".to_string(),
            " 连接   redis-cli".to_string(),
            " 启停   net start / net stop Redis（需管理员）".to_string(),
        ]);

        Ok(())
    }

    async fn pre_uninstall(&self, ctx: &InstallContext<'_>) -> Result<()> {
        let install_dir = ctx.config.tools_dir().join("redis");
        let service_exe = install_dir.join("RedisService.exe");

        // 服务不存在则无需清理；清理失败不中断卸载，但要明确告知
        match query_service_state(REDIS_SERVICE_NAME) {
            ServiceState::NotFound => return Ok(()),
            ServiceState::Running => {
                crate::ui::print_action("停止 Redis 服务...");
                if let Err(e) = run_as_admin("net", &["stop", REDIS_SERVICE_NAME]) {
                    crate::ui::print_warning(&format!("Redis 服务可能未停止: {}", e));
                    crate::ui::print_next_step("请以管理员身份运行: net stop Redis");
                }
            }
            ServiceState::Stopped => {}
        }

        crate::ui::print_action("移除 Redis 服务注册...");
        if service_exe.exists() {
            let service_str = service_exe.to_string_lossy().to_string();
            if let Err(e) = run_as_admin(&service_str, &["uninstall"]) {
                crate::ui::print_warning(&format!("Redis 服务注册可能未移除: {}", e));
                crate::ui::print_next_step("请以管理员身份运行: sc delete Redis");
            }
        } else {
            crate::ui::print_warning("未找到 RedisService.exe，无法自动移除服务注册");
            crate::ui::print_next_step("请以管理员身份运行: sc delete Redis");
        }

        Ok(())
    }
}

/// 生成 redis.conf 配置文件
fn write_redis_conf(install_dir: &PathBuf) -> Result<PathBuf> {
    let conf_path = install_dir.join("redis.conf");
    let data_dir = install_dir.join("data");
    std::fs::create_dir_all(&data_dir)?;

    let data_dir_str = data_dir.to_string_lossy().replace('\\', "/");

    let content = format!(
        "bind 127.0.0.1\n\
         port 6379\n\
         dir {data_dir}\n\
         appendonly yes\n\
         appendfilename \"appendonly.aof\"\n",
        data_dir = data_dir_str,
    );

    std::fs::write(&conf_path, content)?;
    Ok(conf_path)
}

/// 取子进程输出的最后几条非空行做摘要（stderr 优先），避免整段日志倾倒。
/// Windows 下 net 等命令可能输出 GBK 编码，from_utf8_lossy 会乱码但不引入转码依赖。
fn output_summary(out: &std::process::Output) -> Vec<String> {
    let stderr = String::from_utf8_lossy(&out.stderr);
    let text = if stderr.trim().is_empty() {
        String::from_utf8_lossy(&out.stdout).into_owned()
    } else {
        stderr.into_owned()
    };
    let lines: Vec<String> = text
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(String::from)
        .collect();
    let skip = lines.len().saturating_sub(5);
    lines.into_iter().skip(skip).collect()
}

/// 从 `redis-server --version` 输出中提取版本号
/// "Redis server v=8.6.1 sha=..." → "8.6.1"
fn parse_redis_version(output: &str) -> String {
    output
        .split("v=")
        .nth(1)
        .and_then(|s| s.split_whitespace().next())
        .unwrap_or("已安装")
        .to_string()
}
