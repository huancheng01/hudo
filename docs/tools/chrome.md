---
title: "Chrome 安装 - Windows 一键静默安装 Google Chrome - hudo"
description: "使用 hudo 一键静默安装 Google Chrome 浏览器，企业版 MSI 安装包，无弹窗无广告，适合批量部署开发环境。"
head:
  - - meta
    - name: keywords
      content: "Chrome 安装, Google Chrome 下载, Chrome 静默安装, Windows Chrome, Chrome 企业版, 浏览器安装, hudo"
---

# Google Chrome

Google Chrome 是全球市场份额最高的浏览器，其内置的 DevTools 开发者工具是前端调试的行业标准。hudo 使用企业版 MSI 安装包进行静默安装，无弹窗、无捆绑软件，适合快速部署开发环境。

Google Chrome 浏览器，使用企业版 MSI 静默安装。

## 安装

```powershell
hudo install chrome
```

使用企业版 MSI 安装包，静默安装到系统目录（`%ProgramFiles%\Google\Chrome\`），需要 UAC 提权。

## 注意

- Chrome 不支持自定义安装路径，由 Google 安装程序决定
- 安装时会弹出 UAC 提示，点击「是」继续
- Chrome 不会添加到 PATH（不是命令行工具）

## 卸载

```powershell
hudo uninstall chrome
```

卸载时会自动调用 Chrome 内置卸载程序。若未找到，请通过「控制面板 → 程序」手动卸载。

## hudo 安装优势

- **企业版静默安装**：使用 Google 官方企业版 MSI 包，无弹窗、无捆绑软件、无推广页面
- **一条命令搞定**：不需要打开浏览器下载安装包、运行安装向导
- **适合批量部署**：搭建新开发环境时，一条命令即可安装好浏览器，配合其他 hudo 工具快速就绪
- **自动调用卸载程序**：卸载时不留残余

## 常见问题

### 为什么安装需要管理员权限？

Chrome 企业版 MSI 安装到 `Program Files` 系统目录，这是 Google 安装程序的限制，hudo 无法更改安装路径。

### 已经有 Chrome 了还需要通过 hudo 安装吗？

如果系统中已安装 Chrome，无需重复安装。hudo 的 Chrome 安装主要面向全新系统的快速环境搭建场景。

### Chrome 安装后可以正常自动更新吗？

可以。通过 hudo 安装的 Chrome 和手动安装的完全一致，Google 内置的自动更新机制正常工作。
