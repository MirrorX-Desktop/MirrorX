#!/bin/sh
echo "Generating Flutter and Rust bridge code"
flutter_rust_bridge_codegen \
    --dart-output plugins/mirrorx_core/lib/mirrorx_core.dart \
    --c-output plugins/mirrorx_core/src/mirrorx_core.h \
    --class-name MirrorXCore \
    --rust-input core/src/api/api.rs \
    --rust-output core/src/bridge.rs
