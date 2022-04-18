fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    build_native();
}

fn build_native() {
    #[cfg(target_os = "windows")]
    println!("cargo:rustc-link-search=../../build/Release");

    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-search=../../build");

    println!("cargo:rustc-link-lib=static=mirrorx_native");
}
