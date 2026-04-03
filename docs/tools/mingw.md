---
title: "MinGW GCC 安装配置 - Windows 一键安装 C/C++ 编译器 - hudo"
description: "使用 hudo 一键安装 MinGW-w64 GCC 编译器工具链，无需 MSYS2，自动配置 PATH 环境变量，支持 gcc/g++/make 命令。"
head:
  - - meta
    - name: keywords
      content: "MinGW 安装, GCC 安装, Windows C++ 编译器, MinGW-w64, C语言环境搭建, gcc 安装 Windows, hudo"
---

# MinGW (C/C++)

MinGW-w64 是 Windows 平台的 GCC 编译器工具链，让你无需安装庞大的 Visual Studio 即可编译 C/C++ 程序。它也是 Rust 在 Windows 上使用 `gnu` 工具链时必需的链接器，是系统级开发的基础组件。

MinGW-w64 GCC 编译器工具链，来自 [winlibs](https://winlibs.com) 独立构建版，无需 MSYS2。

## 安装

```powershell
hudo install c
```

安装到 `{install_root}\tools\mingw64\`，自动获取最新版本（UCRT 运行时，来自 winlibs 独立构建）。

> Rust 安装时若检测到缺少链接器，会自动提示安装 MinGW。

## 安装后

```powershell
gcc --version
g++ --version
```

## 卸载

```powershell
hudo uninstall c
```

## hudo 安装优势

- **无需安装 Visual Studio**：不用下载几个 GB 的 Visual Studio，一条命令获得完整的 C/C++ 编译器
- **无需 MSYS2**：使用 winlibs 独立构建版，解压即用，不依赖 MSYS2 包管理器
- **UCRT 运行时**：使用现代 Universal C Runtime，兼容性更好
- **Rust 开发必备**：[Rust](/tools/rust) 安装时若检测到缺少链接器会自动提示安装 MinGW，hudo 让这一步无缝衔接
- **自动配置 PATH**：`gcc`、`g++`、`make` 等命令安装后即可在任意终端使用

## 常见问题

### MinGW 和 MSVC 应该选哪个？

如果你只需要编译 C/C++ 程序或为 Rust 提供链接器，MinGW 足够且安装体积小很多。如果你需要开发 Windows 原生应用（使用 Win32 API、COM 等），则需要 MSVC（通过 Visual Studio 安装）。

### 安装后 `gcc` 命令找不到？

重新打开终端窗口，让环境变量生效。如果仍然找不到，运行 `hudo list` 确认 MinGW 已安装成功。

### 如何搭配 VS Code 进行 C/C++ 开发？

安装 MinGW 后，在 [VS Code](/tools/vscode) 中安装 C/C++ 扩展（Microsoft 官方），即可获得代码补全、调试和编译功能。

## 相关阅读

- [不装 Visual Studio 也能写 C/C++ — MinGW-w64 一键安装 + VS Code 配置全流程](/blog/windows-mingw-install) — 详细安装教程与常见问题解答
