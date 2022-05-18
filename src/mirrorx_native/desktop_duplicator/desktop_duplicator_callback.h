#ifndef DESKTOP_DUPLICATOR_CALLBACK_H
#define DESKTOP_DUPLICATOR_CALLBACK_H

#ifdef __APPLE__

#include <stdbool.h>
#include <stdint.h>
#include <string.h>

#endif

#ifdef _MSC_VER

#include <stdint.h>
#include <stdbool.h>

#endif

typedef void (*capture_session_callback)(void* tx,
                                         uint16_t width,
                                         uint16_t height,
                                         bool is_full_color_range,
                                         uint8_t* y_plane_buffer_address,
                                         uint32_t y_plane_stride,
                                         uint8_t* uv_plane_buffer_address,
                                         uint32_t uv_plane_stride,
                                         int64_t dts,
                                         int32_t dts_scale,
                                         int64_t pts,
                                         int32_t pts_scale);

#endif  // DESKTOP_DUPLICATOR_CALLBACK_H