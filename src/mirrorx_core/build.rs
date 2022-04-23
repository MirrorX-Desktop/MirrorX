fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    build_native();
}

fn build_native() {
    #[cfg(target_os = "macos")]
    link_native_external_macos();

    #[cfg(target_os = "windows")]
    link_native_external_windows();
}

#[allow(dead_code)]
fn link_native_external_windows() {
    println!("cargo:rustc-link-search=../mirrorx_native/build/lib/Release");
    println!("cargo:rustc-link-search=../mirrorx_native/dependencies/windows/msvc/lib/x64");

    println!("cargo:rustc-link-lib=mirrorx_native");

    println!("cargo:rustc-link-lib=avutil");
    println!("cargo:rustc-link-lib=avcodec");
    println!("cargo:rustc-link-lib=avformat");
    println!("cargo:rustc-link-lib=avdevice");
}

#[allow(dead_code)]
fn link_native_external_macos() {
    println!("cargo:rustc-link-search=../mirrorx_native/build/lib");
    println!("cargo:rustc-link-lib=mirrorx_native");
    println!("cargo:rustc-link-lib=c++abi");
    println!("cargo:rustc-link-lib=framework=Foundation");
    println!("cargo:rustc-link-lib=framework=CoreVideo");
    println!("cargo:rustc-link-lib=framework=CoreMedia");
    println!("cargo:rustc-link-lib=framework=AVFoundation");
    println!("cargo:rustc-link-lib=framework=VideoToolbox");
}
