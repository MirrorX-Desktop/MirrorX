use cc::Build;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // let mut builder = cc::Build::new();
    // // builder.cpp(true).define("__STDC_CONSTANT_MACROS", None);
    // builder.flag("-mmacosx-version-min=10.11");

    // build_ffi_log(&mut builder);
    // link_library(&mut builder);
    // build_codec(&mut builder);
    // build_duplicator(&mut builder);

    // builder.compile("native");

    // println!("cargo:rustc-link-lib=native");
}

fn link_library(builder: &mut Build) {
    builder.include("dependencies/ffmpeg_build/include");

    println!("cargo:rustc-link-search=dependencies/ffmpeg_build/lib");
    println!("cargo:rustc-link-lib=avcodec");
    println!("cargo:rustc-link-lib=avutil");

    // println!("cargo:rustc-link-search=dependencies/x264_build/lib");
    // println!("cargo:rustc-link-lib=static=x264");

    // println!("cargo:rustc-link-search=dependencies/x265_build/lib");
    // println!("cargo:rustc-link-lib=static=x265");

    // println!("cargo:rustc-link-search=dependencies/opus_build/lib");
    // println!("cargo:rustc-link-lib=static=opus");

    // println!("cargo:rustc-link-search=dependencies/libvpx_build/lib");
    // println!("cargo:rustc-link-lib=static=vpx");

    // if cfg!(target_os = "windows") {
    //     vcpkg_bundle.find_package("ffnvcodec").unwrap();
    // }
}

fn build_codec(builder: &mut Build) {
    builder
        .include("src/native/video_encoder")
        .include("src/native/video_decoder")
        .file("src/native/video_encoder/video_encoder.c")
        .file("src/native/video_decoder/video_decoder.c");
}

fn build_duplicator(builder: &mut Build) {
    builder.include("src/native/duplicator");

    if cfg!(target_os = "windows") {
        builder
            .include("src/native/duplicator/windows")
            .include("src/native/duplicator/windows/shaders")
            .file("src/native/duplicator/windows/duplicator.cpp")
            .file("src/native/duplicator/windows/DuplicationManager.cpp")
            .file("src/native/duplicator/windows/DisplayManager.cpp")
            .file("src/native/duplicator/windows/DisplayOutput.cpp");
    } else if cfg!(target_os = "macos") {
        builder
            .flag("-fno-objc-arc")
            .file("src/native/duplicator/macos/duplicator.m");

        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-link-lib=framework=CoreMedia");
        println!("cargo:rustc-link-lib=framework=CoreVideo");
        println!("cargo:rustc-link-lib=framework=AVFoundation");
        println!("cargo:rustc-link-lib=framework=VideoToolBox");
        // println!("cargo:rustc-link-lib=framework=AudioToolBox");
    } else {
    }
}

fn build_ffi_log(builder: &mut Build) {
    builder
        .include("src/native/ffi_log")
        .file("src/native/ffi_log/ffi_log.c");
}
