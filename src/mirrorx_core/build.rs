fn main() {
    // println!("cargo:rerun-if-changed=build.rs");
    // build_native();

    println!("enter test");

    link_ffmpeg();

    #[cfg(target_os = "macos")]
    link_native_macos();

    #[cfg(target_os = "windows")]
    link_native_windows();
}

#[allow(dead_code)]
fn link_native_windows() {
    println!("cargo:rustc-link-search=../mirrorx_native/dependencies/msvc/lib/x64");
    println!("cargo:rustc-link-lib=libx264");
    // println!("cargo:rustc-link-lib=static=libmfx");
    println!("cargo:rustc-link-lib=libavcodec");
    println!("cargo:rustc-link-lib=libavutil");
    println!("cargo:rustc-link-lib=libavformat");
    println!("cargo:rustc-link-lib=libavdevice");

    println!("cargo:rustc-link-search=../mirrorx_native/build/lib/Release");
    println!("cargo:rustc-link-lib=mirrorx_native");
}

#[allow(dead_code)]
fn link_native_macos() {
    println!("cargo:rustc-link-search=../mirrorx_native/build/lib");
    println!("cargo:rustc-link-lib=mirrorx_native");
    println!("cargo:rustc-link-lib=c++abi");
    println!("cargo:rustc-link-lib=framework=Foundation");
    println!("cargo:rustc-link-lib=framework=CoreVideo");
    println!("cargo:rustc-link-lib=framework=CoreMedia");
    println!("cargo:rustc-link-lib=framework=AVFoundation");
    println!("cargo:rustc-link-lib=framework=VideoToolbox");
}

fn link_ffmpeg() {
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-search=../third/dependencies_build/ffmpeg/lib");
        println!("cargo:rustc-link-lib=avcodec");
        println!("cargo:rustc-link-lib=avformat");
        println!("cargo:rustc-link-lib=avutil");
        println!("cargo:rustc-link-lib=avdevice");

        println!("cargo:rustc-link-search=../third/dependencies_build/x264/lib");
        println!("cargo:rustc-link-lib=x264");
    }
}
