---
date: 2026-07-24
author: Zexa
title: 7z 命令行完全速查：压缩、解压、加密、分卷、排除文件（含 PowerShell 脚本示例）
description: 7z 命令行速查手册：涵盖 a 压缩、x 解压、l 列表、t 测试、-p 加密与 -mhe 隐藏文件名、-v 分卷、-x! 排除文件、-mx 压缩级别等 7zip 命令行参数，每条附可复制示例；另有批量压缩子目录与定时加密备份两个 PowerShell 脚本，以及用 hudo 免管理员安装 7-Zip 并自动配置 PATH 的方法。
keywords:
  - 7zip命令行参数
  - 7z 解压 命令
  - 7zip命令行加密压缩
  - 7z 分卷压缩
  - 7z 排除文件
  - 7z 压缩级别
  - 7-Zip PowerShell 脚本
  - Windows 命令行压缩
  - 7z x 和 e 的区别
  - hudo
---

# 7z 命令行完全速查：压缩、解压、加密、分卷、排除文件（含 PowerShell 脚本示例）

::: tip TL;DR
7-Zip 的命令行程序是 `7z.exe`：`7z a` 压缩、`7z x` 解压、`-p` 配合 `-mhe=on` 做加密并隐藏文件名、`-v` 分卷、`-x!` 排除文件、`-mx` 调压缩级别。官方安装器**不会**把 7z 加进 PATH；执行 `hudo install 7zip` 便携安装，免管理员，装完终端直接可用 `7z`。
:::

## 怎么在 Windows 上装出 7z 命令？

最快的方式是一条命令：

```powershell
hudo install 7zip
```

