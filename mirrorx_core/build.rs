fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    link_ffmpeg();
}

fn link_ffmpeg() {
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

        println!("cargo:rustc-link-search=../third/dependencies_build/ffmpeg/lib");
        println!("cargo:rustc-link-lib=avcodec");
        println!("cargo:rustc-link-lib=avformat");
        println!("cargo:rustc-link-lib=avutil");
        println!("cargo:rustc-link-lib=avdevice");

        println!("cargo:rustc-link-search=../third/dependencies_build/x264/lib");
        println!("cargo:rustc-link-lib=x264");

        println!("cargo:rustc-link-search=../third/dependencies_build/opus/lib");
        println!("cargo:rustc-link-lib=opus");

        println!("cargo:rustc-link-search=../third/dependencies_build/libyuv/lib");
        println!("cargo:rustc-link-lib=static=yuv");
    }

    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-search=../third/dependencies/msvc/lib/x64");
        println!("cargo:rustc-link-lib=libx264");
        println!("cargo:rustc-link-lib=libopus");
        println!("cargo:rustc-link-lib=libmfx");
        println!("cargo:rustc-link-lib=libavcodec");
        println!("cargo:rustc-link-lib=libavutil");
        println!("cargo:rustc-link-lib=libavformat");
        println!("cargo:rustc-link-lib=libavdevice");

        // println!("cargo:rustc-link-search=../third/dependencies/libyuv/Release");
        // println!("cargo:rustc-link-lib=yuv");

        // println!("cargo:rustc-link-search=../third/dependencies/opus_build/lib");
        // println!("cargo:rustc-link-lib=opus");
    }
}
