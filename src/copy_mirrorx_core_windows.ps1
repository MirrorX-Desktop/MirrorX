$cmd_make_debug = "cargo make --cwd ./mirrorx_core --makefile MakeFile.toml make-debug"
$cmd_make_release = "cargo make --cwd ./mirrorx_core --makefile MakeFile.toml make-release"
$cmd_copy_debug_artifacts = "cp -Force mirrorx_core/target/x86_64-pc-windows-msvc/debug/mirrorx_core.dll app_plugin/mirrorx_sdk/windows/bin/mirrorx_core.dll"
$cmd_copy_release_artifacts = "cp -Force mirrorx_core/target/x86_64-pc-windows-msvc/release/mirrorx_core.dll app_plugin/mirrorx_sdk/windows/bin/mirrorx_core.dll"
$cmd_gen_bridge = "flutter_rust_bridge_codegen --rust-input mirrorx_core/src/api.rs --dart-output app_plugin/mirrorx_sdk/lib/bridge_generated.dart --c-output app_plugin/mirrorx_sdk/windows/include/mirrorx_sdk/bridge_generated.h --class-name MirrorXCore"

$cmd = $args[0]

if ($cmd -eq "debug") {
    Write-Output "Building debug version"

    Invoke-Expression -Command $cmd_gen_bridge

    $status = Invoke-Expression -Command $cmd_make_debug; $?

    if ( -not $status ) {
        Write-Output "Build failed"
        Exit-PSSession
    }

    Invoke-Expression -Command $cmd_copy_debug_artifacts
}
elseif ($cmd -eq "release") {
    Write-Output "Building release version"
    
    Invoke-Expression -Command $cmd_gen_bridge

    $status = Invoke-Expression -Command $cmd_make_release; $?

    if ( -not $status ) {
        Write-Output "Build failed"
        Exit-PSSession
    }

    Invoke-Expression -Command $cmd_copy_release_artifacts
    
}
elseif ($cmd -eq "gen_bridge") {
    Write-Output "Generating Flutter and Rust bridge code"
    Invoke-Expression -Command $cmd_gen_bridge
}