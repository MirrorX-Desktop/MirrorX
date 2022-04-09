cmake_minimum_required(VERSION 3.0.0)
project(mirrorx_native VERSION 0.1.0)

# include(CTest)
# enable_testing()

set(CPACK_PROJECT_NAME ${PROJECT_NAME})
set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})
# include(CPack)

# 3rd party libraries include directories
include_directories("../../dependencies_build/ffmpeg/include")
include_directories("../../dependencies_build/libvpx/include")
include_directories("../../dependencies_build/MediaSDK/include")
include_directories("../../dependencies_build/opus/include")
include_directories("../../dependencies_build/x264/include")
include_directories("../../dependencies_build/x265/include")

# 3rd party libraries binary file directories
find_library(LIB_FFMPEG "../../dependencies_build/ffmpeg/lib")
find_library(LIB_LIBVPX "../../dependencies_build/libvpx/lib/x64")
find_library(LIB_MEDIA_SDK "../../dependencies_build/MediaSDK/lib")
find_library(LIB_OPUS "../../dependencies_build/opus/lib")
find_library(LIB_X264 "../../dependencies_build/x264/lib")
find_library(LIB_X265 "../../dependencies_build/x265/lib")

# duplicator
add_subdirectory(./duplicator)

# ffi_log
aux_source_directory(ffi_log FFI_LOG_SOURCE)

# video_encoder
aux_source_directory(video_encoder VIDEO_ENCODER_SOURCE)

# video_decoder
aux_source_directory(video_decoder VIDEO_DECODER_SOURCE)

add_library(mirrorx_native STATIC ${DUPLICATOR_SOURCE} ${FFI_LOG_SOURCE} ${VIDEO_ENCODER_SOURCE} ${VIDEO_DECODER_SOURCE})

target_link_libraries(mirrorx_native ${LIB_LIBVPX} ${LIB_MEDIA_SDK} ${LIB_OPUS} ${LIB_X264} ${LIB_X265} ${LIB_FFMPEG})

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