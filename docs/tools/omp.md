---
title: "Oh My Posh + Nerd Font 安装配置 - Windows 终端美化一键完成 - hudo"
description: "使用 hudo 一键安装 Oh My Posh 终端主题引擎并自动安装注册 CaskaydiaCove Nerd Font 字体（用户级免管理员），自动写入 PowerShell profile。"
head:
  - - meta
    - name: keywords
      content: "Oh My Posh 安装, Nerd Font Windows, PowerShell 美化, 终端主题, CaskaydiaCove, hudo"
---

# Oh My Posh

Oh My Posh 是跨 shell 的终端提示符主题引擎，Windows Terminal 官方教程采用的美化方案。它强依赖 Nerd Font 字体（否则图标全是乱码方块），所以 hudo 把两者捆绑成一步：装引擎 + 装字体 + 写 profile 一次完成——这正是手动配置时最容易漏掉半步的地方。

## 安装

```powershell
hudo install omp
```

hudo 会依次完成：

1. 安装 oh-my-posh 主程序与官方主题包到 `{install_root}\tools\oh-my-posh\`
2. 用户级安装并注册 **CaskaydiaCove Nerd Font**（免管理员，Win10 1809+）
3. 配置 PATH 与 `POSH_THEMES_PATH`
4. 询问后写入 PowerShell profile（5.1 与 pwsh 7 都会写，幂等，带标记便于卸载清理）

装完后**手动做一步**：在终端设置中把字体切换为 `CaskaydiaCove Nerd Font`。

## 换主题

```powershell
# 浏览所有主题
oh-my-posh config export --help
Get-ChildItem $env:POSH_THEMES_PATH

# 修改 profile 中 --config 指向的主题文件即可
```

## 卸载

```powershell
hudo uninstall omp
```

卸载会逆向清理全部痕迹：profile 注入行、36 个字体文件与注册表项、环境变量、安装目录。

## 配置文件版本

```toml
[versions]
# 不填则自动获取最新版
omp = "29.35.1"
```

## 常见问题

### 图标显示为方块/问号？

终端字体没有切换到 Nerd Font。Windows Terminal：设置 → 配置文件 → 外观 → 字体 → 选 `CaskaydiaCove Nerd Font`。VS Code 集成终端：`terminal.integrated.fontFamily` 设为 `CaskaydiaCove Nerd Font`。

### 想用别的 Nerd Font？

hudo 默认装 CaskaydiaCove（微软 Cascadia Code 的 Nerd Font 版本，Windows 终端观感最协调）。其他字体可从 [nerdfonts.com](https://www.nerdfonts.com/) 下载后双击安装。

## 相关阅读

- [Windows 终端美化保姆级教程：PowerShell 7 + Oh My Posh + Nerd Font](/blog/windows-terminal-beautify) — 三件套完整配置流程
- [终端图标乱码、显示方框怎么办](/blog/windows-nerd-font-fix) — Nerd Font 三层排查

