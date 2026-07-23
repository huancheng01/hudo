---
title: "7-Zip 安装配置 - Windows 一键安装 7-Zip 免管理员 - hudo"
description: "使用 hudo 一键安装 7-Zip 便携版，免管理员权限、不写注册表，命令行 7z 和图形界面 7zFM 开箱即用。"
head:
  - - meta
    - name: keywords
      content: "7-Zip 安装, 7zip Windows, 7z 命令行, 免管理员安装, 压缩解压工具, hudo"
---

# 7-Zip

7-Zip 是 Windows 上最常用的开源压缩/解压工具，支持 7z、zip、tar、gz、rar（解压）等几乎所有常见格式。hudo 采用便携方式安装：用官方 7zr.exe 解出安装器载荷，不运行安装向导。

## 安装

```powershell
hudo install 7zip
```

安装到 `{install_root}\tools\7zip\`，免管理员权限、不写注册表，自动配置 PATH。

## 安装后

```powershell
# 命令行压缩/解压
7z a archive.7z .\folder\
7z x archive.7z -o.\out\

# 图形界面文件管理器
7zFM
```

## 卸载

```powershell
hudo uninstall 7zip
```

## 配置文件版本

```toml
[versions]
# 不填则自动获取最新版（GitHub ip7z/7zip 官方镜像）
7zip = "26.02"
```

## hudo 安装优势

- **免管理员**：官方安装器需要 UAC 提权，hudo 便携方式全程用户态
- **不污染系统**：不写注册表、不注册卸载器，删目录即卸载干净
- **CLI 直接可用**：官方安装器不把 7z 加入 PATH，hudo 装完终端直接 `7z`

## 已知限制

便携方式没有资源管理器右键菜单集成（那需要注册系统 DLL 与管理员权限）。需要右键集成的用户请使用官方安装器；hudo 检测外部安装只看 PATH，两者可以共存。
