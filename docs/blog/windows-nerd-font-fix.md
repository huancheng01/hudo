---
date: 2026-07-24
author: Zexa
title: 终端图标乱码、显示方框怎么办？Nerd Font 从装字体到终端关联彻底解决
description: 终端图标乱码、显示方框的三层排查：Nerd Font 没装、终端没关联字体、旧版本冲突。附 Windows Terminal 与 VS Code 的字体设置路径和一键安装方案。
keywords:
  - oh-my-posh 乱码
  - 终端图标方框
  - Nerd Font 安装
  - CaskaydiaCove Nerd Font
  - Windows Terminal 字体设置
  - VS Code 终端字体
  - terminal.integrated.fontFamily
  - PowerShell 终端美化
  - Nerd Font v3
  - hudo
---

# 终端图标乱码、显示方框怎么办？Nerd Font 从装字体到终端关联彻底解决

::: tip TL;DR
终端提示符里的图标变成方框或乱码，绝大多数只有两个原因：Nerd Font 字体没装，或者终端没把字体切换成 Nerd Font——两步缺一不可。运行 `hudo install omp` 可以免管理员自动装好 oh-my-posh 和全部 36 个 CaskaydiaCove Nerd Font 字体文件，之后只剩一步：在终端设置里把字体选为 `CaskaydiaCove Nerd Font`。
:::

## 为什么终端图标会变成方框？

先看症状长什么样——下面两张图是**同一个提示符**，唯一区别是终端字体（上：普通字体，图标全是方框；下：CaskaydiaCove Nerd Font，图标正常）：

![终端图标显示为方框：字体不含 Nerd Font 图标字符的典型症状](/img/blog/terminal-omp-boxes.png)

![切换 Nerd Font 后图标正常显示：文件夹、git 分支、领先提交数](/img/blog/terminal-omp-nerd.png)

因为主题用到的图标字符根本不在你当前字体的字符集里。oh-my-posh、starship 这类提示符主题大量使用 Unicode **私有使用区（PUA）** 码位的字形——比如分支箭头是 U+E0B0，Git、Python 的图标来自 Font Awesome、Devicons、Powerline 等图标集，合计上万个字形。Consolas、微软雅黑这些常规字体没有这些码位，终端只能画一个空心方框（俗称 tofu）兜底。Nerd Font 就是把这些图标字形补进流行等宽字体后的"补丁版"，CaskaydiaCove 即微软 Cascadia Code 的 Nerd Font 版本。

先做一个 10 秒自测，在 PowerShell 里执行：

```powershell
"$([char]0xE0B0)"
```

能看到一个实心三角箭头，说明当前终端已经在用 Nerd Font，问题出在别处；看到方框，就按下面三层逐一排查。

## 第一层：Nerd Font 到底装上了没有？

### 怎么确认字体已经安装？

打开 **设置 → 个性化 → 字体**，在搜索框输入 `CaskaydiaCove`，能搜到条目即已安装。也可以直接看两个目录：用户级字体在 `%LOCALAPPDATA%\Microsoft\Windows\Fonts`，系统级字体在 `C:\Windows\Fonts`。两处都搜不到，就是没装，先补这一步。

### 手动安装 Nerd Font 怎么做才不踩坑？

