---
title: "IntelliJ IDEA 安装配置 - Windows 一键安装 IDEA - hudo"
description: "使用 hudo 一键安装 IntelliJ IDEA（免费社区功能），免安装版解压即用，自动配置环境变量，Java/Kotlin 开发 IDE 开箱即用。"
head:
  - - meta
    - name: keywords
      content: "IntelliJ IDEA 安装, IDEA Community, Windows IDEA, Java IDE, JetBrains IDEA, IDEA 下载, hudo"
---

# IntelliJ IDEA

IntelliJ IDEA 是 JetBrains 出品的 Java/Kotlin 集成开发环境，也是 Stack Overflow 调查中最常用的 JVM IDE。自 2025.3 起社区版并入统一发行版，免费即可使用完整的 Java、Kotlin、Maven/Gradle 开发功能。

## 安装

```powershell
hudo install idea
```

安装到 `{install_root}\ide\idea\`，自动获取最新版本（下载包约 1.4GB，请耐心等待）。

## 安装后

```powershell
# 直接运行
{install_root}\ide\idea\bin\idea64.exe
```

## 卸载

```powershell
hudo uninstall idea
```

## 配置文件版本

```toml
[versions]
# 不填则自动获取最新版
idea = "2025.3"

[mirrors]
# 可选：替换 JetBrains 下载域（镜像站需保持相同路径结构）
# idea = "https://your-mirror.example.com"
```

## hudo 安装优势

- **自动获取最新版本**：下载链接直接取 JetBrains 官方 API，不受 2025.3 发行版命名换代影响
- **免安装版解压即用**：不需要运行安装向导，不写入系统注册表
- **版本可锁定**：通过[配置文件](/guide/config)指定版本号，适合团队统一开发环境
- **与 JVM 工具链联动**：配合 hudo 安装的 [JDK](/tools/jdk)、[Maven](/tools/maven)、[Gradle](/tools/gradle)，打开工程即可识别

## 常见问题

### 免费版够用吗？

2025.3 起 IntelliJ IDEA 为统一发行版，未激活订阅时即为免费层（原社区版能力）：Java/Kotlin 开发、调试、Maven/Gradle、Git 集成都可用。Spring 全家桶深度支持、数据库工具、Profiler 等需要付费订阅。

### IDEA 和 VS Code 应该选哪个？

主力写 Java/Kotlin 推荐 IDEA——补全、重构、构建工具集成明显更强；多语言混合开发或偏好轻量编辑器选 [VS Code](/tools/vscode)。

### JDK 没有被识别？

先用 `hudo install jdk` 安装 JDK（自动配置 `JAVA_HOME`），再在 IDEA 的 Project Structure → SDK 里选择 `{install_root}\lang\java`。
