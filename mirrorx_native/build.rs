use std::{ffi::OsStr, io, path::Path};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    link_sys();

    #[cfg(target_os = "macos")]
    {
        macos_build_libx264();
        macos_build_libopus();
        macos_build_libx265();
        macos_build_ffmpeg();
    }

    #[cfg(target_os = "windows")]
    {
        windows_build_ffmpeg();
    }
}

fn link_sys() {
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=framework=CoreVideo");
        println!("cargo:rustc-link-lib=framework=CoreMedia");
        println!("cargo:rustc-link-lib=framework=AVFoundation");
        println!("cargo:rustc-link-lib=framework=VideoToolbox");
        println!("cargo:rustc-link-lib=framework=ImageIO");
        println!("cargo:rustc-link-lib=framework=CoreServices");
        println!("cargo:rustc-link-lib=framework=AppKit");
        println!("cargo:rustc-link-lib=framework=IOSurface");
        println!("cargo:rustc-link-lib=c++");
    }

    #[cfg(target_os = "windows")]
    {
        // println!("cargo:rustc-link-search=./third/dependencies/msvc/lib/x64");
        // println!("cargo:rustc-link-lib=libx264");
        // println!("cargo:rustc-link-lib=libopus");
        // println!("cargo:rustc-link-lib=libmfx");
        // println!("cargo:rustc-link-lib=libavcodec");
        // println!("cargo:rustc-link-lib=libavutil");
        // println!("cargo:rustc-link-lib=libavformat");
        // println!("cargo:rustc-link-lib=libavdevice");

        // println!("cargo:rustc-link-search=./third/dependencies/libyuv/lib");
        // println!("cargo:rustc-link-lib=yuv");
    }
}

#[cfg(target_os = "macos")]
fn macos_build_libx264() {
    let output_dir = std::env::var_os("OUT_DIR").unwrap();
    let target_dir = Path::new(&output_dir).join("repo").join("x264");
    let artifacts_dir = Path::new(&output_dir).join("artifacts").join("x264");

    println!(
        "cargo:rustc-link-search={}",
        artifacts_dir.join("lib").display()
    );
    println!("cargo:rustc-link-lib=x264");
    println!("cargo:warning=build_libx264_output_dir: {:?}", output_dir);

    if artifacts_dir.exists() {
        return;
    }

    remove_dir_if_exists(&target_dir);

    run_command(
        "libx264::clone",
        "git",
        [
            "clone",
            "-b",
            "stable",
            "--depth=1",
            "https://code.videolan.org/videolan/x264.git",
            format!("{}", target_dir.display()).as_str(),
        ],
        None,
    );

    run_command(
        "libx264::configure",
        "sh",
        [
            "./configure",
            format!("--prefix={}", artifacts_dir.display()).as_str(),
            "--enable-pic",
            "--enable-static",
            "--enable-strip",
            "--disable-cli",
            "--disable-opencl",
        ],
        Some(&target_dir),
    );

    run_command("libx264::make", "make", [""], Some(&target_dir));
    run_command(
        "libx264::make-install",
        "make",
        ["install"],
        Some(&target_dir),
    );
    run_command("libx264::make-clean", "make", ["clean"], Some(&target_dir));
}

#[cfg(target_os = "macos")]
fn macos_build_libopus() {
    let output_dir = std::env::var_os("OUT_DIR").unwrap();
    let target_dir = Path::new(&output_dir).join("repo").join("opus");
    let artifacts_dir = Path::new(&output_dir).join("artifacts").join("opus");

    println!(
        "cargo:rustc-link-search={}",
        artifacts_dir.join("lib").display()
    );
    println!("cargo:rustc-link-lib=opus");
    println!("cargo:warning=build_libopus_output_dir: {:?}", output_dir);

    if artifacts_dir.exists() {
        return;
    }

    remove_dir_if_exists(&target_dir);

    run_command(
        "libopus::clone",
        "git",
        [
            "clone",
            "-b",
            "1.1.2",
            "--depth=1",
            "https://github.com/xiph/opus.git",
            format!("{}", target_dir.display()).as_str(),
        ],
        None,
    );

    run_command(
        "libopus::autogen",
        "sh",
        ["./autogen.sh"],
        Some(&target_dir),
    );

    run_command(
        "libopus::configure",
        "sh",
        [
            "./configure",
            "--host=x86_64",
            format!("--prefix={}", artifacts_dir.display()).as_str(),
            "--enable-static",
            "--disable-shared",
            "--disable-doc",
            "--disable-extra-programs",
        ],
        Some(&target_dir),
    );

    run_command("libopus::make", "make", [""], Some(&target_dir));
    run_command(
        "libopus::make-install",
        "make",
        ["install"],
        Some(&target_dir),
    );
    run_command("libopus::make-clean", "make", ["clean"], Some(&target_dir));
}

