---
date: 2026-07-24
author: Zexa
title: 程序员换电脑完整迁移指南（2026）：环境变量、工具链、配置一个不落
description: 程序员换电脑迁移清单：工具链、环境变量、配置文件、凭据、数据五类逐一讲透，对比纯手动、winget export、hudo 档案三种方案的耗时与完整度。
keywords:
  - 程序员换电脑
  - 换电脑 开发环境迁移
  - 开发环境迁移
  - Windows 环境变量迁移
  - winget export
  - hudo export
  - 配置文件迁移
  - .gitconfig 迁移
  - mysqldump 导出导入
  - 新电脑开发环境
---

# 程序员换电脑完整迁移指南（2026）：环境变量、工具链、配置一个不落

::: tip TL;DR
程序员换电脑要迁移的是五类东西：工具链、环境变量、配置文件、凭据、数据。工具链用清单驱动重装（`winget export` 或 hudo 档案），环境变量导出注册表只作参照、逐条重建，配置文件（`.gitconfig`、`.ssh`、`settings.xml`）直接拷贝，凭据一律不迁、新机重新登录，数据库用 `mysqldump` 逻辑导出。纯手动约 4-8 小时；旧电脑执行 `hudo export`，新电脑 `hudo import hudo-profile.toml`，工具、版本、环境变量 10-20 分钟一次到位。
:::

换电脑最大的风险不是"慢"，是"漏"：新机器用了三天才发现 Go 没装，跑项目报错才想起 `JAVA_HOME` 没配。解决办法只有一个——先列清单，再按类处理，而不是想起什么装什么。

## 换电脑到底要迁移哪些东西？

一共五类：工具链、环境变量、配置文件、凭据、数据。漏掉任何一类，新电脑都会在之后某天突然报错。

| 类别 | 典型内容 | 迁移方式 |
|------|---------|---------|
| 工具链 | Git、Node.js、JDK、MySQL、VS Code 等软件本体 | 清单驱动重装，不要拷安装目录 |
| 环境变量 | PATH、`JAVA_HOME`、`GOPATH`、`MAVEN_HOME` | 导出注册表作参照，逐条重建 |
| 配置文件 | `.gitconfig`、`.ssh`、Maven `settings.xml`、VS Code 设置 | 直接拷贝文件或账号同步 |
| 凭据 | gh token、npm token、各类登录态 | 不迁移，新机重新登录 |
| 数据 | MySQL/PostgreSQL 数据库、Redis 持久化 | `mysqldump` 等逻辑导出再导入 |

顺带回答一个常见念头：为什么不整盘克隆？克隆会把旧盘符依赖、过时驱动、几年攒下的注册表垃圾一起带过去，还可能触发 Windows 激活问题。换电脑本来就是清理旧账的机会，克隆等于放弃这个机会。

## 工具链怎么迁：直接拷文件夹还是重装？

除少数绿色软件外，一律重装——但要用"清单"驱动重装，不能靠回忆。安装式软件的关键状态（注册表项、Windows 服务注册、卸载信息）都不在安装目录里，把 `C:\Program Files` 拷过去大概率启动不了。

### winget export 能导出什么？

`winget export` 只导出"装了哪些软件"这一件事。旧电脑执行：

```powershell
winget export -o packages.json --include-versions
```

新电脑执行 `winget import packages.json` 即可批量重装。它的局限也很明确：只装软件本体——环境变量不配、配置文件不写、MySQL 服务不注册，装完 JDK 之后 `JAVA_HOME` 还是得自己来。此外只有软件是从 winget 源装的才会进清单，官网下载安装的一律不在内。各家包管理器的详细差异见[Scoop、Winget、Chocolatey 与 hudo 的横向对比](/blog/windows-package-manager-compare)。

### hudo 档案和 winget 清单差在哪？

差在"装完即可用"：hudo 的档案除了工具清单和版本号，还记录镜像配置、版本锁定和 Git 身份，导入时按清单批量安装（共支持 26 款开发工具）、自动写环境变量（写入用户级注册表，无需管理员权限）、注册 MySQL/PostgreSQL/Redis 服务。旧电脑一条命令导出：

```powershell
hudo export
```

