fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    build_native();
}

fn build_native() {
    // #[cfg(target_os = "windows")]
    // println!("cargo:rustc-link-search=../mirrorx_native/build/lib/Release");

    // #[cfg(target_os = "macos")]
    // println!("cargo:rustc-link-search=../mirrorx_native/build/lib");

    // println!("cargo:rustc-link-lib=static=mirrorx_native");

    #[cfg(target_os = "windows")]
    link_native_external_windows();
}

fn link_native_external_windows(){
    println!("cargo:rustc-link-search=../mirrorx_native/build/lib/Release");
    println!("cargo:rustc-link-search=../mirrorx_native/dependencies/windows/msvc/lib/x64");

    println!("cargo:rustc-link-lib=mirrorx_native");
    // println!("cargo:rustc-link-lib=x264");
    
    println!("cargo:rustc-link-lib=avutil");
    println!("cargo:rustc-link-lib=avcodec");
    println!("cargo:rustc-link-lib=avformat");
    println!("cargo:rustc-link-lib=avdevice");
}