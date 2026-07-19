---
title: "配置文件 - hudo 配置说明"
description: "hudo 配置文件详解：自定义安装目录、版本锁定、镜像源设置，通过 config.toml 灵活控制工具安装行为。"
head:
  - - meta
    - name: keywords
      content: "hudo 配置, config.toml, hudo 安装目录, 镜像源配置, 版本锁定, hudo 自定义配置"
---

# 配置文件

hudo 的配置文件位于 `%USERPROFILE%\.hudo\config.toml`，首次运行时自动创建。

## 配置项说明

```toml
# 工具安装根目录
root_dir = "D:\\hudo"

[java]
version = "21"        # JDK 大版本

[go]
version = "latest"    # Go 版本，latest 表示自动获取最新版

[versions]
# 固定各工具版本，不填则自动获取最新版
# 可用键: git, gh, nodejs, fnm, mysql, pgsql, pycharm, maven, gradle, claude_code, redis
# git = "2.47.0"

[mirrors]
# 自定义下载镜像（可选）
# 可用键: uv, nodejs, fnm, go, java, vscode, pycharm, mysql, pgsql, maven, gradle, redis
# nodejs = "https://npmmirror.com/mirrors/node"
```

## 修改配置

三种方式任选：

```powershell
# 1. 命令行设置单项（键名见上方注释，未知键会列出全部可用键）
hudo config set mirrors.nodejs https://npmmirror.com/mirrors/node
hudo config set versions.git 2.47.0

# 2. 交互菜单：主菜单 [*] 配置 → 设置镜像 / 设置固定版本（本次会话立即生效）

# 3. 直接编辑配置文件
hudo config edit
```

::: warning 修改 root_dir 须知
`root_dir` 修改后不会迁移已安装的工具，旧目录中的安装记录将不再显示。命令行修改时 hudo 会要求二次确认。
:::

## 固定工具版本

如果需要安装指定版本，在 `[versions]` 下添加：

```toml
[versions]
nodejs = "20.11.0"
```

再次运行 `hudo install` 时会使用指定版本。版本查询失败（如网络受限）时，hudo 会明确提示"使用内置默认版本"，不会静默回退。