生成的 `hudo-profile.toml` 拷到新电脑后 `hudo import hudo-profile.toml`（加 `-y` 可无人值守）。也可以把档案放到网盘或内网，新机装 hudo 时通过 `HUDO_PROFILE` 环境变量直接带上，安装与还原一条命令完成。完整操作流程和团队标准化用法见[hudo 配置档案迁移实战](/blog/windows-dev-env-migration)，命令参数见[配置档案文档](/guide/profile)，此处不展开。

## 环境变量怎么迁：注册表能整段导入吗？

能导出，但强烈不建议整段导入——导出文件只能当"抄写参照"。Windows 的用户级环境变量存在注册表 `HKCU\Environment`，系统级存在 `HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Environment`，导出命令：

```powershell
reg export "HKCU\Environment" env-user.reg
```

拿到 `.reg` 文件后别双击导入，原因有三：

1. **PATH 里全是旧机器的绝对路径**——盘符、用户名（`C:\Users\旧用户名\...`）在新机上多半对不上；
2. **导入是整体覆盖**——会把新系统 PATH 里的默认条目直接顶掉；
3. **几年积累的失效条目会原样带过来**，换电脑本该是清掉它们的时机。

正确做法：用记事本打开 `.reg` 对照，把仍然需要的变量在新机逐条重建。重建时避开一个经典坑——`setx` 写 PATH 会把内容截断到 1024 字符，长 PATH 必然丢数据。要么走"系统属性 → 环境变量"图形界面，要么用 PowerShell：

```powershell
[Environment]::SetEnvironmentVariable("JAVA_HOME", "D:\java\jdk-21", "User")
```

如果工具链走 hudo 档案还原，这一节的手工活基本可以省掉：`JAVA_HOME`、`GOPATH`、`MAVEN_HOME` 这些会在导入时自动写好，只剩你自定义的私有变量需要照着 `.reg` 补几条。

## 配置文件怎么迁：哪些文件值得直接拷走？

这是五类中唯一"拷文件就能用"的一类，重点四个：

