use anyhow::Result;
use async_trait::async_trait;
use std::path::PathBuf;

use super::{
    query_service_exists, query_service_state, run_as_admin, DetectResult, EnvAction,
    InstallContext, InstallResult, Installer, ServiceState, ToolInfo,
};
use crate::config::HudoConfig;
use crate::download;

pub struct PgsqlInstaller;

const PG_VERSION_DEFAULT: &str = "17.8";
const PG_SERVICE_NAME: &str = "PostgreSQL";

#[async_trait]
impl Installer for PgsqlInstaller {
    fn info(&self) -> ToolInfo {
        ToolInfo {
            id: "pgsql",
            name: "PostgreSQL",
            description: "PostgreSQL 数据库",
        }
    }

    async fn detect_installed(&self, ctx: &InstallContext<'_>) -> Result<DetectResult> {
        let psql_exe = ctx.config.tools_dir().join("pgsql").join("bin").join("psql.exe");
        if psql_exe.exists() {
            if let Ok(out) = std::process::Command::new(&psql_exe).arg("--version").output() {
                if out.status.success() {
                    let version = parse_pgsql_version(&String::from_utf8_lossy(&out.stdout));
                    return Ok(DetectResult::InstalledByHudo(version));
                }
            }
        }

        if let Ok(out) = std::process::Command::new("psql").arg("--version").output() {
            if out.status.success() {
                let version = parse_pgsql_version(&String::from_utf8_lossy(&out.stdout));
                return Ok(DetectResult::InstalledExternal(version));
            }
        }

        Ok(DetectResult::NotInstalled)
    }

    fn resolve_download(&self, config: &HudoConfig) -> (String, String) {
        let version = config.versions.pgsql.as_deref().unwrap_or(PG_VERSION_DEFAULT);
        let filename = format!("postgresql-{}-1-windows-x64-binaries.zip", version);
        let base = config
            .mirrors
            .pgsql
            .as_deref()
            .unwrap_or("https://get.enterprisedb.com/postgresql");
        let url = format!("{}/{}", base.trim_end_matches('/'), filename);
        (url, filename)
    }

    async fn install(&self, ctx: &InstallContext<'_>) -> Result<InstallResult> {
        let config = ctx.config;
        let install_dir = config.tools_dir().join("pgsql");

        let version = match &config.versions.pgsql {
            Some(v) => v.clone(),
            None => {
                crate::ui::print_action("查询 PostgreSQL 最新版本...");
                match crate::version::pgsql_latest().await {
                    Some(v) => {
                        crate::ui::print_info(&format!("最新版本: {}", v));
                        v
                    }
                    None => {
                        crate::ui::print_warning(&format!(
                            "获取最新版本失败，使用内置默认版本 {}",
                            PG_VERSION_DEFAULT
                        ));
                        PG_VERSION_DEFAULT.to_string()
                    }
                }
            }
        };

        let filename = format!("postgresql-{}-1-windows-x64-binaries.zip", version);
        let base = config
            .mirrors
            .pgsql
            .as_deref()
            .unwrap_or("https://get.enterprisedb.com/postgresql");
        let url = format!("{}/{}", base.trim_end_matches('/'), filename);

        let zip_path = download::download(&url, &config.cache_dir(), &filename).await?;

        crate::ui::print_action("解压 PostgreSQL...");
        let tmp_dir = config.cache_dir().join("pgsql-extract");
        if tmp_dir.exists() {
            std::fs::remove_dir_all(&tmp_dir).ok();
        }
        download::extract_zip(&zip_path, &tmp_dir)?;

        // zip 内有 pgsql/ 子目录
        let inner = tmp_dir.join("pgsql");
        if install_dir.exists() {
            std::fs::remove_dir_all(&install_dir).ok();
        }
        if inner.exists() {
            std::fs::rename(&inner, &install_dir).ok();
        } else {
            let sub = download::find_single_subdir(&tmp_dir).unwrap_or(tmp_dir.clone());
            std::fs::rename(&sub, &install_dir).ok();
        }
        std::fs::remove_dir_all(&tmp_dir).ok();

        Ok(InstallResult {
            install_path: install_dir,
            version,
        })
    }

    fn env_actions(&self, install_path: &PathBuf, _config: &HudoConfig) -> Vec<EnvAction> {
        vec![
            EnvAction::Set {
                name: "PGDATA".to_string(),
                value: install_path.join("data").to_string_lossy().to_string(),
            },
            EnvAction::AppendPath {
                path: install_path.join("bin").to_string_lossy().to_string(),
            },
        ]
    }

