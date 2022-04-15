fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    build_native();
    build_proto();
}

fn build_native() {
    #[cfg(target_os = "windows")]
    println!("cargo:rustc-link-search=../../build/Release");

    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-search=../../build");

    println!("cargo:rustc-link-lib=static=mirrorx_native");
}

fn build_proto() {
    tonic_build::configure()
        .build_server(false)
        .compile(&["proto/desktop.proto", "proto/device.proto"], &["proto"])
        .unwrap();
}
