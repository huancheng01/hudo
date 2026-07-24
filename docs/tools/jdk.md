---
title: "JDK (Java) Windows 安装与配置 - hudo"
description: "使用 hudo 在 Windows 上一键安装 Eclipse Temurin JDK，自动配置 JAVA_HOME 和 PATH 环境变量，支持国内镜像加速下载。"
head:
  - - meta
    - name: keywords
      content: "JDK 安装, Java Windows, JAVA_HOME 配置, Eclipse Temurin, 一键安装 JDK, Java 环境变量, hudo"
---

# JDK

JDK（Java Development Kit）是 Java 应用开发和运行的基础环境。hudo 使用 [Eclipse Temurin](https://adoptium.net) 发行版（原 AdoptOpenJDK），这是社区最受信赖的开源 JDK 构建之一，适用于企业开发和个人学习。

## 安装

```powershell
hudo install jdk
```

安装到 `{install_root}\lang\java\`，默认安装 JDK 21（LTS），可通过配置文件指定主版本号。

## 安装后

```powershell
java -version
javac -version
```

## 卸载

```powershell
hudo uninstall jdk
```

## 配置文件版本

```toml
[java]
version = "21"   # 主版本号，不填则使用 LTS 默认版本（JDK 用独立的 [java] 段）
```

## hudo 安装优势

- **自动配置 JAVA_HOME 和 PATH**：这是 Java 开发最常见的配置痛点，hudo 全自动完成，无需手动编辑环境变量
- **使用 Eclipse Temurin 发行版**：免费、开源、社区维护的 OpenJDK 构建，兼容性和稳定性有保障
- **默认安装 LTS 版本**：自动安装 JDK 21 LTS，也支持在[配置文件](/guide/config)中指定其他主版本号
- **为 [Maven](/tools/maven) 和 [Gradle](/tools/gradle) 铺路**：安装 JDK 后即可继续安装 Java 构建工具，环境变量自动串联

## 常见问题

**Q: 安装后 `java` 命令找不到怎么办？**

打开新的终端窗口即可，hudo 已自动配置 JAVA_HOME 和 PATH 环境变量，需要新终端加载。

**Q: 如何安装 JDK 17 而不是 JDK 21？**

执行 `hudo config set java.version 17`，或在[配置文件](/guide/config) `~/.hudo/config.toml` 的 `[java]` 段设置 `version = "17"` 即可指定主版本号。

**Q: hudo 安装的 JDK 和 Oracle JDK 有什么区别？**

hudo 使用的 Eclipse Temurin 是基于 OpenJDK 源码构建的发行版，功能与 Oracle JDK 完全相同，且免费无商业授权限制。

## 相关阅读

- [还在手动配 JAVA_HOME？一键安装 JDK 并自动配置环境变量](/blog/windows-jdk-install) — 详细安装教程与常见问题解答
