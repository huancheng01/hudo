---
title: "Rust Windows 安装与配置"
description: "使用 hudo 在 Windows 上一键安装 Rust 工具链（rustup），自动配置 PATH 和 CARGO_HOME，支持国内镜像源加速。"
head:
  - - meta
    - name: keywords
      content: "Rust 安装, Rust Windows, rustup 安装, Cargo 配置, 一键安装 Rust, Rust 国内镜像, hudo"
---

# Rust

Rust 是注重安全性和性能的系统级编程语言，通过 [rustup](https://rustup.rs) 管理工具链和版本。它在 CLI 工具、WebAssembly、嵌入式和高性能后端等领域广泛应用，是近年来增长最快的编程语言之一。

## 安装

```powershell
hudo install rust
```

安装 rustup 到 `{install_root}\tools\rustup\`，Cargo 到 `{install_root}\lang\cargo\`。

> 注意：Rust 编译需要 C/C++ 链接器。hudo 会自动检测并提示安装 MinGW（GCC）。

## 安装后

```powershell
rustc --version
cargo --version
rustup show
```

## 卸载

```powershell
hudo uninstall rust
```

## hudo 安装优势

- **绕过 MSVC 依赖**：hudo 自动检测并提示安装 [MinGW](/tools/mingw) 作为 C 链接器，无需安装体积庞大的 Visual Studio Build Tools
- **自动配置 RUSTUP_HOME 和 CARGO_HOME**：环境变量全自动设置，安装后即可使用 `rustc`、`cargo`、`rustup` 命令
- **安装目录规范**：rustup 安装在 `tools/rustup/`，Cargo 安装在 `lang/cargo/`，目录结构清晰不混乱
- **一键卸载**：`hudo uninstall rust` 完整清除 rustup 和 Cargo 相关文件，不留残余

## 常见问题

**Q: 安装后 `rustc` 命令找不到怎么办？**

打开新的终端窗口即可，hudo 已自动配置环境变量，需要新终端加载。

**Q: 编译时报错 `linker 'cc' not found` 怎么办？**

Rust 编译需要 C 链接器。运行 `hudo install c` 安装 [MinGW](/tools/mingw) GCC 工具链即可解决。hudo 在安装 Rust 时也会自动提示安装。

**Q: 如何切换 Rust 版本（stable/nightly）？**

hudo 安装的是 rustup 工具链管理器，安装后可以通过 `rustup default nightly` 切换到 nightly 版本，或通过 `rustup toolchain install 1.75.0` 安装指定版本。

## 相关阅读

- [Windows 安装 Rust 不踩坑：告别 MSVC 依赖，一条命令搞定](/blog/windows-rust-install) — 详细安装教程与常见问题解答
