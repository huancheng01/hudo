---
title: "Gradle Windows 安装与配置"
description: "使用 hudo 在 Windows 上一键安装 Gradle 构建工具，自动配置 GRADLE_HOME 和 PATH 环境变量，支持 Java、Kotlin、Android 项目。"
head:
  - - meta
    - name: keywords
      content: "Gradle 安装, Gradle Windows, GRADLE_HOME 配置, 一键安装 Gradle, Java 构建工具, Android 构建, hudo"
---

# Gradle

Gradle 是现代化的多语言构建工具，支持 Java、Kotlin、Groovy 和 Android 项目。它采用灵活的 DSL 脚本配置，构建速度优于 Maven，是 Android 官方指定的构建系统，也被越来越多的 Spring Boot 项目采用。

## 安装

```powershell
hudo install gradle
```

安装到 `{install_root}\tools\gradle\`，自动获取最新版本。需要先安装 JDK。

## 安装后

```powershell
gradle --version
gradle build
```

## 卸载

```powershell
hudo uninstall gradle
```

## 配置文件版本

```toml
[versions]
# 不填则自动获取最新版
gradle = "8.12.1"
```

## hudo 安装优势

- **自动配置 GRADLE_HOME 和 PATH**：无需手动设置环境变量，安装后 `gradle` 命令直接可用
- **自动检测 [JDK](/tools/jdk) 依赖**：Gradle 依赖 Java 环境，hudo 会在安装时检查 JDK 是否已安装
- **自动获取最新版本**：通过 Gradle 官方 API 获取最新稳定版，也支持在[配置文件](/guide/config)中锁定版本
- **与 [Maven](/tools/maven) 灵活选择**：hudo 同时支持两大 Java 构建工具，根据项目需要选择安装

## 常见问题

**Q: 安装后 `gradle` 命令找不到怎么办？**

打开新的终端窗口即可，hudo 已自动配置环境变量，需要新终端加载。

**Q: 项目中有 `gradlew`，还需要全局安装 Gradle 吗？**

大多数 Gradle 项目都自带 Gradle Wrapper（`gradlew`/`gradlew.bat`），可以不依赖全局安装。但全局安装 Gradle 方便初始化新项目（`gradle init`）和在没有 Wrapper 的项目中使用。

**Q: 必须先安装 JDK 吗？**

是的，Gradle 是 JVM 构建工具，运行时需要 JDK。请先运行 `hudo install jdk` 安装 [JDK](/tools/jdk)。

## 相关阅读

- [Gradle vs Maven：Windows 上如何选择与快速安装](/blog/windows-gradle-install) — 详细安装教程与常见问题解答
