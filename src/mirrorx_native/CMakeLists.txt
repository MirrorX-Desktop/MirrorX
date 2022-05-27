cmake_minimum_required(VERSION 3.22.0)

if (WIN32 OR MSVC)
project(mirrorx_native VERSION 0.1.0 LANGUAGES C CXX)
set(CMAKE_CXX_FLAGS "-D__STDC_CONSTANT_MACROS")
else()
project(mirrorx_native VERSION 0.1.0 LANGUAGES C)
endif()

set(CMAKE_FIND_LIBRARY_SUFFIXES ".lib;.a")
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/build/lib)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/build/lib)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/build/bin)


# 3rd party libraries include directories
if (WIN32 OR MSVC)
include_directories("${CMAKE_SOURCE_DIR}/../third/dependencies/msvc/include")
else()
include_directories("${CMAKE_SOURCE_DIR}/../third/dependencies_build/ffmpeg/include")
# include_directories("${CMAKE_SOURCE_DIR}/dependencies_build/libvpx/include")
# include_directories("${CMAKE_SOURCE_DIR}/dependencies_build/opus/include")
include_directories("${CMAKE_SOURCE_DIR}/../third/dependencies_build/x264/include")
# include_directories("${CMAKE_SOURCE_DIR}/dependencies_build/x265/include")
include_directories("${CMAKE_SOURCE_DIR}/../third/dependencies_build/MediaSDK/include")
endif()

# # # libvpx
# # if (WIN32 OR MSVC)
# # find_library(LIB_VPX libvpx PATHS "${CMAKE_SOURCE_DIR}/dependencies/windows/msvc/lib/x64" NO_CACHE REQUIRED NO_DEFAULT_PATH)
# # else()
# # find_library(LIB_VPX vpx PATHS "${CMAKE_SOURCE_DIR}/dependencies_build/libvpx/lib" NO_CACHE REQUIRED NO_DEFAULT_PATH)
# # endif()
# # message("LIB_VPX: ${LIB_VPX}")

# # # opus
# # if (WIN32 OR MSVC)
# # find_library(LIB_OPUS libopus PATHS "${CMAKE_SOURCE_DIR}/dependencies/windows/msvc/lib/x64" NO_CACHE REQUIRED NO_DEFAULT_PATH)
# # else()
# # find_library(LIB_OPUS opus PATHS "${CMAKE_SOURCE_DIR}/dependencies_build/opus/lib" NO_CACHE REQUIRED NO_DEFAULT_PATH)
# # endif()
# # message("LIB_OPUS: ${LIB_OPUS}")

# # x264
# # if (WIN32 OR MSVC)
# find_library(LIB_X264 NAMES libx264 x264 PATHS "${CMAKE_SOURCE_DIR}/dependencies_build/x264/lib" NO_CACHE REQUIRED NO_DEFAULT_PATH)
# # else()
# # find_library(LIB_X264 x264 PATHS "${CMAKE_SOURCE_DIR}/dependencies_build/x264/lib" NO_CACHE REQUIRED NO_DEFAULT_PATH)
# # endif()
# message("LIB_X264: ${LIB_X264}")

# # # x265
# # if (WIN32 OR MSVC)
# # find_library(LIB_X265 libx265 PATHS "${CMAKE_SOURCE_DIR}/dependencies/windows/msvc/lib/x64" NO_CACHE REQUIRED NO_DEFAULT_PATH)
# # else()
# # find_library(LIB_X265 x265 PATHS "${CMAKE_SOURCE_DIR}/dependencies_build/x265/lib" NO_CACHE REQUIRED NO_DEFAULT_PATH)
# # endif()
# # message("LIB_X265: ${LIB_X265}")

# # mfx
# if (WIN32 OR MSVC)
# find_library(LIB_MFX mfx PATHS "${CMAKE_SOURCE_DIR}/dependencies_build/MediaSDK/lib" NO_CACHE REQUIRED NO_DEFAULT_PATH)
# message("LIB_MFX: ${LIB_MFX}")
# endif()

# # libavcodec
# # if (WIN32 OR MSVC)
# find_library(LIB_AVCODEC NAMES libavcodec avcodec PATHS "${CMAKE_SOURCE_DIR}/dependencies_build/ffmpeg/lib" NO_CACHE REQUIRED NO_DEFAULT_PATH)
# # else()
# # find_library(LIB_AVCODEC avcodec PATHS "${CMAKE_SOURCE_DIR}/dependencies_build/ffmpeg/lib" NO_CACHE REQUIRED NO_DEFAULT_PATH)
# # endif()
# message("LIB_AVCODEC: ${LIB_AVCODEC}")

