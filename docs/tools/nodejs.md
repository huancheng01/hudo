# Node.js

JavaScript 运行时，直接安装官方预编译包。

## 安装

```powershell
hudo install nodejs
```

下载 Node.js 官方 zip 包并解压到 `{install_root}\lang\node\`，自动安装最新 LTS 版本。

## 安装后

安装完成后重新打开终端即可使用 `node`、`npm`、`npx` 命令，支持 CMD、PowerShell、Git Bash 等所有终端。

```powershell
node --version
npm --version
```

## 卸载

```powershell
hudo uninstall nodejs
```

## 配置

指定安装版本：

```toml
[versions]
nodejs = "24.14.1"
```

自定义下载镜像：

```toml
[mirrors]
nodejs = "https://npmmirror.com/mirrors/node/v24.14.1"
```
