#!/bin/sh
echo "Generating Flutter and Rust bridge code"
flutter_rust_bridge_codegen \
    --dart-output mirrorx/lib/env/sdk/mirrorx_core.dart \
    --c-output mirrorx/macos/Runner/mirrorx_core.h \
    --class-name MirrorXCore \
    --rust-input mirrorx_core/src/api.rs \
    --rust-output mirrorx_core/src/bridge.rs
