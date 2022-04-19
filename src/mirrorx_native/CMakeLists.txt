cmake_minimum_required(VERSION 3.22.0)
project(mirrorx_native VERSION 0.1.0)

cmake_policy(SET CMP0069 NEW) 
set(CMAKE_POLICY_DEFAULT_CMP0069 NEW)

include(./bundle_static_library.cmake)

# check_ipo_supported(RESULT result OUTPUT output)
# if(result)
#   set_property(TARGET foo PROPERTY INTERPROCEDURAL_OPTIMIZATION TRUE)
# else()
#   message(WARNING "IPO is not supported: ${output}")
# endif()

# set_target_properties(mirrorx_native
#     PROPERTIES
#     CMAKE_MODULE_PATH ${PROJECT_SOURCE_DIR}/build_cache
#     LIBRARY_OUTPUT_DIRECTORY ${PROJECT_SOURCE_DIR}/build/lib
# )

set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/build/lib)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/build/lib)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/build/bin)

# 3rd party libraries include directories
include_directories("${CMAKE_SOURCE_DIR}/../../dependencies_build/ffmpeg/include")
include_directories("${CMAKE_SOURCE_DIR}/../../dependencies_build/libvpx/include")
include_directories("${CMAKE_SOURCE_DIR}/../../dependencies_build/opus/include")
include_directories("${CMAKE_SOURCE_DIR}/../../dependencies_build/x264/include")
include_directories("${CMAKE_SOURCE_DIR}/../../dependencies_build/x265/include")

if (WIN32 OR MSVC)
  include_directories("${CMAKE_SOURCE_DIR}/../../dependencies_build/MediaSDK/include")
endif()

# 3rd party libraries binary file directories
find_library(LIB_VPX vpx HINTS "${CMAKE_SOURCE_DIR}/../../dependencies_build/libvpx/lib/x64")
find_library(LIB_X264 libx264 HINTS "${CMAKE_SOURCE_DIR}/../../dependencies_build/x264/lib")
find_library(LIB_X265 x265 HINTS "${CMAKE_SOURCE_DIR}/../../dependencies_build/x265/lib")
find_library(LIB_OPUS opus HINTS "${CMAKE_SOURCE_DIR}/../../dependencies_build/opus/lib")
find_library(LIB_FFMPEG avcodec avdevice avformat avutil HINTS "${CMAKE_SOURCE_DIR}/../../dependencies_build/ffmpeg/lib")
if (WIN32 OR MSVC)
  find_library(LIB_MFX mfx "${CMAKE_SOURCE_DIR}/../../dependencies_build/MediaSDK/lib")
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

# set_target_properties(mirrorx_native
#     PROPERTIES
#     CMAKE_MODULE_PATH ${PROJECT_SOURCE_DIR}/build_cache
#     LIBRARY_OUTPUT_DIRECTORY ${PROJECT_SOURCE_DIR}/build/lib
# )

if (WIN32 OR MSVC)
target_link_libraries(mirrorx_native ${LIB_VPX} ${LIB_X264} ${LIB_X265} ${LIB_OPUS} ${FFMPEG} ${LIB_MFX})
elseif(APPLE)
target_link_libraries(mirrorx_native ${LIB_VPX} ${LIB_X264} ${LIB_X265} ${LIB_OPUS} ${FFMPEG})
endif()

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

# install(TARGETS mirrorx_native LIBRARY DESTINATION "${CMAKE_SOURCE_DIR}/../../dependencies_build/mirrorx_native/lib")

if (APPLE)
    add_custom_command(TARGET ${PROJECT_NAME} POST_BUILD
    COMMAND libtool -static -o ${CMAKE_BINARY_DIR}/lib${PROJECT_NAME}.a ${LIB_VPX} ${LIB_X264} ${LIB_X265} ${LIB_OPUS} ${FFMPEG}
    DEPENDS ${LIB_VPX} ${LIB_X264} ${LIB_X265} ${LIB_OPUS} ${FFMPEG})
elseif (WIN32 OR MSVC)
    add_custom_command(TARGET ${PROJECT_NAME} POST_BUILD
    COMMAND lib /nologo /out:"${CMAKE_SOURCE_DIR}/build/lib/Release/${PROJECT_NAME}.lib" "${CMAKE_SOURCE_DIR}/build/lib/Release/${PROJECT_NAME}.lib" ${LIB_VPX} ${LIB_X264} ${LIB_X265} ${LIB_OPUS} ${LIB_MFX} ${FFMPEG}
    DEPENDS ${LIB_VPX} ${LIB_X264} ${LIB_X265} ${LIB_OPUS} ${LIB_MFX} ${FFMPEG})
else ()
    add_custom_command(TARGET ${PROJECT_NAME} POST_BUILD
    COMMAND ar crsT ${CMAKE_BINARY_DIR}/lib${PROJECT_NAME}.a ${LIB_VPX} ${LIB_X264} ${LIB_X265} ${LIB_OPUS} ${FFMPEG}
    DEPENDS ${LIB_VPX} ${LIB_X264} ${LIB_X265} ${LIB_OPUS} ${FFMPEG})
endif()

# add_custom_target(_merge ALL DEPENDS ${PROJECT_NAME}.lib)