- **`%USERPROFILE%\.gitconfig`**——Git 身份、别名、`autocrlf` 全在里面，整个文件拷过去即可（各字段含义见 [Windows Git 安装与配置教程](/blog/windows-git-install)）；
- **`%USERPROFILE%\.ssh\`**——私钥、`config`、`known_hosts` 整个目录拷贝，拷完确认私钥没进过聊天软件或公共网盘；
- **`%USERPROFILE%\.m2\settings.xml`**——Maven 的阿里云镜像和私服配置，不拷的话新机第一次构建就会卡在中央仓库；
- **VS Code**——首选官方 Settings Sync 登录账号同步设置与扩展；不想登录就拷 `%APPDATA%\Code\User\settings.json`，再用 `code --list-extensions > ext.txt` 导出扩展清单到新机逐个装。用 hudo 装的是便携版 [VS Code](/tools/vscode)，设置和扩展都在 `data\` 目录，整个目录拷过去即可。

同理可拷的还有 `.npmrc`、`.condarc`、`.cargo\config.toml` 这类纯文本配置，规律是：在用户目录下、纯文本、不含绝对路径的，都能直接搬。

## 凭据怎么办：token 和登录态能直接拷过去吗？

不能，也不应该。Windows 凭据管理器里的条目经 DPAPI 加密，绑定当前用户和机器，拷到新电脑就是一堆解不开的密文；而把 token 抄进迁移文档，等于敏感凭据明文落盘，文件一外泄全部失守。hudo 档案设计上就不导出 GitHub CLI 的登录令牌，导入完成后自动提示 `gh auth login`——正确姿势就是这个：新机上 `gh auth login`、`npm login`、`docker login` 挨个重新登录，每个不超过一分钟。

SSH 私钥是唯一的例外，可以随 `.ssh` 目录迁移；但更稳妥的做法是每台机器单独生成一把、旧机器的公钥在 GitHub 上删除——单机泄露时只需吊销一把钥匙。

## 数据库数据怎么迁：mysqldump 怎么用？

用逻辑导出导入，不要直接拷数据目录。数据目录的磁盘格式跨版本不保证兼容，拷过去起不来是常态；逻辑导出生成的是纯 SQL 文本，版本安全。

注意一个 PowerShell 专属的坑：**不要用 `>` 重定向导出**——PowerShell 5.1 的 `>` 会把输出转成 UTF-16 编码，导出的 SQL 文件 `mysql` 读不回去；`<` 输入重定向则是 PowerShell 的保留操作符，直接报解析错误。正确做法是用 mysqldump 自带的 `--result-file` 参数导出（绕开 shell 重定向，编码由 mysqldump 自己控制）：

```powershell
mysqldump --all-databases -u root -p --result-file=all.sql
```

新电脑先装好同一主版本的 [MySQL](/tools/mysql)（8.x 对 8.x），导入时用 `source` 命令（同样不依赖 shell 重定向，PowerShell 和 CMD 里都能跑）：

```powershell
mysql -u root -p -e "source all.sql"
```

如果你习惯经典的 `<` 重定向写法，把命令包进 CMD 执行也可以：`cmd /c "mysql -u root -p < all.sql"`。

PostgreSQL 对应 `pg_dumpall -U postgres > all.sql`，同样用 `psql` 导入。Redis 里如果只是缓存，不用迁；确有需要持久数据就在旧机执行 `SAVE` 后把 `dump.rdb` 拷到新机数据目录。注意：无论 winget 还是 hudo，任何工具迁移方案都不含数据库数据，这一步永远要单独做。

## 三种迁移方案对比：哪种适合你？

结论：装的软件屈指可数选纯手动；桌面软件为主选 winget；Windows 开发环境选 hudo 档案。

| 方案 | 耗时 | 覆盖范围 | 适合 |
|------|------|---------|------|
| 纯手动逐个装 | 4-8 小时 | 全靠回忆，最容易漏 | 工具极少的轻度用户 |
| `winget export` / `import` | 1-2 小时 | 只装软件本体，环境变量、配置、服务仍需手动 | 常规桌面软件为主 |
| hudo 档案 | 10-20 分钟 | 26 款开发工具 + 版本 + 镜像 + 环境变量 + 数据库服务 + Git 身份 | Windows 开发环境 |

注意三种方案都只解决前两类（工具链、部分环境变量）：配置文件、凭据、数据这三类，无论选哪种，都要按上文单独处理。

## 常见问题

### C 盘重装了，D 盘的软件还能用吗？

大多数不能。软件的注册表项、Windows 服务注册（比如 MySQL 服务）、`HKCU` 下的环境变量全部随系统盘清空，D 盘只剩一堆双击报错的文件。例外是绿色/便携软件——本身不依赖注册表，文件还在就能跑，但 PATH 也要重配。hudo 装在 `D:\hudo` 的工具文件同样还在，环境变量和服务注册则需要重新 `hudo import` 档案恢复。

### 新旧电脑必须装完全一样的版本吗？

主版本一致即可，比如 Node.js 22 对 22、JDK 21 对 21。需要精确锁定时，winget 用 `--include-versions` 导出，hudo 档案本身就记录每个工具的版本号，导入即还原。

### 旧电脑已经坏了或不在手边，怎么迁？

从项目侧反推：`.nvmrc`、`package.json` 的 `engines`、`go.mod`、`pom.xml`、README 和 CI 配置文件里藏着项目需要的工具与版本。这也是教训——环境还健康的时候就该导一份清单存网盘，`hudo export` 或 `winget export` 都只要几秒钟。

### 微信、浏览器这类非开发软件怎么迁？

走各自的官方通道：浏览器书签和密码登录账号自动同步；微信用"设置 → 聊天 → 聊天记录迁移与备份"传到新机。这类软件没有清单化方案，好在数量少、单个迁移都有官方引导。

### SSH 私钥直接拷过去安全吗？

可行但不是最佳实践。私钥文件本身可以随 `.ssh` 目录复制，但每多一台机器共用同一把钥匙，泄露时的排查和吊销就多一分麻烦；条件允许就每台机器 `ssh-keygen` 一把新的，公钥加到 GitHub，旧机淘汰时删掉对应公钥即可。

## 相关阅读

- [hudo 配置档案一键迁移开发环境的完整实战流程](/blog/windows-dev-env-migration)
- [配置档案 export / import 命令与档案格式参考](/guide/profile)
- [新电脑从零开始：20 分钟配好完整 Windows 开发环境](/blog/windows-dev-environment-setup)