从 [nerdfonts.com](https://www.nerdfonts.com/font-downloads) 下载 `CascadiaCode.zip`，解压后**全选所有 `.ttf` 文件**，右键 → 安装（当前用户，免管理员）或"为所有用户安装"（需要管理员）。Windows 10 1809 起支持用户级字体安装，文件落在 `%LOCALAPPDATA%` 下、注册表写在 HKCU，公司电脑没有管理员权限也能装。

手动装最常见的坑是**只装了一两个文件**：一套 Nerd Font 包含 Regular、Bold、Italic 及 Mono、Propo 变体共几十个文件，只装 Regular 会导致终端里的粗体、斜体回退到其他字体，观感割裂。另外 oh-my-posh 官方 CLI 也自带 `oh-my-posh font install` 交互式装字体，可作备选。

### 有没有一条命令自动装齐的办法？

有，`hudo install omp` 会把引擎和字体一次装完。先装 hudo（如果还没有）：

```powershell
irm hudo.zexa.cc/install.ps1 | iex
```

再执行：

```powershell
hudo install omp
```

hudo 会安装 oh-my-posh 主程序与官方主题包，**用户级安装并注册全部 36 个 CaskaydiaCove Nerd Font 字体文件**（免管理员，Win10 1809+），配置 PATH 与 `POSH_THEMES_PATH`，并在询问后写入 PowerShell profile（5.1 与 pwsh 7 都会写，重复执行幂等）。手动流程里"字体装不全、profile 忘了写"这两个高频漏项都被自动化掉了。不需要时 `hudo uninstall omp` 会逆向清掉 36 个字体文件、注册表项和 profile 注入行，不留残留。完整参数与换主题方法见 [oh-my-posh 工具页](/tools/omp)。

注意：**无论手动装还是 hudo 装，字体装好只完成了一半**——终端不会自动改用新字体，这就是第二层。

## 第二层：终端把字体关联上了吗？

字体在系统里躺着不等于终端在用它，每个终端程序有各自独立的字体设置，这是整个问题里最容易漏掉的半步。

### Windows Terminal 在哪里设置字体？

按 `Ctrl+,` 打开设置 → 左侧选中你在用的配置文件（如 PowerShell）→ **外观 → 字体**，下拉选择 `CaskaydiaCove Nerd Font`，保存即可，当前窗口立即生效。想一次对所有配置文件生效，就在"默认值"里改。等价的 `settings.json` 写法：

```json
"profiles": {
  "defaults": {
    "font": { "face": "CaskaydiaCove Nerd Font" }
  }
}
```

### VS Code 集成终端在哪里设置字体？

按 `Ctrl+,` 搜索 `terminal.integrated.fontFamily`，填入 `CaskaydiaCove Nerd Font`。对应 `settings.json`：

```json
"terminal.integrated.fontFamily": "CaskaydiaCove Nerd Font"
```

注意这个设置只管集成终端，编辑器字体由 `editor.fontFamily` 控制，两者互不影响；字体名要与系统里注册的家族名**完全一致**（含空格），拼错会静默回退到默认字体。

### 老式控制台（conhost）能用吗？

能，但入口不同：标题栏右键 → 属性 → 字体，且只认严格等宽的变体，列表里选 `CaskaydiaCove Nerd Font Mono` 这类带 Mono 后缀的。

## 第三层：是不是旧版字体或同名冲突在捣乱？

字体装了、终端也选了，却仍有**个别**图标是方框——这几乎都是 Nerd Fonts 版本太旧或新旧共存导致的。

### 为什么旧版 Nerd Font 只有部分图标碎？

2023 年发布的 Nerd Fonts v3 把 Material Design Icons 从 U+F500–U+FD46 迁移到了 U+F0001 起的扩展平面，新主题按 v3 码位引用图标，v2 字体自然渲染不出，症状就是"大部分正常、少数方框"。v2 时代还留下了 `CaskaydiaCove NF` 这类带 NF 后缀的旧家族名，与新版并存时终端下拉框里会出现多个相近名字，选错一个就等于在用旧字形。

### 怎么彻底清掉旧字体重装？

在 **设置 → 个性化 → 字体** 搜索 `Caskaydia`，把搜出来的**所有**同族条目逐一卸载；如果既装过系统级又装过用户级，`C:\Windows\Fonts` 和用户字体两处都要清。删完后彻底退出终端再重开（字体缓存才会刷新），重新安装最新版——版本以 nerdfonts.com 官网最新为准。用 hudo 的话更省事：`hudo uninstall omp` 再 `hudo install omp`，卸载阶段会连注册表项一起清干净，避免手动漏删。

排查到这里三层走完，如果你是从零开始配终端，主题、配色、透明背景这些进阶玩法可以接着看 [Windows 终端美化完整教程](/blog/windows-terminal-beautify)。

## 常见问题

### 装完字体还是方框，要重启电脑吗？

一般不用。完全退出终端程序再打开即可（Windows Terminal 要关闭所有窗口，VS Code 执行 Reload Window）；极少数字体缓存不刷新的情况，注销一次 Windows 即可解决。

### CaskaydiaCove Nerd Font、Mono、Propo 三个变体选哪个？

终端选前两个之一：不带后缀的版本图标占两个字符宽、更大更清楚；Mono 把图标压进单字符宽、对齐最严格，图标被裁掉半边时换它。Propo 是比例字体，不适合终端。

### 只有 VS Code 里乱码，Windows Terminal 正常？

因为两者的字体设置完全独立。Windows Terminal 改的是它自己的配置文件，VS Code 集成终端要单独设置 `terminal.integrated.fontFamily`，见上文第二层。

### 显示的是 � 而不是方框，也是字体问题吗？

不是。� 是 UTF-8 解码失败的替换字符，属于编码问题，方向应查终端代码页（`chcp 65001`）和脚本文件的保存编码；字体缺字形只会显示方框或空白。

### 不想用 CaskaydiaCove，换别的 Nerd Font 行吗？

行。JetBrains Mono、Fira Code、Hack 等主流等宽字体在 nerdfonts.com 都有对应的 Nerd Font 版本，下载后按上文手动方式安装，再把终端字体名同步改掉即可——认准 v3 命名（不带 NF 后缀的完整家族名）。

## 相关阅读

- [oh-my-posh 工具页：hudo 安装参数、换主题与卸载清理完整参考](/tools/omp)
- [Windows 终端美化完整教程：从 Oh My Posh 主题到配色方案](/blog/windows-terminal-beautify)
