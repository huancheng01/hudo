---
title: Gradle vs Maven：Windows 上如何选择与快速安装
description: 对比 Java 两大构建工具 Gradle 和 Maven 的优劣，详解 Windows 下 Gradle 传统安装流程的痛点，以及如何用 hudo 一条命令完成 Gradle 下载、环境变量配置和阿里云镜像设置。
keywords:
  - Windows Gradle 安装
  - Gradle Maven 对比
  - Gradle 安装教程
  - Java 构建工具
  - GRADLE_HOME
  - init.gradle
  - 阿里云镜像
  - hudo
---

# Gradle vs Maven：Windows 上如何选择与快速安装

Java 生态有两大构建工具：**Maven** 和 **Gradle**。Maven 用 XML 配置，约定优于配置，稳定可预测；Gradle 用 Groovy/Kotlin DSL，灵活高效，构建速度更快。两者各有拥趸，选哪个？怎么装？这篇文章帮你理清楚。

## Maven vs Gradle 快速对比

| | Maven | Gradle |
|---|---|---|
| **配置格式** | XML（`pom.xml`） | Groovy/Kotlin DSL（`build.gradle`） |
| **构建速度** | 一般 | 增量编译 + 构建缓存，大项目快数倍 |
| **学习曲线** | 低，XML 直观 | 稍高，需学 DSL 语法 |
| **适用场景** | 传统 Java 企业项目 | Android、Spring Boot、多模块项目 |
| **生态成熟度** | 极高，几乎所有 Java 库都发布到 Maven Central | 完全兼容 Maven 仓库 |

**简单结论**：如果你做 Android 开发或 Spring Boot 新项目，Gradle 是事实标准；如果维护传统企业 Java 项目，Maven 够用且团队更熟悉。当然，两者可以共存，不必二选一。

## 传统安装 Gradle：步骤不少

手动安装 Gradle 的完整流程：

1. 前往 [gradle.org](https://gradle.org/releases/) 下载 Binary-only ZIP（国内直连速度看运气）
2. 解压到某个目录，比如 `D:\gradle-8.12`
3. 打开"系统属性" → "高级" → "环境变量"，新建 `GRADLE_HOME`，值填 Gradle 安装路径
4. 编辑 `PATH`，添加 `%GRADLE_HOME%\bin`
5. 确保已经安装了 JDK 并配置了 `JAVA_HOME`——Gradle 运行必须依赖 JDK
6. 打开新终端，执行 `gradle --version` 验证

这六步和装 Maven 如出一辙，属于"环境变量仪式"。但还有一个关键配置被大多数教程忽略了。

## init.gradle：被遗忘的加速配置

Gradle 默认从 Maven Central（`repo.maven.apache.org`）和 Gradle Plugin Portal 下载依赖，服务器在海外。国内开发者第一次执行 `gradle build`，经常要等十几分钟甚至超时失败。

解决办法是创建 `~/.gradle/init.gradle`，全局配置阿里云镜像：

```groovy
allprojects {
    repositories {
        mavenLocal()
        maven { url 'https://maven.aliyun.com/repository/central' }
        maven { url 'https://maven.aliyun.com/repository/public' }
        mavenCentral()
    }
}
```

这个文件对所有项目生效，不需要逐个修改 `build.gradle`。但问题和 Maven 的 `settings.xml` 一样——**绝大多数安装教程根本不提这一步**，新手装完 Gradle 就直接用，然后被龟速依赖下载劝退。

## 用 hudo 一条命令搞定

[hudo](https://hudo.zexa.cc) 是一个 Windows 开发环境引导工具，安装 Gradle 只需要：

```powershell
hudo install gradle
```

这条命令自动完成以下所有操作：

- **下载 Gradle**：从 Gradle 官方源下载，失败时自动回退到华为云镜像，国内网络也能顺畅下载
- **解压并安装**：自动解压到 hudo 管理目录（`D:\hudo\tools\gradle`）
- **设置环境变量**：自动配置 `GRADLE_HOME` 并将 `bin` 目录添加到用户 `PATH`
- **生成 init.gradle**：自动在 `~/.gradle/` 下创建 `init.gradle`，预置阿里云仓库镜像
- **检测 JDK 依赖**：Gradle 运行必须有 JDK，hudo 会自动检测，未安装时提示先装 JDK（`hudo install jdk` 同样一条命令）

不用打开"系统属性"，不用手写 Groovy 配置文件，连 JDK 依赖检测都帮你做了。

## 安装后验证

安装完成后，打开一个新的终端窗口：

```powershell
gradle --version
```

看到类似以下输出即表示安装成功：

```
Gradle 8.12.1
Build time:   2025-01-24 10:15:42 UTC
JVM:          21.0.6 (Eclipse Adoptium)
OS:           Windows 11 10.0 amd64
```

## 两者可以共存

如果你的工作中既有 Maven 项目又有 Gradle 项目，完全不用纠结，两者互不冲突：

```powershell
hudo install maven
hudo install gradle
```

hudo 会分别设置 `MAVEN_HOME` 和 `GRADLE_HOME`，各自的 `bin` 目录独立添加到 `PATH`，同时为两者生成各自的国内镜像配置（Maven 的 `settings.xml` 和 Gradle 的 `init.gradle`）。需要哪个用哪个，和平共处。
