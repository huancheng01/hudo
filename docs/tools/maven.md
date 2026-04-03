---
title: "Maven Windows 安装与配置 - hudo"
description: "使用 hudo 在 Windows 上一键安装 Apache Maven，自动配置 MAVEN_HOME 和 PATH 环境变量，支持阿里云镜像仓库加速。"
head:
  - - meta
    - name: keywords
      content: "Maven 安装, Maven Windows, MAVEN_HOME 配置, Maven 镜像, 一键安装 Maven, Java 构建工具, hudo"
---

# Maven

Apache Maven 是 Java 生态中最主流的项目构建和依赖管理工具，通过 `pom.xml` 声明式管理项目依赖和构建流程。绝大多数 Java 企业项目和开源项目都使用 Maven 或兼容其仓库格式。

## 安装

```powershell
hudo install maven
```

安装到 `{install_root}\tools\maven\`，自动获取最新版本。需要先安装 JDK。

## 安装后

```powershell
mvn --version
mvn clean install
```

## 卸载

```powershell
hudo uninstall maven
```

## 配置文件版本

```toml
[versions]
# 不填则自动获取最新版
maven = "3.9.9"
```

## hudo 安装优势

- **自动配置 MAVEN_HOME 和 PATH**：无需手动设置环境变量，安装后 `mvn` 命令直接可用
- **自动检测 [JDK](/tools/jdk) 依赖**：Maven 依赖 Java 环境，hudo 会在安装时检查 JDK 是否已安装
- **自动获取最新版本**：通过 Maven 官方 API 获取最新稳定版，也支持在[配置文件](/guide/config)中锁定版本
- **与 [Gradle](/tools/gradle) 灵活选择**：hudo 同时支持两大 Java 构建工具，根据项目需要选择安装

## 常见问题

**Q: 安装后 `mvn` 命令找不到怎么办？**

打开新的终端窗口即可，hudo 已自动配置环境变量，需要新终端加载。

**Q: Maven 下载依赖很慢怎么办？**

建议在 Maven 的 `settings.xml` 中配置阿里云镜像仓库。详细配置方法参考下方博客文章。

**Q: 必须先安装 JDK 吗？**

是的，Maven 是 Java 构建工具，运行时需要 JDK。请先运行 `hudo install jdk` 安装 [JDK](/tools/jdk)。

## 相关阅读

- [Maven 安装配置一条龙：环境变量 + 阿里云镜像，一条命令全搞定](/blog/windows-maven-install) — 详细安装教程与常见问题解答
