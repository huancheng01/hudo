---
date: 2026-07-24
author: Zexa
title: PowerShell 7 是干嘛的？要不要升级？与 Windows PowerShell 5.1 的区别和共存关系
description: PowerShell 7 和系统自带的 5.1 是两个程序，可以共存。对照讲清区别（&&、三元、并行、跨平台）、谁该升级，附 winget、MSI、hudo 三种安装方式。
keywords:
  - PowerShell 7 是干嘛用的
  - PowerShell 7 和 5.1 区别
  - PowerShell 要不要升级
  - pwsh
  - Windows PowerShell 5.1
  - PowerShell 7 安装
  - winget 安装 PowerShell
  - PowerShell 共存
  - Windows Terminal 默认终端
  - oh-my-posh
  - hudo
---

# PowerShell 7 是干嘛的？要不要升级？与 Windows PowerShell 5.1 的区别和共存关系

::: tip TL;DR
PowerShell 7（`pwsh`）是微软目前主力维护的现代版 PowerShell，与 Windows 自带的 PowerShell 5.1（`powershell`）是两个独立程序，装上后并存、互不覆盖。写脚本、用 oh-my-posh 美化终端、做跨平台开发的人建议装；只偶尔敲一两条命令的可以不装。免管理员一键安装：`hudo install pwsh`。
:::

## PowerShell 7 是干嘛用的？和系统自带的是什么关系？

PowerShell 7 是微软当前主力开发的跨平台 shell 与脚本语言，可执行文件叫 `pwsh.exe`；它和 Windows 自带的 Windows PowerShell 5.1（`powershell.exe`）是两个独立程序，安装后二者并存，互不影响。

背景是这样的：Windows 10/11 预装的 Windows PowerShell 5.1 发布于 2016 年，是基于 .NET Framework 的最后一个大版本，此后微软不再为它开发新功能，只做安全维护。开发重心转移到了开源、跨平台的新实现上——先叫 PowerShell Core 6，从 7.0 起去掉 Core 后缀，就是今天的 PowerShell 7，基于现代 .NET 运行时，Windows、Linux、macOS 都能跑。

因为两者可执行文件名不同（`powershell` vs `pwsh`）、安装目录也不同（前者在 System32 里，后者装到 Program Files 或自选目录），装 PowerShell 7 并不会"升级掉"5.1：在终端敲 `powershell` 进的还是 5.1，敲 `pwsh` 进的才是 7。所以"要不要升级"其实是个伪问题——这不是替换升级，是多装一个更好用的，旧的原地不动。

## PowerShell 7 和 5.1 到底差在哪？

一句话：5.1 停在 2016 年的语法和 .NET Framework 上，PowerShell 7 拿到了这十年间的全部新特性。对写命令和脚本的人来说，差距集中在这几处：

| 对比项 | Windows PowerShell 5.1 | PowerShell 7 |
| --- | --- | --- |
| 可执行文件 | `powershell.exe` | `pwsh.exe` |
| 运行时 | .NET Framework 4.x | 现代 .NET（8+） |
| 更新状态 | 仅安全维护，无新功能 | 活跃开发 |
| `&&` / `\|\|` 链式操作符 | 不支持（解析报错） | 7.0 起支持 |
| 三元运算符 `? :` | 不支持 | 7.0 起支持 |
| null 合并 `??` | 不支持 | 7.0 起支持 |
| `ForEach-Object -Parallel` 并行 | 不支持 | 7.0 起支持 |
| 重定向写文件默认编码 | UTF-16 / ANSI 混杂 | 统一 UTF-8（无 BOM） |
| 平台 | 仅 Windows | Windows / Linux / macOS |
| 获取方式 | 系统预装 | 需自行安装 |

表格之外，两个最容易在日常撞上的点值得展开：

第一是 `&&`。几乎所有开源项目的 README 都写 `npm install && npm run dev` 这类命令，粘进 5.1 会直接报"标记 '&&' 不是此版本中的有效语句分隔符"——不是你的操作有问题，是 5.1 真的不认识它，这个操作符 7.0 才加上。

第二是编码。5.1 里 `>` 重定向默认写出 UTF-16LE，`Set-Content` 默认 ANSI（中文系统就是 GBK），同一段脚本写出的文件编码互相都不一致，交给 Git 或别的工具处理就是乱码现场；PowerShell 7 把默认编码统一成了无 BOM 的 UTF-8，跟现代工具链对齐。

## 要不要升级？谁需要，谁无所谓？

判断标准很简单：你会不会把终端当工具用。会，就装；不会，5.1 留着当应急壳子就够了。

**建议装的人：**

- **写 PowerShell 脚本的人**——`&&`、三元、`??` 这些新语法加上并行 ForEach，脚本量大时并行一项就值回票价；而且 5.1 除了安全补丁不会再有任何改进
- **想美化终端的人**——oh-my-posh 等主流终端美化教程默认你用的是 PowerShell 7，在 5.1 上照做容易卡在半路，具体见 [Windows Terminal 美化教程](/blog/windows-terminal-beautify)
- **常从 README 粘命令、常用现代 CLI 工具的人**——`&&` 报错会天天烦你
- **跨平台开发者**——同一份脚本在 Windows / Linux / macOS 上通用

