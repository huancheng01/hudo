---
date: 2026-07-24
author: Zexa
title: "Windows 终端美化保姆级教程（2026）：PowerShell 7 + Oh My Posh + Nerd Font 一步到位"
description: Windows 终端美化完整教程：从 PowerShell 7、Oh My Posh 到 Nerd Font 字体，讲清手动安装的五个步骤和三个最常见的坑（字体没装、profile 没写、终端字体没切换），并给出 hudo 两条命令的一键方案——装引擎、装 36 个字体文件、写 profile 全自动完成，免管理员权限，适用于 Windows Terminal 与 VS Code。
keywords:
  - Windows 终端美化
  - Oh My Posh 安装
  - Windows Terminal 美化教程
  - PowerShell 7
  - Nerd Font
  - CaskaydiaCove
  - PowerShell profile
  - 终端主题
  - pwsh
  - hudo
---

# Windows 终端美化保姆级教程（2026）：PowerShell 7 + Oh My Posh + Nerd Font 一步到位

::: tip TL;DR
Windows 终端美化 = PowerShell 7（现代 shell）+ Oh My Posh（提示符引擎）+ Nerd Font（图标字体），三件缺一不可，网上大量教程失效就是因为只讲了其中一两件。手动装要五步、有三个经典坑；用 hudo 两条命令可以全自动完成：`hudo install pwsh` 装 PowerShell 7，`hudo install omp` 装引擎 + 36 个字体文件 + 写 profile。装完只剩一步手动操作：在终端设置里把字体切成 `CaskaydiaCove Nerd Font`。
:::

## 为什么终端美化需要「三件套」？

因为一个漂亮的终端提示符要同时满足三个条件：有现代 shell 承载、有引擎生成内容、有字体渲染图标——对应 PowerShell 7、Oh My Posh 和 Nerd Font，三者各管一段，谁缺了都会以不同的方式坏掉。

- **PowerShell 7 是前置**。Windows 自带的 Windows PowerShell 5.1 从 2016 年起就停止了大版本更新，而几乎所有终端美化教程（包括 Windows Terminal 官方教程）都默认你在用 [PowerShell 7](/tools/pwsh)（`pwsh`）。它性能更好，语法也补齐了 `&&`/`||`、三元运算符、并行 ForEach 等现代特性，详见[这篇 PowerShell 7 完整指南](/blog/windows-powershell7-guide)。
- **Oh My Posh 是引擎**。它是跨 shell 的提示符主题引擎，负责在每次回车后画出那行带 git 分支、路径、语言版本的彩色提示符。
- **Nerd Font 是渲染层**。提示符里的分支图标、箭头、徽章都是 Nerd Font 打进字体的私有区字符（glyph），终端字体不是 Nerd Font 时这些字符统统显示为方块。

理解了分工，后面每个坑的成因就一目了然了。

## 手动安装的完整步骤是什么？

手动路线共五步：装 PowerShell 7 → 装 Oh My Posh → 装 Nerd Font → 写 profile → 切终端字体。每一步都真实可操作，按顺序来即可。

### 第一步：安装 PowerShell 7

```powershell
winget install Microsoft.PowerShell
```

或者去 GitHub Releases 下载 MSI 安装包（需要管理员权限）。装完在新终端输入 `pwsh` 确认能进入，版本以官网最新为准（写作时为 7.x）。

### 第二步：安装 Oh My Posh

```powershell
winget install JanDeDobbeleer.OhMyPosh -s winget
```

winget 安装会同时配置 PATH 和 `POSH_THEMES_PATH` 环境变量。装完**新开一个终端**再验证 `oh-my-posh version`，旧窗口读不到新写入的环境变量。

### 第三步：安装 Nerd Font 字体

