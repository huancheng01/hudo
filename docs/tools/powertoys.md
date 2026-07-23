---
title: "PowerToys 安装配置 - Windows 一键静默安装 PowerToys - hudo"
description: "使用 hudo 一键静默安装微软 PowerToys 效率工具集（FancyZones、PowerRename、颜色拾取器等），用户级安装免管理员。"
head:
  - - meta
    - name: keywords
      content: "PowerToys 安装, PowerToys Windows, FancyZones, PowerRename, 微软效率工具, hudo"
---

# PowerToys

PowerToys 是微软官方开源的 Windows 效率工具集：窗口布局（FancyZones）、批量重命名（PowerRename）、快速启动器（PowerToys Run）、颜色拾取器、按键映射等。在各类开发者装机清单中出现率极高。

## 安装

```powershell
hudo install powertoys
```

hudo 使用官方 **UserSetup** 用户级安装器静默安装（免管理员），安装路径由微软安装器决定（`%LOCALAPPDATA%\PowerToys`），不写入 hudo 目录。

## 安装后

从开始菜单启动 PowerToys，或等待其随系统自启。常用入口：

- `Win + Shift + T`：屏幕取词（Text Extractor）
- `Alt + Space`：PowerToys Run 快速启动器（需在设置中启用）

## 卸载

```powershell
hudo uninstall powertoys
```

hudo 会先停止 PowerToys 进程，再调用注册表中的静默卸载命令。

## 配置文件版本

```toml
[versions]
# 不填则自动获取最新版
powertoys = "0.100.2"
```

## 说明

- 与 [Chrome](/tools/chrome) 相同，安装路径由官方安装器决定，hudo 通过安装记录（state.json）区分是否由 hudo 安装
- 若系统中已有 PowerToys（winget/手动安装），hudo 检测后不会重复安装
