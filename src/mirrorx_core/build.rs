fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    build_native();
}

fn build_native() {
    #[cfg(target_os = "windows")]
    println!("cargo:rustc-link-search=../mirrorx_native/build/lib/Release");

    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-search=../mirrorx_native/build/lib");

    println!("cargo:rustc-link-lib=static=mirrorx_native");
}
