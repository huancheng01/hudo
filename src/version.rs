use reqwest::Client;

/// GitHub 仓库（owner/repo），用于自更新检查
pub const GITHUB_REPO: &str = "zexadev/hudo";

/// 复用同一个 client，带 5 秒超时（经 download::client_builder 统一走全局代理）
fn make_client() -> reqwest::Result<Client> {
    crate::download::client_builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
}

/// GitHub CLI: GitHub API → 最新版本号（如 "2.87.3"）
pub async fn gh_latest() -> Option<String> {
    let client = make_client().ok()?;
    let resp: serde_json::Value = client
        .get("https://api.github.com/repos/cli/cli/releases/latest")
        .header("User-Agent", "hudo")
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()?;
    let tag = resp["tag_name"].as_str()?; // "v2.87.3"
    Some(tag.trim_start_matches('v').to_string())
}

/// Git: GitHub API → tag "v2.47.1.windows.2" → "2.47.1.2"
pub async fn git_latest() -> Option<String> {
    let client = make_client().ok()?;
    let resp: serde_json::Value = client
        .get("https://api.github.com/repos/git-for-windows/git/releases/latest")
        .header("User-Agent", "hudo")
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()?;
    let tag = resp["tag_name"].as_str()?;
    parse_git_tag(tag)
}

/// "v2.47.1.windows.2" → "2.47.1.2", "v2.53.0.windows.1" → "2.53.0"
fn parse_git_tag(tag: &str) -> Option<String> {
    let tag = tag.strip_prefix('v')?;
    let parts: Vec<&str> = tag.split('.').collect();
    // ["2","47","1","windows","2"] or ["2","53","0","windows","1"]
    let idx = parts.iter().position(|&p| p == "windows")?;
    let ver_parts = &parts[..idx]; // ["2","47","1"]
    let win_patch = parts.get(idx + 1)?; // "2" or "1"
    if *win_patch == "1" {
        Some(ver_parts.join(".")) // "2.53.0"
    } else {
        Some(format!("{}.{}", ver_parts.join("."), win_patch)) // "2.47.1.2"
    }
}

/// Go: go.dev/dl API → "1.24.0"
pub async fn go_latest() -> Option<String> {
    let client = make_client().ok()?;
    let resp: Vec<serde_json::Value> = client
        .get("https://go.dev/dl/?mode=json")
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()?;
    let ver = resp.first()?["version"].as_str()?; // "go1.24.0"
    Some(ver.strip_prefix("go")?.to_string())
}

/// PostgreSQL: versions.json → 当前大版本最新完整版本号（如 "18.2"）
pub async fn pgsql_latest() -> Option<String> {
    let client = make_client().ok()?;
    let resp: Vec<serde_json::Value> = client
        .get("https://www.postgresql.org/versions.json")
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()?;
    resp.iter()
        .find(|v| v["current"].as_bool() == Some(true))
        .and_then(|v| {
            let major = v["major"].as_str()?;
            let minor = v["latestMinor"].as_str()?;
            Some(format!("{}.{}", major, minor))
        })
}

/// Maven: GitHub API → 最新稳定版本号（如 "3.9.9"）
pub async fn maven_latest() -> Option<String> {
    let client = make_client().ok()?;
    let resp: serde_json::Value = client
        .get("https://api.github.com/repos/apache/maven/releases/latest")
        .header("User-Agent", "hudo")
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()?;
    // tag_name 格式: "maven-3.9.9"
    let tag = resp["tag_name"].as_str()?;
    tag.strip_prefix("maven-").map(|s| s.to_string())
}

/// Gradle: services.gradle.org API → 最新发布版本号（如 "8.12.1"）
pub async fn gradle_latest() -> Option<String> {
    let client = make_client().ok()?;
    let resp: serde_json::Value = client
        .get("https://services.gradle.org/versions/current")
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()?;
    resp["version"].as_str().map(|s| s.to_string())
}