[hudo](https://hudo.zexa.cc)（支持 26 款开发工具的 Windows 环境引导工具）用便携方式安装 7-Zip：免管理员权限、不写注册表、自动把 `7z` 配置进 PATH，新开终端即可使用。版本号不指定时自动取最新版（写作时为 26.02，以官网最新为准）。版本锁定、卸载与已知限制见 [7-Zip 工具页](/tools/7zip)。

手动安装也可以，但有一个几乎人人都会踩的坑：**官方安装器不把 7z 加入 PATH**。步骤是：

1. 从 [7-zip.org](https://www.7-zip.org/) 下载 `7z-x64.exe` 安装器，运行（需要 UAC 提权），默认装到 `C:\Program Files\7-Zip`
2. 此时终端里输入 `7z` 会提示"不是内部或外部命令"，需要手动把安装目录加入用户 PATH：

```powershell
[Environment]::SetEnvironmentVariable('Path',
  [Environment]::GetEnvironmentVariable('Path', 'User') + ';C:\Program Files\7-Zip', 'User')
```

不要用网上常见的 `setx PATH "%PATH%;..."`——`setx` 会把系统 PATH 和用户 PATH 合并后整段写回用户变量，且超过 1024 字符会静默截断，PATH 长的机器直接被写坏。改完 PATH 后**开一个新终端**才生效。

## 7z 的命令语法是什么结构？

固定结构是：`7z <命令> [开关] <压缩包> [文件或目录...]`。

```text
7z a  -mx=9 -p123  backup.7z  D:\data\    # a 是命令，-mx/-p 是开关
7z x  backup.7z  -oD:\out                 # 开关也可以放在压缩包后面
```

两个高频坑提前说：`-o`（输出目录）和 `-p`（密码）**后面直接跟值，不能有空格**；`7z x a.7z -o D:\out` 是错的，`-oD:\out` 才对。

## 7z 怎么压缩文件和文件夹？（a 命令）

`7z a 压缩包名 要压缩的内容` 即可创建压缩包，格式由扩展名或 `-t` 决定。

```powershell
# 把整个 docs 文件夹压进去（压缩包内包含 docs 这一层目录）
7z a archive.7z .\docs\

# 只压缩 docs 里面的内容（压缩包内没有 docs 这一层）
7z a archive.7z .\docs\*

# 压成 zip 格式（发给别人、对方没装 7-Zip 时用）
7z a -tzip archive.zip .\docs\

# 压完删除源文件（谨慎使用）
7z a archive.7z .\logs\* -sdel
```

`.\docs\` 和 `.\docs\*` 的区别经常被忽略：前者归档时保留顶层目录，后者不保留。解压后目录结构乱掉，多半是这里没选对。

## 7z 怎么解压？x 和 e 有什么区别？

`x` 保留目录结构，`e` 把所有文件平铺到一个目录——日常解压几乎总是该用 `x`。

```powershell
# 解压到当前目录，保留目录结构
7z x archive.7z

# 解压到指定目录（-o 后面不能有空格）
7z x archive.7z -oD:\out

# e 会把所有子目录里的文件抽平到一层，同名文件互相覆盖提示
7z e archive.7z -oD:\flat
```

遇到目标目录已有同名文件时，7z 默认逐个询问。脚本里用不上交互，加覆盖策略开关：`-aoa` 全部覆盖、`-aos` 全部跳过、`-aou` 冲突时自动改名。

## 怎么查看压缩包内容和测试完整性？（l 和 t 命令）

`7z l` 列出内容不解压，`7z t` 校验每个文件的 CRC 是否完好。

```powershell
7z l archive.7z          # 列出文件、大小、压缩率
7z l -slt archive.7z     # 每个文件的详细技术信息（加密方式、压缩算法等）
7z t archive.7z          # 测试完整性，下载的大文件解压前先跑一遍
```

## 7z 怎么加密压缩？-p 和 -mhe 有什么区别？

`-p` 只加密文件内容，文件名仍然裸露；加上 `-mhe=on` 才连文件名一起加密——加密压缩建议两个一起用。

```powershell
# 内容加密，但 7z l 不输密码也能看到文件名列表
7z a -pMyS3cret archive.7z .\secret\

# 内容 + 文件名都加密，列表前就要求输密码（推荐）
7z a -pMyS3cret -mhe=on archive.7z .\secret\

# 解压加密压缩包（不带 -p 会交互式询问密码）
7z x archive.7z -pMyS3cret
```

注意 `-mhe=on` **只对 7z 格式有效**。如果必须用 zip 格式加密，用 `-mem=AES256` 替代默认的弱加密 ZipCrypto，但 zip 格式无论如何都藏不住文件名：

```powershell
7z a -tzip -pMyS3cret -mem=AES256 archive.zip .\secret\
```

密码里含 PowerShell 特殊字符（`$`、反引号等）时，把整个开关用单引号包起来：`'-pMy$ecret'`。

## 怎么分卷压缩大文件？（-v 参数）

`-v` 后跟每卷大小即可分卷，单位支持 b、k、m、g，产物是 `.7z.001`、`.7z.002` 这样的序列。

```powershell
# 按 2GB 一卷切分（比如要传到单文件限 2GB 的网盘）
7z a -v2g backup.7z D:\bigdata\

# 4.48GB 一卷，刚好一张 DVD
7z a -v4480m backup.7z D:\bigdata\
```

解压时**只操作第一卷**，7z 自动接续后面的卷：

```powershell
7z x backup.7z.001
```

两个限制：所有分卷必须在同一目录且一卷不缺；分卷压缩包**不支持更新**（`u` 命令无效），内容变了只能整个重压。

## 怎么排除不想压缩的文件？（-x! 参数）

`-x!通配符` 排除匹配的文件，`-xr!` 递归排除所有层级——排除目录基本都要用 `-xr!`。

```powershell
# 压缩项目但排除依赖和构建产物（PowerShell 里给开关加引号，避免解析歧义）
7z a project.7z .\myapp\ '-xr!node_modules' '-xr!target' '-xr!.git'

# 排除所有 .log 和 .tmp 文件
7z a data.7z .\data\ '-xr!*.log' '-xr!*.tmp'
```

反向操作是 `-i!`（只包含匹配项），规则相同。

## 压缩级别 -mx 怎么选？

日常用默认的 `-mx=5` 就够，追求体积用 `-mx=9`，只打包不求压缩用 `-mx=0`。

| 开关 | 含义 | 适用场景 |
|------|------|---------|
| `-mx=0` | 仅存储不压缩 | 打包已压缩内容（图片、视频、jar） |
| `-mx=1` | 最快 | 临时中转，速度优先 |
| `-mx=5` | 默认均衡 | 日常备份 |
| `-mx=9` | 极限压缩 | 长期归档、上传前榨体积，慢且吃内存 |

```powershell
7z a -mx=9 archive.7z .\src\     # 极限压缩
7z a -mx=0 pack.7z .\videos\     # 视频只打包，压了也白压
```

7z 格式默认 LZMA2 算法，多核并行默认开启；想限制线程数（比如避免备份任务占满 CPU）加 `-mmt=2`。

## PowerShell 实战脚本

### 怎么批量把每个子目录压成独立压缩包？

用 `Get-ChildItem -Directory` 枚举子目录，逐个喂给 `7z a`：

```powershell
Get-ChildItem D:\projects -Directory | ForEach-Object {
    7z a -mx=7 "D:\archive\$($_.Name).7z" $_.FullName
    if ($LASTEXITCODE -ne 0) { Write-Warning "$($_.Name) 压缩失败，退出码 $LASTEXITCODE" }
}
```

跑完 `D:\archive\` 下就是 `项目名.7z` 一一对应，适合归档多个旧项目。

### 怎么做每天凌晨的定时加密备份？

先写一个备份脚本 `D:\scripts\backup.ps1`（按日期命名、加密、排除依赖目录、只保留最近 14 份）：

```powershell
$stamp = Get-Date -Format 'yyyy-MM-dd'
$dest  = "E:\backup\project-$stamp.7z"

7z a -mx=5 -mhe=on '-pYourStrongPassword' '-xr!node_modules' '-xr!.git' $dest 'D:\project\'
if ($LASTEXITCODE -gt 1) { throw "备份失败，7z 退出码 $LASTEXITCODE" }

# 按文件名倒序，跳过最新 14 份，其余删除
Get-ChildItem E:\backup\project-*.7z | Sort-Object Name -Descending |
    Select-Object -Skip 14 | Remove-Item
```

再用任务计划注册为每天 02:30 执行：

```powershell
schtasks /Create /TN "daily-backup" /SC DAILY /ST 02:30 `
  /TR "powershell -NoProfile -ExecutionPolicy Bypass -File D:\scripts\backup.ps1"
```

脚本里判断 `-gt 1` 而不是 `-ne 0`，是因为 7z 退出码 1 只是警告（比如个别文件被占用没压进去），2 才是致命错误——备份任务里把警告当失败会导致大量误报。

## 常见问题

### 7z 和 zip 格式怎么选？

自用选 7z，分发选 zip。7z 用 LZMA2 算法，对文本、代码、日志这类内容压缩率明显高于 zip，且支持 `-mhe=on` 加密文件名；zip 的优势只有一个——Windows 资源管理器不装任何软件就能打开。给不确定装没装 7-Zip 的人发文件用 `-tzip`，自己备份归档一律 7z。

### 为什么装了 7-Zip，终端里输入 7z 还是提示"不是内部或外部命令"？

因为官方安装器不把安装目录加入 PATH。要么按上文手动把 `C:\Program Files\7-Zip` 加进用户 PATH，要么用 `hudo install 7zip` 便携安装（自动配 PATH，且免管理员）。改完记得开新终端——PATH 变更对已开着的终端窗口不生效。

### 加密的 7z 文件忘了密码还能解开吗？

不能。7z 加密用 AES-256，没有后门和"恢复密码"功能，唯一的办法是穷举猜密码——强密码下不可行。所以定时备份脚本里的密码务必单独记录在密码管理器里。

### 7z 的退出码 0、1、2 分别代表什么？

0 是完全成功；1 是警告，操作完成但有个别文件异常（典型如文件被其他进程占用而跳过）；2 是致命错误。此外 7 表示命令行参数错误，255 表示用户中断。写脚本时按需选择把 1 当成功还是失败。

## 相关阅读

- [7-Zip 工具页：hudo 安装参数、版本锁定与便携方式的已知限制](/tools/7zip)
- [新电脑到手后，用 hudo 20 分钟配好完整 Windows 开发环境](/blog/windows-dev-environment-setup)