#[cfg(target_os = "macos")]
fn macos_build_libx265() {
    let output_dir = std::env::var_os("OUT_DIR").unwrap();
    let target_dir = Path::new(&output_dir).join("repo").join("x265");
    let artifacts_dir = Path::new(&output_dir).join("artifacts").join("x265");

    println!(
        "cargo:rustc-link-search={}",
        artifacts_dir.join("lib").display()
    );
    println!("cargo:rustc-link-lib=x265");
    println!("cargo:warning=build_libx265_output_dir: {:?}", output_dir);

    if artifacts_dir.exists() {
        return;
    }

    remove_dir_if_exists(&target_dir);

    run_command(
        "libx265::clone",
        "git",
        [
            "clone",
            "-b",
            "Release_3.5",
            "https://bitbucket.org/multicoreware/x265_git.git",
            format!("{}", target_dir.display()).as_str(),
        ],
        None,
    );

    run_command(
        "libx265::configure",
        "cmake",
        [
            "./source",
            format!("-DCMAKE_INSTALL_PREFIX={}", artifacts_dir.display()).as_str(),
            "-DENABLE_SHARED=OFF",
            "-DENABLE_CLI=OFF",
            "-DENABLE_PIC=ON",
        ],
        Some(&target_dir),
    );

    run_command(
        "libx265::make",
        "cmake",
        ["--build", ".", "--config", "Release"],
        Some(&target_dir),
    );

    run_command(
        "libx265::make-install",
        "cmake",
        ["--install", "."],
        Some(&target_dir),
    );
}

#[cfg(target_os = "macos")]
fn macos_build_ffmpeg() {
    let output_dir = std::env::var_os("OUT_DIR").unwrap();
    let target_dir = Path::new(&output_dir).join("repo").join("ffmpeg");
    let artifacts_dir = Path::new(&output_dir).join("artifacts").join("ffmpeg");
    let x264_pkg_config_dir = Path::new(&output_dir)
        .join("artifacts")
        .join("x264")
        .join("lib")
        .join("pkgconfig");
    let x265_pkg_config_dir = Path::new(&output_dir)
        .join("artifacts")
        .join("x265")
        .join("lib")
        .join("pkgconfig");

    println!(
        "cargo:rustc-link-search={}",
        artifacts_dir.join("lib").display()
    );
    println!("cargo:rustc-link-lib=avcodec");
    println!("cargo:rustc-link-lib=avformat");
    println!("cargo:rustc-link-lib=avutil");
    println!("cargo:rustc-link-lib=avdevice");
    println!("cargo:warning=build_ffmpeg_output_dir: {:?}", output_dir);

    if artifacts_dir.exists() {
        return;
    }

    remove_dir_if_exists(&target_dir);

    run_command(
        "ffmpeg::clone",
        "git",
        [
            "clone",
            "-b",
            "release/5.1",
            "--depth=1",
            "https://git.ffmpeg.org/ffmpeg.git",
            format!("{}", target_dir.display()).as_str(),
        ],
        None,
    );

    std::env::set_var(
        "PKG_CONFIG_PATH",
        format!(
            "$PKG_CONFIG_PATH:{}:{}",
            x264_pkg_config_dir.display(),
            x265_pkg_config_dir.display(),
        ),
    );

    run_command("ffmpeg::pkg-config", "pkg-config", ["--list-all"], None);

    run_command(
        "ffmpeg::configure",
        "sh",
        [
            "./configure",
            format!("--prefix={}", artifacts_dir.display()).as_str(),
            "--disable-all",
            "--disable-autodetect",
            "--arch=x86_64",
            "--pkg-config-flags=--static",
            "--enable-stripping",
            "--disable-debug",
            "--enable-pic",
            "--enable-hardcoded-tables",
            "--enable-gpl",
            "--enable-version3",
            "--enable-avdevice",
            "--enable-avcodec",
            "--enable-avformat",
            "--disable-doc",
            "--disable-htmlpages",
            "--disable-manpages",
            "--disable-podpages",
            "--disable-txtpages",
            "--disable-network",
            "--enable-libx264",
            "--enable-libx265",
            "--enable-videotoolbox",
            "--enable-audiotoolbox",
            "--enable-encoder=libx264",
            "--enable-encoder=libx265",
            "--enable-decoder=h264",
            "--enable-decoder=hevc",
            "--enable-encoder=h264_videotoolbox",
            "--enable-encoder=hevc_videotoolbox",
            "--enable-hwaccel=h264_videotoolbox",
            "--enable-hwaccel=hevc_videotoolbox",
            "--enable-hwaccel=vp9_videotoolbox",
            "--enable-parser=h264",
            "--enable-parser=hevc",
        ],
        Some(&target_dir),
    );

    run_command("ffmpeg::make", "make", [""], Some(&target_dir));
    run_command(
        "ffmpeg::make-install",
        "make",
        ["install"],
        Some(&target_dir),
    );
    run_command("ffmpeg::make-clean", "make", ["clean"], Some(&target_dir));
}

