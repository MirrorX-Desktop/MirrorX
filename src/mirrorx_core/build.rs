fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-link-search=../../build/Release");
    println!("cargo:rustc-link-lib=static=mirrorx_native");
}
