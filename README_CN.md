<h1 align="center" style="border-bottom: none">
    MirrorX</br>
</h1>

<p align="center">
安全、快速与轻量的远程桌面&文件管理工具
</p>

<p align="center">
  <a href="https://github.com/MirrorX-Desktop/MirrorX"><img src="https://img.shields.io/github/stars/MirrorX-Desktop/MirrorX"></a>
  <a href="https://discord.gg/asT4deaEGh"><img src="https://img.shields.io/discord/1001077628238827620?label=Discord"></a>
  <a href="https://github.com/MirrorX-Desktop/MirrorX"><img src="https://img.shields.io/github/license/MirrorX-Desktop/MirrorX"></a>
</p>
  
<p align="center">
<img src="https://raw.githubusercontent.com/MirrorX-Desktop/MirrorX/master/screenshot.png" width="50%" height="50%">
</p>

<p align="center">

[English](https://github.com/MirrorX-Desktop/MirrorX/blob/master/README.md)

[简体中文](https://github.com/MirrorX-Desktop/MirrorX/blob/master/README_CN.md)

<p align="center">

MirrorX 是一个使用 [Rust](https://github.com/rust-lang/rust) 构建的远程桌面控制软件。配合完全开源的客户端与服务端、原生支持的端到端加密，用户可以构造快速且安全的远程控制网络，并且完全处于用户控制之下。

> **MirrorX 项目还处于早期积极开发阶段。请原谅我们暂时还无法做出任何向后兼容的承诺**

## 组件

- [MirrorX Client](https://github.com/MirrorX-Desktop/MirrorX)
- [MirrorX Signaling Server](https://github.com/MirrorX-Desktop/signaling)
- [MirrorX Endpoints Server](https://github.com/MirrorX-Desktop/endpoints)

## 可用平台

- [x] macOS
- [x] Windows
- [ ] Linux
- [ ] Android
- [ ] iOS
- [ ] Web

## 如何构造

### 先决条件

1. `nodejs && npm && yarn`
2. 对于 Windows: `Visual Studio 2019+ && C++桌面开发`

### 对于 Windows

1. 请确保你已提前安装 Visual Studio 2019+ 和 C++桌面开发工作负载。
2. 以管理员身份运行 Developer PowerShell for VS。
3. 切换目录到 `MirrorX\third` 并运行 PowerShell 脚本：

```PowerShell
PS > Set-Location MirrorX\third
PS C:\MirrorX\third> .\build_dependencies.ps1
```

4. 在脚本安装与编译完成后，安装 `tauri-cli`:

```PowerShell
PS > cargo install tauri-cli
```

5. 切换目录到 MirrorX 的根目录并且运行：

```PowerShell
PS > cargo tauri dev
```

### 对于 MacOS

1. 切换目录到 `MirrorX/third` 并运行 shell 脚本：

```console
$ cd MirrorX/third
$ ./build_dependencies.sh
```

2. 在脚本安装与编译完成后，安装 `tauri-cli`：

```console
$ cargo install tauri-cli
```

3. 切换到 MirrorX 根目录并运行：

```console
$ cargo tauri dev
```

### 其它平台

暂不支持

## 感谢

### 感谢那些令人惊叹的开源项目使得 MirrorX 得以成真.

(排名不分先后，仅列出部分项目, 感谢所有在 Cargo.toml 和 package.json 中依赖的库的作者)

1. [Rust](https://github.com/rust-lang/rust)
2. [Tokio](https://github.com/tokio-rs/tokio)
3. [FFMPEG](https://ffmpeg.org)
4. [serde](https://github.com/serde-rs/serde)
5. [ring](https://github.com/briansmith/ring)
6. [egui](https://github.com/emilk/egui)
7. [windows-rs](https://github.com/microsoft/windows-rs)
8. [sveltekit](https://github.com/sveltejs/kit)
9. [daisyUI](https://github.com/saadeghi/daisyui)
10. [tailwindcss](https://github.com/tailwindlabs/tailwindcss)