/// PyCharm: JetBrains API → 最新 CE 版本号
pub async fn pycharm_latest() -> Option<String> {
    let client = make_client().ok()?;
    let resp: serde_json::Value = client
        .get("https://data.services.jetbrains.com/products/releases?code=PCC&latest=true&type=release")
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()?;
    resp["PCC"][0]["version"].as_str().map(|s| s.to_string())
}

/// Oh My Posh: GitHub API → 最新版本号（tag "v29.35.1" → "29.35.1"）
pub async fn omp_latest() -> Option<String> {
    let client = make_client().ok()?;
    let resp: serde_json::Value = client
        .get("https://api.github.com/repos/JanDeDobbeleer/oh-my-posh/releases/latest")
        .header("User-Agent", "hudo")
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()?;
    let tag = resp["tag_name"].as_str()?;
    Some(tag.trim_start_matches('v').to_string())
}

/// PowerToys: GitHub API → 最新版本号（tag "v0.100.2" → "0.100.2"）
pub async fn powertoys_latest() -> Option<String> {
    let client = make_client().ok()?;
    let resp: serde_json::Value = client
        .get("https://api.github.com/repos/microsoft/PowerToys/releases/latest")
        .header("User-Agent", "hudo")
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()?;
    let tag = resp["tag_name"].as_str()?;
    Some(tag.trim_start_matches('v').to_string())
}

/// .NET SDK: releases-index.json → 最新活跃 LTS 通道的 latest-sdk（如 "10.0.302"）
/// 只取 LTS：STS 通道 18 个月即停止支持，不适合默认安装
pub async fn dotnet_latest() -> Option<String> {
    let client = make_client().ok()?;
    let resp: serde_json::Value = client
        .get("https://builds.dotnet.microsoft.com/dotnet/release-metadata/releases-index.json")
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()?;
    resp["releases-index"]
        .as_array()?
        .iter()
        .find(|e| {
            e["release-type"].as_str() == Some("lts")
                && e["support-phase"].as_str() == Some("active")
        })
        .and_then(|e| e["latest-sdk"].as_str())
        .map(|s| s.to_string())
}

/// PowerShell 7: GitHub API → 最新版本号（tag "v7.6.4" → "7.6.4"）
pub async fn pwsh_latest() -> Option<String> {
    let client = make_client().ok()?;
    let resp: serde_json::Value = client
        .get("https://api.github.com/repos/PowerShell/PowerShell/releases/latest")
        .header("User-Agent", "hudo")
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()?;
    let tag = resp["tag_name"].as_str()?;
    Some(tag.trim_start_matches('v').to_string())
}

/// 7-Zip: GitHub API (ip7z/7zip 官方镜像) → 最新版本号（tag 即版本，如 "26.02"）
pub async fn sevenzip_latest() -> Option<String> {
    let client = make_client().ok()?;
    let resp: serde_json::Value = client
        .get("https://api.github.com/repos/ip7z/7zip/releases/latest")
        .header("User-Agent", "hudo")
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()?;
    resp["tag_name"].as_str().map(|s| s.to_string())
}

/// IntelliJ IDEA: JetBrains API → (最新版本, Windows zip 下载链接)
/// 链接直接取 API 返回：2025.3 起社区版并入统一发行版，文件命名跨代变化，不手拼
pub async fn idea_latest() -> Option<(String, String)> {
    let client = make_client().ok()?;
    let resp: serde_json::Value = client
        .get("https://data.services.jetbrains.com/products/releases?code=IIC&latest=true&type=release")
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()?;
    let entry = &resp["IIC"][0];
    let version = entry["version"].as_str()?.to_string();
    let link = entry["downloads"]["windowsZip"]["link"].as_str()?.to_string();
    Some((version, link))
}

