cmake_minimum_required(VERSION 3.0.0)
project(mirrorx_native VERSION 0.1.0)

# include(CTest)
# enable_testing()

set(CPACK_PROJECT_NAME ${PROJECT_NAME})
set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})
# include(CPack)

# 3rd party libraries include directories
include_directories("${CMAKE_SOURCE_DIR}/../../dependencies_build/ffmpeg/include")
include_directories("${CMAKE_SOURCE_DIR}/../../dependencies_build/libvpx/include")
include_directories("${CMAKE_SOURCE_DIR}/../../dependencies_build/opus/include")
include_directories("${CMAKE_SOURCE_DIR}/../../dependencies_build/x264/include")
include_directories("${CMAKE_SOURCE_DIR}/../../dependencies_build/x265/include")

if (MSVC)
  include_directories("${CMAKE_SOURCE_DIR}/../../dependencies_build/MediaSDK/include")
endif()

# 3rd party libraries binary file directories
find_library(LIB_FFMPEG avcodec avdevice avformat avutil "${CMAKE_SOURCE_DIR}/../../dependencies_build/ffmpeg/lib")
find_library(LIB_OPUS opus "${CMAKE_SOURCE_DIR}/../../dependencies_build/opus/lib")
find_library(LIB_X264 x264 "${CMAKE_SOURCE_DIR}/../../dependencies_build/x264/lib")
find_library(LIB_X265 x265 "${CMAKE_SOURCE_DIR}/../../dependencies_build/x265/lib")

if (MSVC)
  find_library(LIB_LIBVPX vpx "${CMAKE_SOURCE_DIR}/../../dependencies_build/libvpx/lib/x64")
  find_library(LIB_MEDIA_SDK mfx "${CMAKE_SOURCE_DIR}/../../dependencies_build/MediaSDK/lib")
elseif(APPLE)
  find_library(LIB_LIBVPX vpx "${CMAKE_SOURCE_DIR}/../../dependencies_build/libvpx/lib")
endif()

# duplicator
add_subdirectory(./duplicator)

# ffi_log
aux_source_directory(ffi_log FFI_LOG_SOURCE)

# video_encoder
aux_source_directory(video_encoder VIDEO_ENCODER_SOURCE)

# video_decoder
aux_source_directory(video_decoder VIDEO_DECODER_SOURCE)

add_library(mirrorx_native STATIC ${DUPLICATOR_SOURCE} ${FFI_LOG_SOURCE} ${VIDEO_ENCODER_SOURCE} ${VIDEO_DECODER_SOURCE})

if (MSVC)
target_link_libraries(mirrorx_native ${LIB_LIBVPX} ${LIB_MEDIA_SDK} ${LIB_OPUS} ${LIB_X264} ${LIB_X265} ${LIB_FFMPEG})
elseif(APPLE)
target_link_libraries(mirrorx_native ${LIB_LIBVPX} ${LIB_OPUS} ${LIB_X264} ${LIB_X265} ${LIB_FFMPEG})
endif()

if(MSVC)     
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
endif(MSVC)