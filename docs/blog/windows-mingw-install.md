---
date: 2026-03-15
author: Zexa
title: 不装 Visual Studio 也能写 C/C++ — MinGW-w64 一键安装 + VS Code 配置全流程
description: Windows 上写 C/C++ 一定要装几十 GB 的 Visual Studio 吗？MinGW-w64 是轻量替代方案，但官网下载页让人头大。本文介绍用 hudo 一条命令安装 MinGW-w64 最新 GCC，自动配好 PATH，再搭配 VS Code 快速搭建 C/C++ 开发环境。
keywords:
  - Windows MinGW 安装
  - MinGW-w64 GCC
  - C++ 开发环境
  - VS Code C/C++ 配置
  - hudo
  - Windows GCC 安装
  - MinGW 下载
---

# 不装 Visual Studio 也能写 C/C++

在 Windows 上写 C/C++，大多数人的第一反应是装 Visual Studio。一个完整的 VS 安装动辄 20-30 GB，光是"使用 C++ 的桌面开发"工作负载就要好几个 GB。如果你只是想编译一个 `.c` 文件，或者学习算法刷题，这个成本未免太高了。

另一个选择是 MSVC Build Tools，虽然没有 IDE，但安装器同样庞大，而且命令行工具需要在特定的 Developer Command Prompt 中才能使用，对新手很不友好。

MinGW-w64 才是真正的轻量方案：一套完整的 GCC 工具链，解压即用，不依赖任何 IDE。

## MinGW 下载页的选择困难症

理论上，去官网下载 MinGW-w64 就行了。但实际操作时你会发现：

- **mingw.org 和 mingw-w64.org 是两个不同的项目。** 前者只支持 32 位，早已过时，但搜索排名靠前，很容易下错。
- **架构选择：** i686 还是 x86_64？现在几乎所有 Windows 电脑都是 64 位，应该选 x86_64，但页面上两个选项并排放着。
- **线程模型：** posix 还是 win32？如果你要用 `std::thread`，必须选 posix，选错了编译会报错。
- **异常处理：** seh 还是 sjlj？64 位系统应该选 seh，但这些术语对初学者毫无意义。
- **运行时库：** msvcrt 还是 ucrt？ucrt 是更现代的选择，但又多了一个决策点。

一个初学者想编译 Hello World，却要先搞懂四个技术选型。更别提从 SourceForge 下载速度感人，经常中断。

## 一条命令，跳过所有选择

[hudo](https://hudo.zexa.cc) 把这些决策都替你做好了：

```powershell
hudo install c
```

执行过程：

1. 自动下载最新的 winlibs GCC 构建（x86_64 + posix + seh + ucrt）
2. 解压到 `X:\hudo\tools\mingw64\`
3. 将 `mingw64\bin\` 写入用户 PATH 环境变量

不需要选架构，不需要选线程模型，不需要从 SourceForge 龟速下载。

## 验证安装

重新打开终端，运行：

```powershell
gcc --version
g++ --version
```

看到 GCC 版本号输出即安装成功。`gcc`、`g++`、`gdb`、`make` 等工具都可以直接使用。

快速测试编译：

```powershell
echo #include ^<stdio.h^> > hello.c && echo int main(){printf("Hello\n");return 0;} >> hello.c
gcc hello.c -o hello.exe && hello.exe
```

## 搭配 VS Code 使用

MinGW 提供编译器，VS Code 提供编辑体验，两者搭配是 Windows 上最轻量的 C/C++ 开发方案。

1. 安装 VS Code（也可以通过 `hudo install vscode`）
2. 在 VS Code 中安装 **C/C++** 扩展（微软官方，扩展 ID：`ms-vscode.cpptools`）
3. 打开你的项目文件夹，创建 `.vscode/tasks.json`：

```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "gcc build",
      "type": "shell",
      "command": "gcc",
      "args": ["-g", "${file}", "-o", "${fileDirname}/${fileBasenameNoExtension}.exe"],
      "group": { "kind": "build", "isDefault": true }
    }
  ]
}
```

之后按 `Ctrl+Shift+B` 即可编译当前文件。配合 C/C++ 扩展的调试功能，可以直接在 VS Code 里设断点、单步调试。

## 为什么 hudo 选择 winlibs 构建

市面上有多种 MinGW-w64 发行版：MSYS2 自带的、Cygwin 的、以及 winlibs 的独立构建。hudo 选择 winlibs 是因为：

- **版本最新：** winlibs 紧跟 GCC 上游发布，通常在 GCC 新版本发布后很快提供构建。
- **完全独立：** 不依赖 MSYS2 或任何外部运行时，解压就能用。
- **包含完整工具链：** GCC、G++、GDB、MinGW-w64 运行时、binutils 一应俱全。

这意味着你不需要先装 MSYS2 再在里面 `pacman -S mingw-w64-x86_64-gcc`，省去了一层包管理器的复杂度。

## 总结

Windows 上写 C/C++ 不一定要装 Visual Studio。MinGW-w64 提供了完整的 GCC 工具链，配合 VS Code 就是一套轻量高效的开发环境。唯一的门槛是下载和配置过程繁琐，而 `hudo install c` 把这个门槛降到了零：自动选择正确的版本、自动下载、自动配好环境变量，打开终端就能编译。

还没安装 hudo？一条命令即可：

```powershell
irm hudo.zexa.cc/install.ps1 | iex
```


---

> 查看 [MinGW 工具文档](/tools/mingw) 了解完整安装参数与配置选项。
