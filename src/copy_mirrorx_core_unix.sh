#!/bin/sh

cmd_make_debug="cargo make --cwd ./mirrorx_core --makefile MakeFile.toml make-debug"
cmd_make_release="cargo make --cwd ./mirrorx_core --makefile MakeFile.toml make-release"
cmd_copy_debug_artifacts="cp -f mirrorx_core/target/x86_64-apple-darwin/debug/libmirrorx_core.dylib app_plugin/mirrorx_sdk/macos/libmirrorx_core.dylib"
cmd_copy_release_artifacts="cp -f mirrorx_core/target/x86_64-apple-darwin/release/libmirrorx_core.dylib app_plugin/mirrorx_sdk/macos/libmirrorx_core.dylib"
cmd_gen_bridge="flutter_rust_bridge_codegen --rust-crate-dir mirrorx_core --rust-input mirrorx_core/src/api/mod.rs --dart-output app_plugin/mirrorx_sdk/lib/bridge_generated.dart --c-output app_plugin/mirrorx_sdk/macos/Classes/bridge_generated.h --class-name MirrorXCore"

status=$?

if [ "$1" = "debug" ]; then
    echo "Building debug version"
    $cmd_gen_bridge

    $cmd_make_debug

    if [ $status -ne 0 ]; then
        echo "Build failed"
        exit $status
    fi

    $cmd_copy_debug_artifacts
elif [ "$1" = "release" ]; then
    echo "Building release version"
    $cmd_gen_bridge

    $cmd_make_release

    if [ $status -ne 0 ]; then
        echo "Build failed"
        exit $status
    fi

    $cmd_copy_release_artifacts
elif [ "$1" = "gen_bridge" ]; then
    echo "Generating Flutter and Rust bridge code"
    flutter_rust_bridge_codegen \
        --dart-output app/lib/plugin/bridge_generated.dart \
        --c-output app/macos/Runner/bridge_generated.h \
        --class-name MirrorXCore \
        --rust-input mirrorx_core/src/api/api.rs \
        --rust-output mirrorx_core/src/bridge.rs
fi
