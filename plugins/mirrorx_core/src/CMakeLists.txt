# The Flutter tooling requires that developers have CMake 3.10 or later
# installed. You should not increase this version, as doing so will cause
# the plugin to fail to compile for some customers of the plugin.
cmake_minimum_required(VERSION 3.10)

project(mirrorx_core_library VERSION 0.0.1 LANGUAGES C)

add_library(mirrorx_core SHARED mirrorx_core.c)

set_target_properties(mirrorx_core PROPERTIES
  PUBLIC_HEADER mirrorx_core.h
  OUTPUT_NAME "mirrorx_core"
)

include(FetchContent)

FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.2.1 # Optionally specify a commit hash, version tag or branch here
)

FetchContent_MakeAvailable(Corrosion)

corrosion_import_crate(MANIFEST_PATH ../../../../../../../core/Cargo.toml)

target_link_libraries(mirrorx_core PUBLIC core)

target_compile_definitions(mirrorx_core PUBLIC DART_SHARED_LIB)