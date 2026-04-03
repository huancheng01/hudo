---
title: "Go (Golang) Windows 安装与配置 - hudo"
description: "使用 hudo 在 Windows 上一键安装 Go 语言，自动配置 GOROOT、GOPATH 和 PATH 环境变量，支持国内 GOPROXY 镜像加速。"
head:
  - - meta
    - name: keywords
      content: "Go 安装, Golang Windows, Go 环境变量, GOPATH 配置, GOPROXY 镜像, 一键安装 Go, hudo"
---

# Go

Go（Golang）是 Google 开发的静态编译型语言，以简洁的语法、出色的并发支持和极快的编译速度著称。它在云原生、微服务、网络编程和 DevOps 工具领域被广泛采用。

## 安装

```powershell
hudo install go
```

安装到 `{install_root}\lang\go\`，自动获取最新版本，自动设置 `GOPATH` 到 `{install_root}\lang\gopath\`。

## 安装后

```powershell
go version
go env GOPATH
```

## 卸载

```powershell
hudo uninstall go
```

## 配置文件版本

```toml
[versions]
go = "1.23.0"
```

## hudo 安装优势

- **自动配置 GOROOT 和 GOPATH**：无需手动设置环境变量，安装后 `go` 命令和模块系统立即可用
- **GOPATH 独立管理**：GOPATH 自动设置到 `{install_root}\lang\gopath\`，与 Go 安装目录分离，结构清晰
- **自动获取最新版本**：通过 Go 官方 API 获取最新稳定版，也支持在[配置文件](/guide/config)中锁定特定版本
- **干净卸载**：`hudo uninstall go` 一键清除 Go 工具链和相关环境变量配置

## 常见问题

**Q: 安装后 `go` 命令找不到怎么办？**

打开新的终端窗口即可，hudo 已自动配置环境变量，需要新终端加载。

**Q: `go get` 下载依赖很慢怎么办？**

建议配置 GOPROXY 国内代理：`go env -w GOPROXY=https://goproxy.cn,direct`。这将使用国内镜像加速 Go 模块下载。

**Q: hudo 安装的 Go 和官方 MSI 安装包冲突吗？**

如果系统已通过 MSI 安装包安装了 Go，建议先卸载旧版本再使用 hudo 安装，避免多个 GOROOT 导致的环境变量冲突。

## 相关阅读

- [Windows Go 开发环境配置避坑指南：GOPATH、模块代理一文搞定](/blog/windows-go-install) — 详细安装教程与常见问题解答