去 [nerdfonts.com](https://www.nerdfonts.com/) 下载 CascadiaCode 压缩包（也就是 CaskaydiaCove——微软 Cascadia Code 的 Nerd Font 版本），解压后全选 `.ttf` 文件，右键选「安装」。注意选「安装」而不是「为所有用户安装」：前者是用户级安装，Win10 1809 以上免管理员权限。

### 第四步：把初始化命令写进 PowerShell profile

```powershell
notepad $PROFILE
```

文件不存在就让记事本新建，然后写入这一行并保存：

```powershell
oh-my-posh init pwsh --config "$env:POSH_THEMES_PATH\jandedobbeleer.omp.json" | Invoke-Expression
```

如果重启终端后报错「禁止运行脚本」，是执行策略拦住了 profile，放开即可：

```powershell
Set-ExecutionPolicy RemoteSigned -Scope CurrentUser
```

### 第五步：把终端字体切换成 Nerd Font

Windows Terminal：设置 → 你的配置文件 → 外观 → 字体 → 选 `CaskaydiaCove Nerd Font`。这一步没有任何工具能替你做，装完字体必须手动切换。

## 手动安装最容易踩的三个坑是什么？

三个坑分别对应三件套里「字体、profile、终端设置」三个环节的缺失，症状不同，很好对号入座。

### 坑一：装的根本不是 Nerd Font

症状是图标位置全是方块或问号。很多人装的是普通 Cascadia Code 或微软雅黑，它们没有图标字符。必须是名字里带「Nerd Font」的版本，排查细节见[这篇终端图标乱码修复指南](/blog/windows-nerd-font-fix)。

### 坑二：profile 没写，或写错了文件

症状是新开终端后提示符毫无变化。除了忘写之外还有一个隐蔽变体：PowerShell 5.1 和 7 的 `$PROFILE` 是**两个不同的文件**（分别在文档目录的 `WindowsPowerShell\` 和 `PowerShell\` 下），在 5.1 里写的 profile 对 pwsh 完全不生效，反之亦然。文档目录被 OneDrive 重定向的机器还会再偏一层路径。

### 坑三：字体装上了，但终端还在用旧字体

症状是提示符出来了、颜色也对，唯独图标是方块。字体安装进系统只是第一半，Windows Terminal、VS Code 各自的字体设置才决定实际渲染用哪个，第五步漏掉就是这个结果。

## 如何用 hudo 两条命令一步到位？

先装 [hudo](https://hudo.zexa.cc)（一个支持 26 款工具的 Windows 开发环境引导工具），再执行两条安装命令，上面五步里的四步就全自动完成了。

```powershell
irm hudo.zexa.cc/install.ps1 | iex
hudo install pwsh
hudo install omp
```

`hudo install pwsh` 安装 PowerShell 7 的 zip 便携版，全程免管理员、自动配 PATH，与系统自带的 5.1 共存互不影响。`hudo install omp` 则一次完成：

1. 安装 oh-my-posh 主程序与官方主题包
2. 用户级安装并注册 CaskaydiaCove Nerd Font 的全部 **36 个字体文件**（免管理员）
3. 配置 PATH 与 `POSH_THEMES_PATH`
4. 询问确认后写入 PowerShell profile——**5.1 和 pwsh 7 两份都写**，幂等且带标记，正好绕开上面的坑二

唯一剩下的手动操作就是坑三那步：在终端设置里把字体切换为 `CaskaydiaCove Nerd Font`。不想要了，`hudo uninstall omp` 会逆向清理 profile 注入行、36 个字体文件与注册表项、环境变量和安装目录，不留残余。安装参数与版本锁定等细节见 [pwsh 工具页](/tools/pwsh)和 [omp 工具页](/tools/omp)。

## 装好之后怎么换主题？

列出 `$env:POSH_THEMES_PATH` 目录下的主题文件，把 profile 里 `--config` 指向的文件名换掉、重开终端即可。

```powershell
Get-ChildItem $env:POSH_THEMES_PATH
```

官方主题包有上百个主题，可以先在 [ohmyposh.dev 的主题画廊](https://ohmyposh.dev/docs/themes)看效果图再挑名字，省去逐个试的时间。改完 profile 不需要重装任何东西，主题只是一个 JSON 配置文件。

## 常见问题

### Oh My Posh 只能在 PowerShell 7 上用吗？

不是，Windows PowerShell 5.1 上同样能跑，hudo 也会把两个版本的 profile 都写好。但推荐配合 PowerShell 7 使用：5.1 已停止大版本更新，且各类主题、教程都以 pwsh 为默认环境测试。

### 图标显示成方块或问号怎么办？

九成是终端字体没切到 Nerd Font：Windows Terminal 在设置 → 外观 → 字体里选 `CaskaydiaCove Nerd Font`；VS Code 把 `terminal.integrated.fontFamily` 设为同名即可。仍不行就检查字体是否真的装上了，见[乱码修复指南](/blog/windows-nerd-font-fix)。

### 美化会拖慢终端启动吗？

影响很小。oh-my-posh 是编译型二进制，提示符渲染在毫秒级；如果明显变慢，多半是 profile 里其他模块（如 conda 初始化）拖的后腿，可逐行注释排查。

### 便携版 pwsh 为什么不出现在 Windows Terminal 下拉菜单里？

便携版不向系统注册 Terminal 配置文件。在 Windows Terminal 设置里手动新建一个配置文件，命令行填 pwsh.exe 的完整路径即可，具体见 [pwsh 工具页](/tools/pwsh)。

### 想用 CaskaydiaCove 之外的字体可以吗？

可以，任何 Nerd Font 都行。从 nerdfonts.com 下载后按本文第三步安装，再把终端字体切换过去；hudo 默认选 CaskaydiaCove 是因为它与 Windows 终端的观感最协调。

## 相关阅读

- [Oh My Posh 工具页：安装参数、版本锁定与卸载清理](/tools/omp)
- [PowerShell 7 工具页：便携版安装与 5.1 共存说明](/tools/pwsh)
- [终端图标乱码变方块？Nerd Font 问题排查与修复](/blog/windows-nerd-font-fix)
- [PowerShell 7 完整指南：与 5.1 的区别及迁移建议](/blog/windows-powershell7-guide)