// #[cfg(target_os = "windows")]
fn windows_build_ffmpeg() {
    let output_dir = std::env::var_os("OUT_DIR").unwrap();
    let download_file_path = Path::new(&output_dir).join("ffmpeg-release-full-shared.7z");
    let artifacts_dir = Path::new(&output_dir).join("artifacts").join("ffmpeg");

    println!(
        "cargo:rustc-link-search={}",
        artifacts_dir.join("lib").display()
    );
    println!("cargo:rustc-link-lib=libavcodec");
    println!("cargo:rustc-link-lib=libavformat");
    println!("cargo:rustc-link-lib=libavutil");
    println!("cargo:rustc-link-lib=libavdevice");
    println!("cargo:warning=build_ffmpeg_output_dir: {:?}", output_dir);

    let mut resp =
        reqwest::blocking::get("https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-full-shared.7z")
            .unwrap_or_else(|err| panic!("request ffmpeg pre built binary failed: {:?}", err));

    let mut download_file = std::fs::File::create(&download_file_path)
        .unwrap_or_else(|err| panic!("create ffmpeg pre built binary file failed: {:?}", err));

    io::copy(&mut resp, &mut download_file)
        .unwrap_or_else(|err| panic!("write ffmpeg pre built binary file failed: {:?}", err));

    download_file
        .sync_all()
        .unwrap_or_else(|err| panic!("sync all to ffmpeg pre built binary file failed: {:?}", err));

    // close the file
    drop(download_file);

    sevenz_rust::decompress_file(&download_file_path, artifacts_dir)
        .unwrap_or_else(|err| panic!("decompress ffmpeg pre built binary file failed: {:?}", err));
}

fn run_command<P, I, S>(stage: &str, program: P, args: I, working_directory: Option<&Path>)
where
    P: AsRef<OsStr>,
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut command = std::process::Command::new(program);
    command.args(args);

    if let Some(wd) = working_directory {
        command.current_dir(wd);
    }

    let output = command
        .output()
        .unwrap_or_else(|_| panic!("stage: {} execute failed", stage));

    if !output.stdout.is_empty() {
        println!(
            "stage: {} stdout output: {}",
            stage,
            String::from_utf8_lossy(&output.stdout)
        );
    }

    if !output.stderr.is_empty() {
        eprintln!(
            "stage: {} stderr output: {}",
            stage,
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

fn remove_dir_if_exists(path: &Path) {
    if path.exists() {
        std::fs::remove_dir_all(path).unwrap_or_else(|err| {
            panic!("remove dir: {:?} and it's contents failed: {:?}", path, err)
        })
    }
}
