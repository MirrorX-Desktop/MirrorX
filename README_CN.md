<p align="center">
    <a href="https://github.com/MirrorX-Desktop/MirrorX"><img width="128" src="https://raw.githubusercontent.com/MirrorX-Desktop/MirrorX/master/mirrorx/src-tauri/assets/icons/icon.png"></a>
</p>

<h1 align="center" style="border-bottom: none">
    MirrorX</br>
</h1>

<p align="center">
面向企业、团队与个人的远程控制工具
</p>

<p align="center">
  <a href="https://github.com/MirrorX-Desktop/MirrorX"><img src="https://img.shields.io/github/stars/MirrorX-Desktop/MirrorX"></a>
  <a href="https://discord.gg/asT4deaEGh"><img src="https://img.shields.io/discord/1001077628238827620?label=Discord"></a>
  <a href="https://github.com/MirrorX-Desktop/MirrorX"><img src="https://img.shields.io/github/license/MirrorX-Desktop/MirrorX"></a>
</p>
  
<p align="center">
<img src="https://raw.githubusercontent.com/MirrorX-Desktop/MirrorX/master/screenshot1.png?" width="30%" height="30%">
<img src="https://raw.githubusercontent.com/MirrorX-Desktop/MirrorX/master/screenshot2.png?" width="30%" height="30%">
</p>

<p align="center">
    <a href="https://github.com/MirrorX-Desktop/MirrorX/blob/master/README.md">English</a>
    <a href="https://github.com/MirrorX-Desktop/MirrorX/blob/master/README_CN.md">简体中文</a>
<p align="center">

MirrorX 是一款面向企业、团队与个人用户的远程控制工具。配合完全开源的客户端与服务端、端到端加密与域支持等，无论你是团队管理者还是个人用户，都可以在短时间内构造一个快速且安全的远程控制网络，并且所有数据都处于用户控制之下。

> **MirrorX 项目还处于早期积极开发阶段。请原谅我们暂时还无法做出任何向后兼容的承诺。**

## 组件

- [MirrorX Client](https://github.com/MirrorX-Desktop/MirrorX)
- [MirrorX Signaling Server](https://github.com/MirrorX-Desktop/signaling)
- [MirrorX Endpoints Server](https://github.com/MirrorX-Desktop/endpoints)

## 免费公共服务器

> 这台服务器是我自费的，所以请不要滥用它。

| 位置 |      配置      |
| :--: | :------------: |
| 首尔 | 1vCPU & 1G RAM |

## 可用平台

- [x] macOS
- [x] Windows
- [ ] Linux (WIP)
- [ ] Android (WIP)
- [ ] iOS (WIP)
- [ ] Web (WIP)

## 如何构建

### 先决条件

1. 已安装 `nodejs && yarn(v3)` 。
2. 安装 `tauri-cli` 。

```console
cargo install tauri-cli
```

### 步骤

1. 从 [MirrorX-Desktop/media_libraries_auto_build](https://github.com/MirrorX-Desktop/media_libraries_auto_build) 下载预编译的多媒体库产物。
2. 解压多媒体库产物到任何你喜欢的路径。
3. **将刚才解压的多媒体库产物路径添加到环境变量中**

   - 对于 MacOS

     ```console
     $ export MIRRORX_MEDIA_LIBS_PATH=你的产物解压路径
     ```

   - 对于 Windows **(以管理员身份运行)**
     ```PowerShell
     PS > [Environment]::SetEnvironmentVariable('MIRRORX_MEDIA_LIBS_PATH', '你的产物解压路径' , 'Machine')
     ```

4. 以 Debug 模式运行

```console
cargo tauri dev
```

## 关于预编译的多媒体库

为了加速编译过程，我们建立了 [MirrorX-Desktop/media_libraries_auto_build](https://github.com/MirrorX-Desktop/media_libraries_auto_build) 来自动化和透明化构建这些依赖库。包括 [FFmpeg](https://git.ffmpeg.org/ffmpeg.git) 、libx264（[Windows](https://github.com/ShiftMediaProject/x264.git), [MacOS](https://code.videolan.org/videolan/x264.git)）、libx265（[Windows](https://github.com/ShiftMediaProject/x265.git), [MacOS](https://bitbucket.org/multicoreware/x265_git.git)）、libopus（[Windows](https://github.com/ShiftMediaProject/opus.git), [MacOS](https://github.com/xiph/opus.git)） 和 MFXDispatch（只用于 [Windows](https://github.com/ShiftMediaProject/mfx_dispatch.git)）。你可以在 [MirrorX-Desktop/media_libraries_auto_build](https://github.com/MirrorX-Desktop/media_libraries_auto_build) 浏览 [工作流](https://github.com/MirrorX-Desktop/media_libraries_auto_build/tree/main/.github/workflows) 以获取更多细节。

当然，你也完全可以根据我们的 [工作流](https://github.com/MirrorX-Desktop/media_libraries_auto_build/tree/main/.github/workflows) 来自行构建这些依赖库。

## 感谢

### 感谢那些令人惊叹的开源项目使得 MirrorX 得以成真。

（排名不分先后，仅列出部分项目，感谢所有在 Cargo.toml 和 package.json 中依赖的库的作者）

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
11. [ShiftMediaProject](https://github.com/ShiftMediaProject)
