#!/bin/sh

if [ "$1" = "debug" ]; then
    echo "Building debug version"
    cargo make --cwd ./mirrorx_core --makefile MakeFile.toml make-debug
elif [ "$1" = "release" ]; then
    echo "Building release version"
    cargo make --cwd ./mirrorx_core --makefile MakeFile.toml make-release
fi

status=$?

if [ $status -ne 0 ]; then
    echo "Build failed"
    exit $status
fi

cp -f mirrorx_core/target/x86_64-apple-darwin/debug/libmirrorx_core.dylib app_plugin/mirrorx_sdk/macos/libmirrorx_core.dylib
flutter_rust_bridge_codegen --rust-input mirrorx_core/src/api.rs --dart-output app_plugin/mirrorx_sdk/lib/bridge_generated.dart --c-output app_plugin/mirrorx_sdk/macos/Classes/bridge_generated.h --class-name MirrorXCore