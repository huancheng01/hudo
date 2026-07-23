---
title: "PowerShell 7 安装配置 - Windows 一键安装 pwsh - hudo"
description: "使用 hudo 一键安装 PowerShell 7 便携版，免管理员权限，与系统自带的 Windows PowerShell 5.1 共存。"
head:
  - - meta
    - name: keywords
      content: "PowerShell 7 安装, pwsh Windows, PowerShell 升级, Windows Terminal PowerShell, hudo"
---

# PowerShell 7

Windows 自带的是 2016 年停止大版本更新的 Windows PowerShell 5.1；PowerShell 7（`pwsh`）是跨平台的现代版本，性能更好、语法更完整（`&&`/`||`、三元运算符、并行 ForEach），也是各类终端美化教程（oh-my-posh 等）的默认前提。

## 安装

```powershell
hudo install pwsh
```

安装 zip 便携版到 `{install_root}\tools\pwsh\`，免管理员权限，自动配置 PATH。与系统自带的 `powershell`（5.1）共存，互不影响。

## 安装后

```powershell
# 新终端中启动
pwsh

# 查看版本
pwsh -Version
```

建议在 Windows Terminal 设置中把默认配置文件切换为 PowerShell 7。

## 卸载

```powershell
hudo uninstall pwsh
```

## 配置文件版本

```toml
[versions]
# 不填则自动获取最新版
pwsh = "7.6.4"
```

## hudo 安装优势

- **免管理员**：官方 MSI 默认机器级安装需要 UAC，hudo 用官方 zip 便携版全程用户态
- **干净卸载**：不写注册表，删目录 + 清 PATH 即彻底移除
- **版本可锁定**：通过[配置文件](/guide/config)固定版本

## 常见问题

### pwsh 和 powershell 有什么区别？

`powershell` 是系统自带的 5.1（仅安全维护），`pwsh` 是 PowerShell 7。二者可执行文件不同、可以共存；脚本兼容大部分场景，但 5.1 的部分 Windows 专属模块在 7 中需要兼容层。

### 为什么 Windows Terminal 里没有它？

便携版不向系统注册 Terminal 配置文件。在 Windows Terminal 设置中手动添加一个配置文件，命令行填 `{install_root}\tools\pwsh\pwsh.exe` 即可。
