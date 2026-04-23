---
title: "fnm Windows 安装与配置 - hudo"
description: "使用 hudo 在 Windows 上一键安装 fnm（Fast Node Manager），多版本 Node.js 切换，自动写入 PowerShell profile，无需手动配置。"
head:
  - - meta
    - name: keywords
      content: "fnm 安装, fnm Windows, Node.js 版本管理, nvm 替代, fast node manager, hudo"
---

# fnm

[fnm](https://github.com/Schniz/fnm) 是一个用 Rust 写的快速 Node.js 版本管理器，适合需要同时维护多个 Node.js 版本的项目（如老项目用 16、新项目用 22）。相比 nvm-windows 启动更快、切换更丝滑。

hudo 同时提供 [Node.js 直接安装](./nodejs) 和 fnm 两种方式，二者相互独立，可按需选择或并存：

- **只跑一个项目** → 用 [Node.js](./nodejs)，最省心。
- **要切换多个 Node 版本** → 用 fnm。

## 安装

```powershell
hudo install fnm
```

hudo 会：

1. 下载 fnm 可执行文件到 `{install_root}\tools\fnm\`
2. 将 `FNM_DIR` 指向 `{install_root}\lang\node-fnm\`（与纯 Node.js 的 `lang\node\` 分开，互不干扰）
3. 用 `fnm install --lts` 安装最新 LTS，并设为默认
4. 自动把 fnm 初始化脚本写入 PowerShell `$PROFILE`

## 安装后

重新打开终端：

```powershell
node --version
fnm list
fnm use 20
```

## 卸载

```powershell
hudo uninstall fnm
```

## 配置

指定 fnm 版本：

```toml
[versions]
fnm = "1.38.1"
```

自定义下载镜像：

```toml
[mirrors]
fnm = "https://github.com/Schniz/fnm/releases/download/v1.38.1"
```

## 与纯 Node.js 共存

fnm 管理的 Node.js 安装在 `lang\node-fnm\`，hudo 直接安装的 Node.js 在 `lang\node\`。两者的 PATH 项与环境变量完全独立，可同时存在。

如果你两个都装了，终端实际用哪个 `node`，取决于 PATH 顺序与 PowerShell profile 中 `fnm env` 的执行时机——profile 执行后，fnm 的 shim 会优先生效。

## 常见问题

**Q: 装了 fnm 后 `node` 命令找不到？**

fnm 依赖 PowerShell profile 中的 `fnm env` 初始化 PATH。请确认新开的是 PowerShell（不是 CMD），且 `$PROFILE` 文件已包含 fnm 初始化行。

**Q: 和旧版 hudo 装的 fnm 冲突吗？**

旧版 hudo (v0.1.x) 的 fnm 把 Node.js 装在 `lang\node\`，新版单独装在 `lang\node-fnm\`。如果旧环境仍在用，建议先 `hudo uninstall fnm` 清理后再通过新版重装。

**Q: CMD 里 fnm 能用吗？**

可以。`fnm env --shell cmd` 可以生成 CMD 用的初始化脚本，但 hudo 默认只写入 PowerShell profile，CMD 需手动配置。