    async fn configure(&self, ctx: &InstallContext<'_>) -> Result<()> {
        let install_dir = ctx.config.tools_dir().join("pgsql");
        let initdb = install_dir.join("bin").join("initdb.exe");
        let pg_ctl = install_dir.join("bin").join("pg_ctl.exe");
        let data_dir = install_dir.join("data");

        // 1. 初始化数据目录（无需管理员权限）
        let is_data_empty = data_dir
            .read_dir()
            .map(|mut d| d.next().is_none())
            .unwrap_or(true);

        if is_data_empty {
            // initdb 会直通约 25 行英文日志，与 spinner 重绘互相覆盖，必须捕获
            let pb = crate::ui::spinner("初始化 PostgreSQL 数据目录...");
            let output = std::process::Command::new(&initdb)
                .args([
                    "-D",
                    &data_dir.to_string_lossy(),
                    "-U",
                    "postgres",
                    "-E",
                    "UTF8",
                    "--no-locale",
                ])
                .output();
            pb.finish_and_clear();

            let manual_cmd = format!("请手动执行: initdb -D {} -U postgres", data_dir.display());
            match output {
                Ok(out) if out.status.success() => {
                    crate::ui::print_success("数据目录初始化完成");
                }
                Ok(out) => {
                    crate::ui::print_warning("PostgreSQL 初始化失败");
                    for line in output_summary(&out) {
                        crate::ui::print_info(&line);
                    }
                    crate::ui::print_next_step(&manual_cmd);
                    return Ok(());
                }
                Err(e) => {
                    crate::ui::print_warning(&format!("PostgreSQL 初始化失败: {}", e));
                    crate::ui::print_next_step(&manual_cmd);
                    return Ok(());
                }
            }
        }

        // 2. 注册 Windows 服务（需要管理员权限）
        if !query_service_exists(PG_SERVICE_NAME) {
            crate::ui::print_action("注册 PostgreSQL Windows 服务...");
            let pg_ctl_str = pg_ctl.to_string_lossy().to_string();
            let data_str = data_dir.to_string_lossy().to_string();

            // 先直接尝试（hudo 以管理员运行时无需 UAC）；捕获输出避免直通控制台
            let _ = std::process::Command::new(&pg_ctl_str)
                .args(["register", "-N", PG_SERVICE_NAME, "-D", &data_str])
                .output();

            // pg_ctl register 权限不足时可能返回 0，用 sc query 验证
            if !query_service_exists(PG_SERVICE_NAME) {
                crate::ui::print_info("需要管理员权限，请在弹出的 UAC 窗口中点击\"是\"...");
                run_as_admin(&pg_ctl_str, &["register", "-N", PG_SERVICE_NAME, "-D", &data_str])?;

                if !query_service_exists(PG_SERVICE_NAME) {
                    anyhow::bail!("PostgreSQL 服务注册失败，请以管理员身份运行 hudo 后重试");
                }
            }
            crate::ui::print_success("PostgreSQL 服务注册成功");
        } else {
            crate::ui::print_info("PostgreSQL 服务已存在，跳过注册");
        }

        // 3. 启动服务
        match query_service_state(PG_SERVICE_NAME) {
            ServiceState::Running => {
                crate::ui::print_success("PostgreSQL 服务已在运行");
            }
            ServiceState::Stopped => {
                // net start 是同步阻塞调用，用 spinner 显示等待状态；
                // 输出必须捕获，net 的本地化输出会与 spinner 重绘互相覆盖花屏
                let pb = crate::ui::spinner("PostgreSQL 服务启动中...");
                let start_output = tokio::task::spawn_blocking(|| {
                    std::process::Command::new("net")
                        .args(["start", PG_SERVICE_NAME])
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
                    crate::ui::print_success("PostgreSQL 服务已启动");
                } else {
                    crate::ui::print_info("需要管理员权限，请在弹出的 UAC 窗口中点击\"是\"...");
                    match run_as_admin("net", &["start", PG_SERVICE_NAME]) {
                        Ok(_) => crate::ui::print_success("PostgreSQL 服务已启动"),
                        Err(_) => {
                            crate::ui::print_warning("PostgreSQL 服务未能自动启动");
                            // 提权进程的输出无法捕获，展示直接尝试时的错误摘要辅助排查
                            if let Some(out) = &start_output {
                                for line in output_summary(out) {
                                    crate::ui::print_info(&line);
                                }
                            }
                            crate::ui::print_next_step("请以管理员身份运行: net start PostgreSQL");
                        }
                    }
                }
            }
            ServiceState::NotFound => {
                crate::ui::print_warning("PostgreSQL 服务未找到，请重新安装");
                return Ok(());
            }
        }

        crate::ui::print_box(&[
            format!(" {}", console::style("PostgreSQL 连接信息").bold()),
            String::new(),
            " 地址   127.0.0.1:5432".to_string(),
            " 账号   postgres（trust 认证，本地免密）".to_string(),
            " 连接   psql -U postgres".to_string(),
            " 启停   net start / net stop PostgreSQL（需管理员）".to_string(),
        ]);

        Ok(())
    }

    async fn pre_uninstall(&self, ctx: &InstallContext<'_>) -> Result<()> {
        let pg_ctl = ctx
            .config
            .tools_dir()
            .join("pgsql")
            .join("bin")
            .join("pg_ctl.exe");
        let pg_ctl_str = pg_ctl.to_string_lossy().to_string();

        // 服务不存在则无需清理；清理失败不中断卸载，但要明确告知
        match query_service_state(PG_SERVICE_NAME) {
            ServiceState::NotFound => return Ok(()),
            ServiceState::Running => {
                crate::ui::print_action("停止 PostgreSQL 服务...");
                if let Err(e) = run_as_admin("net", &["stop", PG_SERVICE_NAME]) {
                    crate::ui::print_warning(&format!("PostgreSQL 服务可能未停止: {}", e));
                    crate::ui::print_next_step("请以管理员身份运行: net stop PostgreSQL");
                }
            }
            ServiceState::Stopped => {}
        }

        crate::ui::print_action("移除 PostgreSQL 服务注册...");
        if let Err(e) = run_as_admin(&pg_ctl_str, &["unregister", "-N", PG_SERVICE_NAME]) {
            crate::ui::print_warning(&format!("PostgreSQL 服务注册可能未移除: {}", e));
            crate::ui::print_next_step("请以管理员身份运行: sc delete PostgreSQL");
        }

        Ok(())
    }
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

/// 从 `psql --version` 输出中提取版本号
/// "psql (PostgreSQL) 17.8" → "17.8"
fn parse_pgsql_version(output: &str) -> String {
    output
        .split(')')
        .nth(1)
        .and_then(|s| s.split_whitespace().next())
        .unwrap_or("已安装")
        .to_string()
}
