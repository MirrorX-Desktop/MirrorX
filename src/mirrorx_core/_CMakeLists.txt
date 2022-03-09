# this cmake file is just for test and supports editor or IDE to provide intelligent code completion, we use rustc tool
# chain to compile those source files.

cmake_minimum_required(VERSION 3.21)

STRING(REGEX REPLACE "\\\\" "/" VCPKG_ROOT_DIR $ENV{VCPKG_ROOT})

project(mirrorx_core)

include_directories("dependencies/ffmpeg_build/include")

link_libraries("dependencies/ffmpeg_build/lib")

add_library(mirrorx_core STATIC
        export.h
        src/native/duplicator/duplicator.h
        src/native/duplicator/callback.h
        src/native/ffi_log/ffi_log.h
        src/native/ffi_log/ffi_log.c
        src/native/video_encoder/video_encoder.h
        src/native/video_encoder/video_encoder.c
        src/native/video_decoder/video_decoder.h
        src/native/video_decoder/video_decoder.c
        )

if (WIN32)
    target_sources(mirrorx_core PUBLIC
            src/native/duplicator/windows/common.h
            src/native/duplicator/windows/shaders/vertex_shader.h
            src/native/duplicator/windows/shaders/pixel_shader_y.h
            src/native/duplicator/windows/shaders/pixel_shader_uv.h
            src/native/duplicator/windows/DuplicationManager.cpp
            src/native/duplicator/windows/DuplicationManager.h
            src/native/duplicator/windows/DisplayOutput.h
            src/native/duplicator/windows/DisplayManager.h
            src/native/duplicator/windows/duplicator.cpp
            src/native/duplicator/windows/DisplayOutput.cpp
            src/native/duplicator/windows/DisplayManager.cpp
            )
    add_definitions(-D__STDC_CONSTANT_MACROS)
    set_target_properties(mirrorx_core PROPERTIES LINKER_LANGUAGE CXX)
elseif (APPLE)
    set(CMAKE_C_FLAGS "-x objective-c")
    target_sources(mirrorx_core PUBLIC src/native/duplicator/macos/duplicator.m)
    target_link_libraries(mirrorx_core "-framework AVFoundation")
elseif (LINUX)
    set(CMAKE_SOURCE_DIR "src/duplicator/linux")
endif ()