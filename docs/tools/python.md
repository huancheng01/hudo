---
title: "Python uv 安装配置 - Windows 一键安装 Python 环境 - hudo"
description: "使用 hudo 一键安装 uv Python 包管理器，自动配置环境变量，支持国内镜像加速下载，极速管理 Python 版本和虚拟环境。"
head:
  - - meta
    - name: keywords
      content: "Python 安装, uv Python, Windows Python 安装, Python 环境配置, Python 包管理器, uv 安装, Python 虚拟环境, hudo"
---

# Python (uv)

Python 是全球最流行的编程语言之一，广泛用于数据科学、Web 开发、自动化脚本和 AI/ML 领域。hudo 通过 [uv](https://github.com/astral-sh/uv) 管理 Python 环境——uv 是 Rust 编写的极速 Python 包管理器，安装速度比 pip 快 10-100 倍。

## 安装

```powershell
hudo install uv
```

安装 uv 到 `{install_root}\tools\uv\`。

## 安装后

```powershell
uv --version

# 创建虚拟环境
uv venv

# 安装包
uv pip install requests

# 运行 Python
uv run python script.py
```

## 安装指定 Python 版本

```powershell
uv python install 3.12
uv python list
```

## 卸载

```powershell
hudo uninstall uv
```

## hudo 安装优势

- **无需从 python.org 下载安装包**：`hudo install uv` 一条命令安装 uv，然后通过 `uv python install` 按需安装任意 Python 版本
- **自动配置 PATH**：安装后 `uv` 命令在所有终端中立即可用
- **极速包管理**：uv 的包安装和依赖解析速度比 pip 快 10-100 倍，大幅缩短 `pip install` 等待时间
- **Python 版本管理内置**：无需额外安装 pyenv-win 等工具，`uv python install 3.12` 即可安装指定 Python 版本
- **与 [Miniconda](/tools/miniconda) 互补**：uv 适合通用 Python 开发，Miniconda 适合数据科学和科学计算场景

## 常见问题

**Q: 安装后 `uv` 命令找不到怎么办？**

打开新的终端窗口即可，hudo 已自动配置环境变量，需要新终端加载。

**Q: uv 和 pip 有什么区别？**

uv 是 pip 的高性能替代品，完全兼容 pip 的命令格式（`uv pip install`），同时还提供虚拟环境管理、Python 版本管理等功能。速度比 pip 快 10-100 倍。

**Q: 如何在项目中使用特定 Python 版本？**

运行 `uv python install 3.12` 安装所需版本，然后在项目目录中运行 `uv venv --python 3.12` 创建使用该版本的虚拟环境。

## 相关阅读

- [pip 太慢？uv 一键管理 Python 版本和虚拟环境（Windows 实战）](/blog/windows-python-uv-install) — 详细安装教程与常见问题解答
