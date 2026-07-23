---
date: 2026-07-24
author: Zexa
title: Java 开发环境搭建 2026：一条命令装齐 JDK + Maven + IntelliJ IDEA
description: Windows Java 环境搭建完整教程：拆解手动装 JDK 配 JAVA_HOME、Maven 配 settings.xml 镜像、IDEA 安装的四步之痛，用 hudo 三条命令装齐 JDK 21 + Maven + IntelliJ IDEA，环境变量与阿里云镜像全自动，含 IDEA 2025.3 免费层说明。
keywords:
  - java环境搭建
  - idea安装教程及环境配置
  - JDK 安装
  - JAVA_HOME 配置
  - Maven 安装
  - 阿里云镜像
  - settings.xml
  - IntelliJ IDEA 免费版
  - IDEA 2025.3
  - Windows Java 开发环境
  - hudo
---

# Java 开发环境搭建 2026：一条命令装齐 JDK + Maven + IntelliJ IDEA

::: tip TL;DR
在 Windows 上搭一套完整 Java 开发环境（JDK 21 + Maven + IntelliJ IDEA），用 [hudo](https://hudo.zexa.cc) 只需三条命令：`hudo install jdk`、`hudo install maven`、`hudo install idea`。JAVA_HOME、MAVEN_HOME、PATH、阿里云镜像 settings.xml 全部自动配置，IDEA 为免安装版解压即用，全程免管理员权限。还没装 hudo？先执行 `irm hudo.zexa.cc/install.ps1 | iex`。
:::

搭 Java 环境从来不是"装一个东西"，而是装一串东西：JDK、构建工具、IDE，每一样都拖着自己的配置尾巴。按传统方式走完全程，熟手要半小时起步，新手踩着坑走一下午也不稀奇。这篇按"全家桶"思路走一遍：先看清手动方式的四关卡在哪，再用三条命令一次装齐。

## 手动搭 Java 环境要过哪几关？

四关：JDK 与 JAVA_HOME、Maven 与镜像、Gradle（如果项目用）、IDE。每一关都能独立卡人。

### 第一关：JDK 装完为什么命令行找不到 java？

因为解压 JDK 只是第一步，环境变量才是正戏。手动流程：从 [adoptium.net](https://adoptium.net) 下载 Temurin 21（LTS）压缩包并解压；`Win + R` 输入 `sysdm.cpl` → 高级 → 环境变量，新建 `JAVA_HOME` 指向 JDK 目录；再编辑 `PATH` 追加 `%JAVA_HOME%\bin`；最后**开新终端**验证 `java -version`——在旧终端里敲永远是"不是内部或外部命令"。路径带 `+` 号的目录名、过时教程里的 `CLASSPATH`，都是这一关的经典翻车点，完整的坑列表见[这篇 JDK 安装详解](/blog/windows-jdk-install)。

### 第二关：Maven 装好了，为什么拉依赖还是龟速？

因为 Maven 默认从海外的 Maven Central 下载依赖，不配国内镜像就是几十 KB/s。手动方式除了解压、配 `MAVEN_HOME` 和 `PATH`，还必须手动创建 `%USERPROFILE%\.m2\settings.xml`，把 central 指向阿里云镜像（`https://maven.aliyun.com/repository/central`）。这一步没有任何安装向导会提醒你，也是最常被漏掉的一步，完整 XML 写法见 [Maven 镜像配置详解](/blog/windows-maven-install)。

### 第三关：项目用 Gradle 怎么办？

又是一套独立仪式：下载解压、`GRADLE_HOME`、`PATH`、仓库镜像，一样都不能少。hudo 同样支持[一键安装 Gradle](/tools/gradle)，本文以 Maven 为主线不展开。

### 第四关：IDEA 安装向导到底在装什么？

官方安装包约 1.4GB，安装向导一路写注册表、建开始菜单项、问你要不要加 PATH；装完打开工程，还有最后一步——在 IDE 里手动指认 SDK，否则代码全是红的。

四关走完，你在"系统属性"对话框里进出了至少两次，编辑了一个 XML，点了十几次"下一步"。

## hudo 三条命令分别做了什么？

每条命令 = 下载 + 解压 + 环境变量 + 该工具的专属配置，装完即用。hudo 是支持 27 款工具的 Windows 开发环境引导工具，先安装它：

```powershell
irm hudo.zexa.cc/install.ps1 | iex
```

然后按顺序执行：

```powershell
hudo install jdk
hudo install maven
hudo install idea
```

### hudo install jdk 装的是什么版本？

默认装 [Eclipse Temurin](https://adoptium.net) JDK 21（LTS），解压到 `{install_root}\lang\java`（`install_root` 是初始化时选的盘，例如 `D:\hudo`），并自动配置 `JAVA_HOME` 与 `PATH`。环境变量写的是用户级（`HKCU\Environment`），所以不需要管理员权限。细节见 [JDK 工具页](/tools/jdk)。

### hudo install maven 会帮我配阿里云镜像吗？

会，这正是它和"下载解压"式安装的最大区别。除了装到 `{install_root}\tools\maven`、配置 `MAVEN_HOME` 和 `PATH`，hudo 还会自动生成 `~/.m2/settings.xml` 并预置阿里云中央仓库镜像；如果该文件已存在则跳过，不会覆盖你已有的配置。安装前还会检测 JDK 是否就位，没装会提示先装。版本以 Maven 官网最新稳定版为准，参数见 [Maven 工具页](/tools/maven)。

### hudo install idea 和官网安装包有什么区别？

hudo 装的是免安装版：下载链接直接取 JetBrains 官方 API 的最新版（约 1.4GB，耐心等待），解压到 `{install_root}\ide\idea`，不运行安装向导、不写系统注册表。启动方式就是运行 `{install_root}\ide\idea\bin\idea64.exe`。详见 [IntelliJ IDEA 工具页](/tools/idea)。

三样装完，开一个新终端执行 `mvn --version`，一行输出同时确认两件事——`Maven home` 指向 hudo 目录、`Java version` 的 vendor 是 Eclipse Adoptium，说明整条链路已经串通。如果提示找不到命令，先确认开的是**新**终端：环境变量只对修改之后新开的终端生效，这是手动派和自动派共同的第一坑。

需要锁定版本（比如团队统一环境）时，编辑 `%USERPROFILE%\.hudo\config.toml`：

```toml
[versions]
jdk = "21"        # JDK 主版本号
maven = "3.9.9"   # 不填则取官网最新
idea = "2025.3"   # 不填则取官网最新
```

## IDEA 里怎么关联 hudo 装的 JDK？

把 SDK 指向 `{install_root}\lang\java` 即可，四步：

1. 打开 IDEA，File → Project Structure（快捷键 `Ctrl + Alt + Shift + S`）
2. Platform Settings → SDKs → 点 `+` → Add JDK
3. 选择 `{install_root}\lang\java`（例如 `D:\hudo\lang\java`），IDEA 会自动识别版本
4. 回到 Project Settings → Project，把 Project SDK 选成刚添加的这个

SDK 是 IDEA 的全局配置，添加一次之后其他项目直接在下拉框里选即可，不用重复指认；新建项目时的对话框里同样能选到它。

Maven 那边不用任何操作：IDEA 默认读取 `~/.m2/settings.xml` 作为 User settings file，hudo 生成的阿里云镜像在 IDE 里同样生效，打开 Maven 工程直接走国内镜像拉依赖。

## IDEA 2025.3 之后还分社区版和旗舰版吗？

不分了。自 2025.3 起社区版并入统一发行版：同一个安装包，未激活订阅时即为免费层，能力对应原社区版——Java/Kotlin 开发、调试、Maven/Gradle、Git 集成都可用。Spring 全家桶深度支持、数据库工具、Profiler 等属于付费订阅。对安装环节的影响是下载页和包名都换了代——如果你搜到的教程还在教"去官网下载社区版（Community Edition）"，那个入口已经合并，不必再找。hudo 的下载链接直接取 JetBrains 官方 API，不受命名换代影响，`hudo install idea` 拿到的始终是当前最新统一发行版。

## 常见问题

### 免费层（原社区版）做 Java 开发够用吗？

够用。Java/Kotlin 编码、调试、Maven/Gradle 构建、Git 集成在免费层全部可用，学习和大多数后端项目完全足够；重度 Spring 支持、内置数据库工具、Profiler 才需要订阅。

### Maven 的阿里云镜像配置在哪，想改怎么改？

在 `%USERPROFILE%\.m2\settings.xml`。hudo 生成的配置把 central 指向 `https://maven.aliyun.com/repository/central`，要换公司私服直接编辑这个文件即可；hudo 不会覆盖已存在的 settings.xml，改完不怕被重装冲掉。

### 必须按 jdk → maven → idea 的顺序装吗？

JDK 必须在 Maven 之前——Maven 是 Java 程序，hudo 安装时会检测 JDK，未装会提示先装。IDEA 的顺序无所谓，装完在 Project Structure 里指认 SDK 即可。

### 项目要 JDK 17，怎么不装 21？

在 `%USERPROFILE%\.hudo\config.toml` 里写 `[versions]` 下的 `jdk = "17"`，再执行 `hudo install jdk`。Maven 和 IDEA 同理可在[配置文件](/guide/config)锁定版本。

### 整个过程需要管理员权限吗？

不需要。hudo 把 `JAVA_HOME`、`MAVEN_HOME`、`PATH` 都写入用户级环境变量，IDEA 免安装版也不写系统注册表，全程没有 UAC 弹窗。

## 相关阅读

- [IntelliJ IDEA 工具页：版本锁定、下载镜像与卸载](/tools/idea)
- [JDK 工具页：Temurin 发行版说明与配置项](/tools/jdk)
- [Maven 工具页：安装参数与版本管理](/tools/maven)
- [JDK 手动配置 JAVA_HOME 的所有坑与一键方案](/blog/windows-jdk-install)
- [Maven settings.xml 阿里云镜像完整配置详解](/blog/windows-maven-install)