**无所谓的人：**

- 一年打开终端的次数一只手数得过来，只跑 `ipconfig`、`ping` 这类命令
- 生产脚本依赖某些只在 5.1 下验证过的 Windows 专属管理模块的运维场景（7 有兼容层，但关键脚本谨慎为先）

装了也不亏：两者共存，代价只是一点磁盘空间。

## 怎么安装 PowerShell 7？三种方式选哪个？

结论：想省事用 winget，要离线包或更新选项用官方 MSI，账户没有管理员权限就用 hudo 便携版。版本号以官网最新为准。

### 方式一：winget 一条命令

Windows 10/11 自带 winget，一条命令搞定：

```powershell
winget install --id Microsoft.PowerShell --source winget
```

默认机器级安装到 `C:\Program Files\PowerShell\7\`，过程中会弹一次 UAC 提权确认。以后升级用 `winget upgrade Microsoft.PowerShell`。

### 方式二：官方 MSI 安装包

适合离线环境或想控制安装选项的人。到 GitHub 的 [PowerShell Releases](https://github.com/PowerShell/PowerShell/releases) 页面下载 `PowerShell-7.x.x-win-x64.msi`，双击安装。安装向导里可以勾选"通过 Microsoft Update 获取更新"，勾上之后版本就跟着系统更新走，不用手动维护。同样需要管理员权限。

### 方式三：hudo 便携版（免管理员）

如果你的账户没有管理员权限，或者不想在注册表里留痕迹，可以用 [hudo](https://hudo.zexa.cc)——一个支持 26 款工具一键安装的 Windows 开发环境引导工具：

```powershell
# 先装 hudo（如果还没有）
irm hudo.zexa.cc/install.ps1 | iex

# 装 PowerShell 7
hudo install pwsh
```

hudo 用的是官方 zip 便携版，装到 `{安装盘}\hudo\tools\pwsh\`，全程用户态、不弹 UAC，自动把路径写进用户 PATH；卸载时 `hudo uninstall pwsh` 删目录加清 PATH，不碰注册表。需要锁定版本可以在配置文件里写 `pwsh = "7.6.4"`。完整参数见 [PowerShell 7 工具页](/tools/pwsh)。

装完后**新开一个终端**验证：

```powershell
pwsh -Version
```

## 怎么把 Windows Terminal 的默认终端换成 PowerShell 7？

路径是：Windows Terminal 设置（`Ctrl+,`）→ 启动 → 默认配置文件 → 选 "PowerShell"。注意下拉列表里会同时出现 "Windows PowerShell"（这是 5.1）和 "PowerShell"（这才是 7），名字很像，别选混了。

winget 和 MSI 装的版本会自动向 Windows Terminal 注册配置文件，装完重开 Terminal 就能在列表里看到。

hudo 装的便携版不向系统注册配置文件，需要手动加一次：设置 → 添加新配置文件 → 新建空配置文件，"命令行"一栏填 `pwsh.exe` 的完整路径（例如 `D:\hudo\tools\pwsh\pwsh.exe`），名称填 "PowerShell 7"，保存后再回到"启动"里把它设为默认。之后每个新标签页打开的就都是 PowerShell 7 了。

## 常见问题

### 安装 PowerShell 7 后需要卸载 Windows PowerShell 5.1 吗？

不需要，也卸不掉。5.1 是 Windows 的系统组件，没有常规卸载入口；它和 PowerShell 7 各占一个目录、各用一个可执行文件名，共存没有任何冲突。

### 老的 .ps1 脚本在 PowerShell 7 里还能跑吗？

绝大部分能直接跑。少数依赖 Windows 专属模块的脚本在 7 里需要走兼容层（`Import-Module -UseWindowsPowerShell`），个别行为有差异。实在不行，那个脚本继续用 `powershell.exe` 跑就是——两边本来就共存。

### 怎么确认我当前用的是 5.1 还是 7？

执行 `$PSVersionTable.PSVersion`，输出 5.1.x 就是 Windows PowerShell，7.x 就是 PowerShell 7。窗口标题也能区分："Windows PowerShell" 是 5.1，"PowerShell" 是 7。

### PowerShell 7 会自动更新吗？

看安装方式：winget 装的用 `winget upgrade` 手动升级；MSI 装的如果勾了 Microsoft Update 选项，会随系统更新；hudo 便携版不会自动更新，需要时重新执行 `hudo install pwsh` 获取新版（不锁版本时装的就是当时的最新版）。

## 相关阅读

- [PowerShell 7 工具页：hudo 安装参数、版本锁定与卸载说明](/tools/pwsh)
- [Windows Terminal 终端美化教程：配合 PowerShell 7 配置 oh-my-posh 与字体主题](/blog/windows-terminal-beautify)
- [20 分钟配好完整 Windows 开发环境：hudo 全套工具一键安装实录](/blog/windows-dev-environment-setup)
