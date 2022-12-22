<p align="center">
    <a href="https://github.com/MirrorX-Desktop/MirrorX"><img width="128" src="https://raw.githubusercontent.com/MirrorX-Desktop/MirrorX/master/mirrorx/src-tauri/assets/icons/icon.png"></a>
</p>

<h1 align="center" style="border-bottom: none">
    MirrorX</br>
</h1>

<p align="center">
Secure, Fast and Light remote desktop & file manager tool
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

MirrorX is a remote desktop control tool powered by [Rust](https://github.com/rust-lang/rust). With fully open-source client and server, native E2EE support, users can build SECURITY and FAST remote control network, which is fully under control of users.

> **MirrorX is on the early stage with active developing now. Please forgive us that we cannot make any backward compatibility commitments at this time.**

## Component

- [MirrorX Client](https://github.com/MirrorX-Desktop/MirrorX)
- [MirrorX Signaling Server](https://github.com/MirrorX-Desktop/signaling)
- [MirrorX Endpoints Server](https://github.com/MirrorX-Desktop/endpoints)

## Free Public Servers

> This server is support at my own expense, so please do not abuse it.

| Location | Specification  |
| :------: | :------------: |
|  Seoul   | 1vCPU & 1G RAM |

## Available Platform

- [x] macOS
- [x] Windows
- [ ] Linux (WIP)
- [ ] Android (WIP)
- [ ] iOS (WIP)
- [ ] Web (WIP)

## How to build

### Prerequisite

1. Install `nodejs && yarn(v3)`.
2. Install `tauri-cli`.

```console
cargo install tauri-cli
```

### Steps

1. Download pre built media libraries artifacts from [MirrorX-Desktop/media_libraries_auto_build](https://github.com/MirrorX-Desktop/media_libraries_auto_build) Release.
2. Unzip artifacts to anywhere you'd like to put in.
3. **Add unzipped artifacts path to your Environment Variables**

   - For MacOS

     ```console
     $ export MIRRORX_MEDIA_LIBS_PATH=your artifacts unzip destination path
     ```

   - For Windows **(run As Administrator)**
     ```PowerShell
     PS > [Environment]::SetEnvironmentVariable('MIRRORX_MEDIA_LIBS_PATH', 'your artifacts unzip destination path' , 'Machine')
     ```

4. Run as Debug Mode

```console
cargo tauri dev
```

## About Pre Built Media Libraries

To speed up the build process, we made [MirrorX-Desktop/media_libraries_auto_build](https://github.com/MirrorX-Desktop/media_libraries_auto_build) to automatically and transparently build external libraries. Includes [FFmpeg](https://git.ffmpeg.org/ffmpeg.git), libx264([Windows](https://github.com/ShiftMediaProject/x264.git), [MacOS](https://code.videolan.org/videolan/x264.git)), libx265([Windows](https://github.com/ShiftMediaProject/x265.git), [MacOS](https://bitbucket.org/multicoreware/x265_git.git)), libopus([Windows](https://github.com/ShiftMediaProject/opus.git), [MacOS](https://github.com/xiph/opus.git)) and MFXDispatch([Windows](https://github.com/ShiftMediaProject/mfx_dispatch.git) only). For more details, you can look through [Workflows](https://github.com/MirrorX-Desktop/media_libraries_auto_build/tree/main/.github/workflows) on [MirrorX-Desktop/media_libraries_auto_build](https://github.com/MirrorX-Desktop/media_libraries_auto_build).

Of course, you can completely built those libraries by yourself according to our [Workflows](https://github.com/MirrorX-Desktop/media_libraries_auto_build/tree/main/.github/workflows).

## Thanks

### Thanks these awesome open source projects that make MirrorX becomes true.

(listed partial with in no particular order, thanks all the authors of dependencies in Cargo.toml and package.json)

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
