---
date: 2026-07-24
author: Zexa
title: .NET SDK 安装教程：环境变量、多版本共存与 global.json 一次讲清
description: Windows 下 .NET SDK 安装完整教程：对比官方安装器、winget 与 hudo 用户级免管理员安装三种方式，讲清 DOTNET_ROOT 与 PATH 两个环境变量的分工、多个 SDK 版本共存的原理，以及用 global.json 的 version 与 rollForward 字段锁定项目 SDK 版本的写法，附 STS 与 LTS 选择建议。
keywords:
  - .NET SDK 安装教程
  - .NET SDK 多版本共存
  - dotnet 环境变量
  - DOTNET_ROOT
  - global.json
  - rollForward
  - dotnet-install.ps1
  - Windows .NET 开发环境
  - .NET LTS
  - hudo
---

# .NET SDK 安装教程：环境变量、多版本共存与 global.json 一次讲清

::: tip TL;DR
开发机只需装 SDK（已含 Runtime），不用单独装 Runtime。多个 SDK 版本天生可以并存，项目里放 `global.json` 即可锁定版本。Windows 上免管理员的装法是一条命令：`hudo install dotnet`——用户级安装最新 LTS，自动配好 `DOTNET_ROOT` 和 PATH。
:::

## SDK 和 Runtime 到底差在哪？

一句话：**SDK 能编译也能运行，Runtime 只能运行**。SDK 包含 C#/F# 编译器、`dotnet build` / `dotnet run` 等命令行工具和完整运行时；Runtime 只有执行已编译程序所需的部分。所以开发机装 SDK 就够了，Runtime 已经包含在内；只有部署服务器这种"只跑程序"的机器才值得单独装 Runtime。

## Windows 上装 .NET SDK 有哪几种方式？

三种：官方安装器、winget、用户级脚本安装（hudo 把这条路自动化了）。核心区别在于**要不要管理员权限**和**装到哪个目录**。

### 官方安装器为什么需要管理员权限？