# # libavdevice
# # if (WIN32 OR MSVC)
# find_library(LIB_AVDEVICE NAMES libavdevice avdevice PATHS "${CMAKE_SOURCE_DIR}/dependencies_build/ffmpeg/lib" NO_CACHE REQUIRED NO_DEFAULT_PATH)
# # else()
# # find_library(LIB_AVDEVICE avdevice PATHS "${CMAKE_SOURCE_DIR}/dependencies_build/ffmpeg/lib" NO_CACHE REQUIRED NO_DEFAULT_PATH)
# # endif()
# message("LIB_AVDEVICE: ${LIB_AVDEVICE}")

# # libavformat
# # if (WIN32 OR MSVC)
# find_library(LIB_AVFORMAT NAMES libavformat avformat PATHS "${CMAKE_SOURCE_DIR}/dependencies_build/ffmpeg/lib" NO_CACHE REQUIRED NO_DEFAULT_PATH)
# # else()
# # find_library(LIB_AVFORMAT avformat PATHS "${CMAKE_SOURCE_DIR}/dependencies_build/ffmpeg/lib" NO_CACHE REQUIRED NO_DEFAULT_PATH)
# # endif()
# message("LIB_AVFORMAT: ${LIB_AVFORMAT}")

# # libavutil
# # if (WIN32 OR MSVC)
# find_library(LIB_AVUTIL NAMES libavutil avutil PATHS "${CMAKE_SOURCE_DIR}/dependencies_build/ffmpeg/lib" NO_CACHE REQUIRED NO_DEFAULT_PATH)
# # else()
# # find_library(LIB_AVUTIL avutil PATHS "${CMAKE_SOURCE_DIR}/dependencies_build/ffmpeg/lib" NO_CACHE REQUIRED NO_DEFAULT_PATH)
# # endif()
# message("LIB_AVUTIL: ${LIB_AVUTIL}")

# include_directories("duplicator/include")
# include_directories("ffi_log")
# include_directories("video_decoder")
# include_directories("video_encoder")

set(SOURCES "")

# rust_log
list(APPEND SOURCES "rust_log/rust_log.h")
list(APPEND SOURCES "rust_log/rust_log.c")
list(APPEND SOURCES "rust_log/test.m")

# video_encoder
list(APPEND SOURCES "video_encoder/video_encoder.h")
list(APPEND SOURCES "video_encoder/video_encoder.c")

# video_decoder
list(APPEND SOURCES "video_decoder/video_decoder.h")
list(APPEND SOURCES "video_decoder/video_decoder.c")

message("SOURCES:")
message("${SOURCES}")

# set_source_files_properties(${SOURCES} PROPERTIES LANGUAGE CXX)

add_library(mirrorx_native STATIC ${SOURCES})

# if (APPLE)
# # ${LIB_VPX} ${LIB_X264} ${LIB_X265} ${LIB_OPUS}
#     add_custom_command(TARGET ${PROJECT_NAME} POST_BUILD
#     COMMAND xcrun -r libtool -static -a -o "${CMAKE_SOURCE_DIR}/build/lib/lib${PROJECT_NAME}.a" "${CMAKE_SOURCE_DIR}/build/lib/lib${PROJECT_NAME}.a" ${LIB_AVFORMAT} ${LIB_AVDEVICE} ${LIB_AVUTIL} ${LIB_AVCODEC} ${LIB_X264}
#     DEPENDS ${LIB_AVCODEC} ${LIB_AVFORMAT} ${LIB_AVDEVICE} ${LIB_AVUTIL} ${LIB_X264})
# elseif (UNIX)
#     add_custom_command(TARGET ${PROJECT_NAME} POST_BUILD
#     COMMAND ar crsT ${CMAKE_BINARY_DIR}/lib${PROJECT_NAME}.a ${LIB_VPX} ${LIB_X264} ${LIB_X265} ${LIB_OPUS} ${LIB_FFMPEG}
#     DEPENDS ${LIB_VPX} ${LIB_X264} ${LIB_X265} ${LIB_OPUS} ${LIB_FFMPEG})
# else()
#     add_custom_command(TARGET ${PROJECT_NAME} POST_BUILD
#     COMMAND lib.exe /out:${CMAKE_LIBRARY_OUTPUT_DIRECTORY}/Release/${PROJECT_NAME}.lib "${CMAKE_LIBRARY_OUTPUT_DIRECTORY}/Release/${PROJECT_NAME}.lib" ${LIB_AVFORMAT} ${LIB_AVDEVICE} ${LIB_AVUTIL} ${LIB_AVCODEC} ${LIB_X264} ${LIB_MFX}
#     DEPENDS ${LIB_AVCODEC} ${LIB_AVFORMAT} ${LIB_AVDEVICE} ${LIB_AVUTIL} ${LIB_X264} ${LIB_MFX})
# endif()