/// IntelliJ IDEA: 查指定版本的 Windows zip 下载链接（版本锁定用）
pub async fn idea_release_link(version: &str) -> Option<String> {
    let client = make_client().ok()?;
    let resp: serde_json::Value = client
        .get("https://data.services.jetbrains.com/products/releases?code=IIC&type=release")
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()?;
    resp["IIC"]
        .as_array()?
        .iter()
        .find(|e| e["version"].as_str() == Some(version))
        .and_then(|e| e["downloads"]["windowsZip"]["link"].as_str())
        .map(|s| s.to_string())
}

/// Claude Code: GCS → 最新版本号
pub async fn claude_code_latest() -> Option<String> {
    let client = make_client().ok()?;
    let resp = client
        .get(format!(
            "{}/latest",
            "https://storage.googleapis.com/claude-code-dist-86c565f3-f756-42ad-8dfa-d59b1c096819/claude-code-releases"
        ))
        .send()
        .await
        .ok()?
        .text()
        .await
        .ok()?;
    Some(resp.trim().to_string())
}

/// Redis: GitHub API (redis-windows) → 最新版本号（如 "8.6.1"）
/// tag 格式: "8.6.1.1" → 取前三段 "8.6.1"
/// 返回 (tag, version)，tag 用于下载 URL，version 用于文件名
/// 例如 tag="8.6.2" version="8.6.2" 或 tag="8.6.1.1" version="8.6.1"
pub async fn redis_latest() -> Option<(String, String)> {
    let client = make_client().ok()?;
    let resp: serde_json::Value = client
        .get("https://api.github.com/repos/redis-windows/redis-windows/releases/latest")
        .header("User-Agent", "hudo")
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()?;
    let tag = resp["tag_name"].as_str()?;
    // 取前三段作为 Redis 版本号（文件名用）
    let parts: Vec<&str> = tag.split('.').collect();
    let version = if parts.len() >= 3 {
        parts[..3].join(".")
    } else {
        tag.to_string()
    };
    Some((tag.to_string(), version))
}

/// MySQL: endoflife.date API → 最新 LTS 周期的最新补丁版（如 "9.7.1"）
/// 只取 LTS：innovation 版本发布三个月即 EOL，不适合默认安装
pub async fn mysql_latest() -> Option<String> {
    let client = make_client().ok()?;
    let resp: Vec<serde_json::Value> = client
        .get("https://endoflife.date/api/mysql.json")
        .header("User-Agent", "hudo")
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()?;
    // 数组按周期从新到旧排列，第一个 lts 条目即当前最新 LTS
    resp.iter()
        .find(|v| v["lts"].as_bool() == Some(true))
        .and_then(|v| v["latest"].as_str())
        .map(|s| s.to_string())
}

/// Node.js: nodejs.org API → 最新 LTS 版本号（如 "22.14.0"）
pub async fn nodejs_lts_latest() -> Option<String> {
    let client = make_client().ok()?;
    let resp: Vec<serde_json::Value> = client
        .get("https://nodejs.org/dist/index.json")
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()?;
    // 找第一个 lts 不为 false 的条目
    let entry = resp.iter().find(|v| !v["lts"].is_boolean() || v["lts"].as_bool() == Some(true))?;
    let ver = entry["version"].as_str()?; // "v22.14.0"
    Some(ver.trim_start_matches('v').to_string())
}

/// MinGW-w64 via winlibs：GitHub Releases → (tag, filename, gcc_version)
/// tag 格式: "15.2.0posix-13.0.0-ucrt-r6"
/// 文件格式: "winlibs-x86_64-posix-seh-gcc-15.2.0-mingw-w64ucrt-13.0.0-r6.zip"
pub async fn mingw_latest() -> Option<(String, String, String)> {
    let client = make_client().ok()?;
    let resp: serde_json::Value = client
        .get("https://api.github.com/repos/brechtsanders/winlibs_mingw/releases/latest")
        .header("User-Agent", "hudo")
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()?;
    let tag = resp["tag_name"].as_str()?.to_string();
    // 从 assets 找 x86_64 posix ucrt zip
    let filename = resp["assets"]
        .as_array()?
        .iter()
        .filter_map(|a| a["name"].as_str())
        .find(|name| {
            name.contains("x86_64")
                && name.contains("posix")
                && name.contains("ucrt")
                && name.ends_with(".zip")
        })?
        .to_string();
    // 从文件名提取 gcc 版本: "winlibs-x86_64-posix-seh-gcc-15.2.0-mingw-..."
    let gcc_version = filename
        .strip_prefix("winlibs-x86_64-posix-seh-gcc-")?
        .split('-')
        .next()?
        .to_string();
    Some((tag, filename, gcc_version))
}

