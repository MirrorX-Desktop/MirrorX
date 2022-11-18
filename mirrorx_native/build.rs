use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    link_sys();
    link_media_libraries_artifacts();
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
}

#[cfg(target_os = "macos")]
fn link_media_libraries_artifacts() {
    let mirrorx_media_libraries_path = match std::env::var("MIRRORX_MEDIA_LIBS_PATH") {
        Ok(path) => PathBuf::from(path),
        Err(_) => panic!("environment variable 'MIRRORX_MEDIA_LIBS_PATH' not exists"),
    };

    println!(
        "cargo:rustc-link-search={}",
        mirrorx_media_libraries_path
            .join("x264")
            .join("lib")
            .display()
    );
    println!("cargo:rustc-link-lib=x264");

    println!(
        "cargo:rustc-link-search={}",
        mirrorx_media_libraries_path
            .join("x265")
            .join("lib")
            .display()
    );
    println!("cargo:rustc-link-lib=x265");

    println!(
        "cargo:rustc-link-search={}",
        mirrorx_media_libraries_path
            .join("opus")
            .join("lib")
            .display()
    );
    println!("cargo:rustc-link-lib=opus");

    println!(
        "cargo:rustc-link-search={}",
        mirrorx_media_libraries_path
            .join("ffmpeg")
            .join("lib")
            .display()
    );
    println!("cargo:rustc-link-lib=avcodec");
    println!("cargo:rustc-link-lib=avutil");
    println!("cargo:rustc-link-lib=avformat");
    println!("cargo:rustc-link-lib=avdevice");
    println!("cargo:rustc-link-lib=swresample");
}

#[cfg(target_os = "windows")]
fn link_media_libraries_artifacts() {
    let mirrorx_media_libraries_path = match std::env::var("MIRRORX_MEDIA_LIBS_PATH") {
        Ok(path) => PathBuf::from(path),
        Err(_) => panic!("environment variable 'MIRRORX_MEDIA_LIBS_PATH' not exists"),
    };

    println!(
        "cargo:rustc-link-search={}",
        mirrorx_media_libraries_path
            .join("lib")
            .join("x64")
            .display()
    );
    println!("cargo:rustc-link-lib=libx264");
    println!("cargo:rustc-link-lib=libx265");
    println!("cargo:rustc-link-lib=libopus");
    println!("cargo:rustc-link-lib=libmfx");
    println!("cargo:rustc-link-lib=libavcodec");
    println!("cargo:rustc-link-lib=libavutil");
    println!("cargo:rustc-link-lib=libavformat");
    println!("cargo:rustc-link-lib=libavdevice");
    println!("cargo:rustc-link-lib=libswresample");
}
