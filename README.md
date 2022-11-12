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
<img src="https://raw.githubusercontent.com/MirrorX-Desktop/MirrorX/master/screenshot.png" width="50%" height="50%">
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

## Available Platform

- [x] macOS
- [x] Windows
- [ ] Linux
- [ ] Android
- [ ] iOS
- [ ] Web

## How to build

### Prerequisite

1. `nodejs && npm && yarn`
2. For Windows: `Visual Studio 2019+ && Desktop Development with C++ workloads`

### For Windows

1. Make sure you have installed Visual Studio 2019+ and install Desktop Development with C++ workloads.
2. Launch Developer PowerShell for VS with `Administorator priviliges`.
3. Switch terminal location to `MirrorX\third` and run PowerShell script:

```PowerShell
PS > Set-Location MirrorX\third
PS C:\MirrorX\third> .\build_dependencies.ps1
```

4. After script install and compile, install `tauri-cli`:

```PowerShell
PS > cargo install tauri-cli
```

5. Switch terminal location to MirrorX root dir and run:

```PowerShell
PS > cargo tauri dev
```

### For MacOS

1. Switch terminal location to `MirrorX/third` and run shell script:

```console
$ cd MirrorX/third
$ ./build_dependencies.sh
```

2. After script install and compile, install `tauri-cli`:

```console
$ cargo install tauri-cli
```

3. Switch terminal location to MirrorX root dir and run:

```console
$ cargo tauri dev
```

### Other Platforms

not support yet

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
