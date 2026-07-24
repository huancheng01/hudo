---
title: "Miniconda 安装配置 - Windows 一键安装 Conda 环境"
description: "使用 hudo 一键安装 Miniconda，自动配置环境变量和国内镜像源，适合科学计算、数据分析和机器学习开发场景。"
head:
  - - meta
    - name: keywords
      content: "Miniconda 安装, Conda 安装, Windows Miniconda, Conda 环境配置, 数据分析环境, 科学计算, Python Conda, hudo"
---

# Miniconda

Miniconda 是 Anaconda 的精简版本，仅包含 Conda 包管理器和 Python 核心组件。它是科学计算、数据分析和机器学习开发者的首选环境管理工具，能够轻松创建隔离的 Python 环境并管理复杂的依赖关系（如 NumPy、PyTorch 等需要编译的库）。

Conda 包管理器最小安装版，适合科学计算、数据分析场景。

## 安装

```powershell
hudo install miniconda
```

静默安装到 `{install_root}\tools\miniconda\`，仅安装当前用户，不注册为系统 Python，不自动修改 PATH（由 hudo 统一管理）。

## 安装后

```powershell
conda --version

# 创建环境
conda create -n myenv python=3.11

# 激活环境
conda activate myenv

# 安装包
conda install numpy pandas
```

## 卸载

```powershell
hudo uninstall miniconda
```

## hudo 安装优势

- **静默安装，不污染系统**：仅安装到当前用户目录，不注册为系统 Python，不影响已有 Python 环境
- **环境变量统一管理**：不修改系统 PATH，由 hudo 统一管理，卸载时彻底清理
- **一条命令搞定**：无需手动下载安装包、点击安装向导、勾选配置项
- **与 [uv (Python)](/tools/python) 互补**：uv 适合纯 Python 包管理，Miniconda 适合科学计算生态（NumPy、PyTorch 等需要编译的库）

## 常见问题

### Miniconda 和 uv 应该选哪个？

如果你主要做 Web 开发或一般 Python 开发，推荐使用 [uv](/tools/python)，速度更快、更轻量。如果你需要 NumPy、SciPy、PyTorch 等科学计算库，或者需要管理非 Python 依赖（如 CUDA），推荐使用 Miniconda。两者可以共存，互不干扰。

### 安装后 `conda` 命令找不到？

重新打开终端窗口即可。hudo 会自动配置环境变量，但已打开的终端不会自动刷新。

### 如何配置国内镜像源？

安装后可以手动配置清华镜像：

```powershell
conda config --add channels https://mirrors.tuna.tsinghua.edu.cn/anaconda/pkgs/main
conda config --set show_channel_urls yes
```

## 相关阅读

- [Miniconda vs Anaconda：Windows 轻量 Python 环境 5 分钟搭建指南](/blog/windows-miniconda-install) — 详细安装教程与常见问题解答