/// fnm: GitHub API → 最新版本号（tag "v1.39.0" → "1.39.0"）
pub async fn fnm_latest() -> Option<String> {
    let client = make_client().ok()?;
    let resp: serde_json::Value = client
        .get("https://api.github.com/repos/Schniz/fnm/releases/latest")
        .header("User-Agent", "hudo")
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()?;
    let tag = resp["tag_name"].as_str()?;
    Some(tag.trim_start_matches('v').to_string())
}

/// Bun: GitHub API → 最新版本号（tag "bun-v1.3.14" → "1.3.14"）
pub async fn bun_latest() -> Option<String> {
    let client = make_client().ok()?;
    let resp: serde_json::Value = client
        .get("https://api.github.com/repos/oven-sh/bun/releases/latest")
        .header("User-Agent", "hudo")
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()?;
    let tag = resp["tag_name"].as_str()?;
    tag.strip_prefix("bun-v").map(|s| s.to_string())
}

/// uv: GitHub API → 最新版本号（tag 无前缀，如 "0.11.31"）
pub async fn uv_latest() -> Option<String> {
    let client = make_client().ok()?;
    let resp: serde_json::Value = client
        .get("https://api.github.com/repos/astral-sh/uv/releases/latest")
        .header("User-Agent", "hudo")
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()?;
    let tag = resp["tag_name"].as_str()?;
    Some(tag.trim_start_matches('v').to_string())
}

/// VS Code: releases API → 最新稳定版（数组按新到旧排列，如 "1.130.0"）
pub async fn vscode_latest() -> Option<String> {
    let client = make_client().ok()?;
    let resp: Vec<String> = client
        .get("https://update.code.visualstudio.com/api/releases/stable")
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()?;
    resp.into_iter().next()
}

/// JDK: Adoptium API → 指定主版本的最新 GA 版本（semver "21.0.11+10.0.LTS" → "21.0.11"）
pub async fn jdk_latest(major: &str) -> Option<String> {
    let client = make_client().ok()?;
    let resp: Vec<serde_json::Value> = client
        .get(format!(
            "https://api.adoptium.net/v3/assets/latest/{}/hotspot?image_type=jdk&os=windows&architecture=x64",
            major
        ))
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()?;
    let semver = resp.first()?["version"]["semver"].as_str()?;
    Some(semver.split('+').next()?.to_string())
}

/// hudo 自身：GitHub Releases → 最新版本号（如 "0.2.0"）
pub async fn hudo_latest() -> Option<String> {
    let client = make_client().ok()?;
    let resp: serde_json::Value = client
        .get(&format!(
            "https://api.github.com/repos/{}/releases/latest",
            GITHUB_REPO
        ))
        .header("User-Agent", "hudo")
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()?;
    let tag = resp["tag_name"].as_str()?; // "v0.2.0"
    Some(tag.trim_start_matches('v').to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_git_tag_with_patch() {
        assert_eq!(
            parse_git_tag("v2.47.1.windows.2"),
            Some("2.47.1.2".to_string())
        );
    }

    #[test]
    fn test_parse_git_tag_without_patch() {
        assert_eq!(
            parse_git_tag("v2.53.0.windows.1"),
            Some("2.53.0".to_string())
        );
    }

    #[test]
    fn test_parse_git_tag_invalid() {
        assert_eq!(parse_git_tag("invalid"), None);
        assert_eq!(parse_git_tag("2.47.1"), None);
    }
}
