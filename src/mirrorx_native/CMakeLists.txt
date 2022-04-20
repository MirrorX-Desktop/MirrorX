cmake_minimum_required(VERSION 3.22.0)
project(mirrorx_native VERSION 0.1.0)

set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/build/lib)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/build/lib)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/build/bin)

# 3rd party libraries include directories
include_directories("${CMAKE_SOURCE_DIR}/../../dependencies_build/ffmpeg/include")
include_directories("${CMAKE_SOURCE_DIR}/../../dependencies_build/libvpx/include")
include_directories("${CMAKE_SOURCE_DIR}/../../dependencies_build/opus/include")
include_directories("${CMAKE_SOURCE_DIR}/../../dependencies_build/x264/include")
include_directories("${CMAKE_SOURCE_DIR}/../../dependencies_build/x265/include")
include_directories("${CMAKE_SOURCE_DIR}/../../dependencies_build/MediaSDK/include")

# 3rd party libraries binary file directories
find_library(LIB_VPX vpx PATHS "${CMAKE_SOURCE_DIR}/../../dependencies_build/libvpx/lib" "${CMAKE_SOURCE_DIR}/../../dependencies_build/libvpx/lib/x64" NO_CACHE REQUIRED NO_DEFAULT_PATH)
message("LIB_VPX: ${LIB_VPX}")

find_library(LIB_X264 x264 PATHS "${CMAKE_SOURCE_DIR}/../../dependencies_build/x264/lib" NO_CACHE REQUIRED NO_DEFAULT_PATH)
message("LIB_X264: ${LIB_X264}")

find_library(LIB_X265 x265 PATHS "${CMAKE_SOURCE_DIR}/../../dependencies_build/x265/lib" NO_CACHE REQUIRED NO_DEFAULT_PATH)
message("LIB_X265: ${LIB_X265}")

find_library(LIB_OPUS opus PATHS "${CMAKE_SOURCE_DIR}/../../dependencies_build/opus/lib" NO_CACHE REQUIRED NO_DEFAULT_PATH)
message("LIB_OPUS: ${LIB_OPUS}")

find_library(LIB_AVCODEC avcodec PATHS "${CMAKE_SOURCE_DIR}/../../dependencies_build/ffmpeg/lib" NO_CACHE REQUIRED NO_DEFAULT_PATH)
message("LIB_AVCODEC: ${LIB_AVCODEC}")

find_library(LIB_AVFORMAT avformat PATHS "${CMAKE_SOURCE_DIR}/../../dependencies_build/ffmpeg/lib" NO_CACHE REQUIRED NO_DEFAULT_PATH)
message("LIB_AVFORMAT: ${LIB_AVFORMAT}")

find_library(LIB_AVDEVICE avdevice PATHS "${CMAKE_SOURCE_DIR}/../../dependencies_build/ffmpeg/lib" NO_CACHE REQUIRED NO_DEFAULT_PATH)
message("LIB_AVDEVICE: ${LIB_AVDEVICE}")

find_library(LIB_AVUTIL avutil PATHS "${CMAKE_SOURCE_DIR}/../../dependencies_build/ffmpeg/lib" NO_CACHE REQUIRED NO_DEFAULT_PATH)
message("LIB_AVUTIL: ${LIB_AVUTIL}")

find_library(LIB_MFX mfx PATHS "${CMAKE_SOURCE_DIR}/../../dependencies_build/MediaSDK/lib")
message("LIB_MFX: ${LIB_MFX} (exclude macOS)")

# duplicator
if (WIN32 OR MSVC)
    aux_source_directory("duplicator/windows" DUPLICATOR_SOURCE)
elseif (APPLE)
    aux_source_directory("duplicator/macos" DUPLICATOR_SOURCE)
endif()
message("duplicator source: ${DUPLICATOR_SOURCE}")

# ffi_log
aux_source_directory(ffi_log FFI_LOG_SOURCE)
message("ffi_log source: ${FFI_LOG_SOURCE}")