因为它是机器级安装——写入 `C:\Program Files\dotnet` 并修改系统级环境变量，这两个动作都需要 UAC 提权。从 [dotnet.microsoft.com/download](https://dotnet.microsoft.com/download) 下载 exe，双击、提权、下一步即可，版本以官网最新为准。优点是所有用户共享一份；缺点是公司管控的电脑没有管理员权限就装不了。

### winget 一条命令能装吗？

能，但本质仍是机器级安装，执行时会弹 UAC：

```powershell
winget install Microsoft.DotNet.SDK.10
```

包名尾号就是大版本号，要装 .NET 8 就把 `10` 换成 `8`。winget 只是替你下载并静默运行官方安装器，权限要求没有变化。

### 怎么免管理员做用户级安装？

用微软官方的 `dotnet-install.ps1` 脚本，它专为用户级和自动化场景设计，全程不需要提权。手动做法：

```powershell
irm https://dot.net/v1/dotnet-install.ps1 -OutFile dotnet-install.ps1
.\dotnet-install.ps1 -Channel LTS -InstallDir D:\dotnet
```

但脚本只负责把文件解压到位，`DOTNET_ROOT` 和 PATH 得自己写——这是最容易漏的一步（原因见下节）。

[hudo](/tools/dotnet) 把整条链路压缩成一条命令：

```powershell
hudo install dotnet
```

它调用官方 `dotnet-install.ps1` 做用户级安装，装到 `{安装盘}:\hudo\lang\dotnet\`，自动写入 `DOTNET_ROOT` 和 PATH（用户级环境变量，写 HKCU，免管理员），版本默认跟随最新活跃 LTS。hudo 是 Windows 开发环境引导工具，同样的方式还能装 Git、Node.js、JDK 等共 27 款工具。

## DOTNET_ROOT 和 PATH 各自管什么？

**PATH 决定你在终端敲 `dotnet` 时启动哪个可执行文件；`DOTNET_ROOT` 告诉已编译好的 .NET 程序去哪找运行时。** 两者缺一不可：

- 只配 PATH 不配 `DOTNET_ROOT`：`dotnet run` 正常，但直接运行编译产物 exe（apphost）或 `dotnet tool install -g` 装的全局工具时，程序会按默认路径 `C:\Program Files\dotnet` 找运行时，找不到就报 **"You must install .NET to run this application"**。
- 只配 `DOTNET_ROOT` 不配 PATH：终端里直接提示 `dotnet` 不是内部或外部命令。

机器级安装装在默认路径，所以感知不到 `DOTNET_ROOT` 的存在；一旦走用户级安装（自定义目录），两个变量必须同时指向安装目录。很多人手动跑完 `dotnet-install.ps1` 后"装是装上了、工具却跑不起来"，根因就在这里。

## 多个 .NET SDK 版本怎么共存？

**SDK 天生支持并存**：所有版本都装在 `DOTNET_ROOT` 下的 `sdk\` 子目录里，一个版本一个文件夹，互不覆盖。`dotnet` 命令本身是个"选版器"（muxer），每次执行时按规则挑一个 SDK 出来干活。

查看装了哪些版本：

```powershell
dotnet --list-sdks
# 8.0.412 [D:\hudo\lang\dotnet\sdk]
# 10.0.302 [D:\hudo\lang\dotnet\sdk]
```

没有任何配置时默认用**最新**的那个；想让某个项目固定用旧版，就在项目里放 `global.json`。

### global.json 怎么锁定 SDK 版本？

在仓库根目录放一个 `global.json`，`dotnet` 会从当前目录逐级向上查找，找到即生效：

```json
{
  "sdk": {
    "version": "10.0.302",
    "rollForward": "latestFeature"
  }
}
```

两个字段的含义：

- **version**：基准版本。注意 SDK 版本号第三段是"特性波段 + 补丁"——`10.0.302` 表示 3xx 波段的第 02 个补丁。
- **rollForward**：机器上没有精确匹配的版本时怎么办。常用值：
  - `latestPatch`（不写时的默认值）：只接受同波段内更高补丁，`10.0.305` 可以，`10.0.400` 不行
  - `latestFeature`：放宽到更高波段，`10.0.400` 也接受
  - `latestMinor` / `latestMajor`：逐级放宽到次版本 / 主版本
  - `disable`：一个字都不能差，没有就报错

踩坑提示：`version` 写死加默认的 `latestPatch`，在没装对应波段 SDK 的机器（尤其 CI）上会直接报 "A compatible .NET SDK was not found"。团队协作建议 `version` 写波段起点（如 `10.0.100`）配 `latestFeature`，既锁住大版本又不至于动不动罢工。在项目目录跑 `dotnet --version`，输出的就是 global.json 选中的版本，以此验证是否生效。

最后区分一下层次：`global.json` 锁的是"这个项目用哪个 SDK"，hudo [配置文件](/guide/config)里 `[versions]` 段的 `dotnet = "10.0.302"` 锁的是"这台机器装哪个 SDK"，两者配合就能做到团队环境可复现。

## 常见问题

### 只跑程序不开发，要装 SDK 还是 Runtime？

装 Runtime 就够。SDK 是给写代码的机器用的；部署机或只运行别人发布的程序，装对应版本的 Runtime（Web 应用则装 ASP.NET Core Runtime）即可，体积小得多。

### STS 和 LTS 选哪个？

没有特殊理由就选 LTS。.NET 每年 11 月发一个大版本，偶数号是 LTS（支持 3 年），奇数号是 STS（只支持 18 个月）——比如 STS 的 .NET 9 已在 2026 年 5 月停止支持。hudo 默认只跟最新活跃 LTS，不会给你装一个一年半后就没有安全更新的版本。

### hudo 装的 .NET 会和系统里已有的冲突吗？

不会重复安装。hudo 检测到机器级 .NET（`C:\Program Files\dotnet`）时不会再装一份，可以选择让 hudo 接管。真正要留意的是 PATH 中两个目录的先后顺序——它决定终端里的 `dotnet` 用哪一份。

### 装完提示找不到 dotnet 命令怎么办？

先开一个**新终端**再试——Windows 的环境变量修改只对新开的进程生效，旧终端读不到。新终端仍不行，就在 PowerShell 里执行 `[Environment]::GetEnvironmentVariable('Path','User')` 检查用户 PATH 是否包含安装目录，缺了就补上。

## 相关阅读

- [.NET SDK 工具页：hudo 安装命令、目录约定与版本锁定配置参考](/tools/dotnet)
- [新电脑用 hudo 20 分钟配好完整 Windows 开发环境（27 款工具一键装）](/blog/windows-dev-environment-setup)