# video_encoder
aux_source_directory(video_encoder VIDEO_ENCODER_SOURCE)
message("video_encoder source: ${VIDEO_ENCODER_SOURCE}")

# video_decoder
aux_source_directory(video_decoder VIDEO_DECODER_SOURCE)
message("video_decoder source: ${VIDEO_DECODER_SOURCE}")

add_library(mirrorx_native STATIC ${DUPLICATOR_SOURCE} ${FFI_LOG_SOURCE} ${VIDEO_ENCODER_SOURCE} ${VIDEO_DECODER_SOURCE})

# if (WIN32 OR MSVC)
# target_link_libraries(mirrorx_native ${LIB_VPX} ${LIB_X264} ${LIB_X265} ${LIB_OPUS} ${LIB_MFX} ${LIB_FFMPEG} )
# elseif(APPLE)
# target_link_libraries(mirrorx_native ${LIB_VPX} ${LIB_X264} ${LIB_X265} ${LIB_OPUS} ${LIB_FFMPEG})
# endif()

if(WIN32 OR MSVC)     
    # Use the static C library for all build types
    foreach(var 
        CMAKE_C_FLAGS CMAKE_C_FLAGS_DEBUG CMAKE_C_FLAGS_RELEASE
        CMAKE_C_FLAGS_MINSIZEREL CMAKE_C_FLAGS_RELWITHDEBINFO
        CMAKE_CXX_FLAGS CMAKE_CXX_FLAGS_DEBUG CMAKE_CXX_FLAGS_RELEASE
        CMAKE_CXX_FLAGS_MINSIZEREL CMAKE_CXX_FLAGS_RELWITHDEBINFO
      )
      if(${var} MATCHES "/MD")
        string(REGEX REPLACE "/MD" "/MT" ${var} "${${var}}")
      endif()
    endforeach()
endif()

if (APPLE)
    add_custom_command(TARGET ${PROJECT_NAME} POST_BUILD
    COMMAND xcrun -r libtool -static -a -o "${CMAKE_SOURCE_DIR}/build/lib/lib${PROJECT_NAME}.a" "${CMAKE_SOURCE_DIR}/build/lib/lib${PROJECT_NAME}.a" ${LIB_VPX} ${LIB_X264} ${LIB_X265} ${LIB_OPUS} ${LIB_AVFORMAT} ${LIB_AVDEVICE} ${LIB_AVUTIL} ${LIB_AVCODEC}
    DEPENDS ${LIB_VPX} ${LIB_X264} ${LIB_X265} ${LIB_OPUS} ${LIB_AVCODEC} ${LIB_AVFORMAT} ${LIB_AVDEVICE} ${LIB_AVUTIL})
elseif (WIN32 OR MSVC)
    add_custom_command(TARGET ${PROJECT_NAME} POST_BUILD
    COMMAND lib /nologo /out:"${CMAKE_SOURCE_DIR}/build/lib/Release/${PROJECT_NAME}.lib" "${CMAKE_SOURCE_DIR}/build/lib/Release/${PROJECT_NAME}.lib" ${LIB_VPX} ${LIB_X264} ${LIB_X265} ${LIB_OPUS} ${LIB_MFX} ${LIB_AVFORMAT} ${LIB_AVDEVICE} ${LIB_AVUTIL} ${LIB_AVCODEC}
    DEPENDS ${LIB_VPX} ${LIB_X264} ${LIB_X265} ${LIB_OPUS} ${LIB_MFX} ${LIB_AVCODEC} ${LIB_AVFORMAT} ${LIB_AVDEVICE} ${LIB_AVUTIL})
else ()
    add_custom_command(TARGET ${PROJECT_NAME} POST_BUILD
    COMMAND ar crsT ${CMAKE_BINARY_DIR}/lib${PROJECT_NAME}.a ${LIB_VPX} ${LIB_X264} ${LIB_X265} ${LIB_OPUS} ${LIB_FFMPEG}
    DEPENDS ${LIB_VPX} ${LIB_X264} ${LIB_X265} ${LIB_OPUS} ${LIB_FFMPEG})
endif()