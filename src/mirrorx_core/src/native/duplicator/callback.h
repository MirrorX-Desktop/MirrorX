#ifndef CALLBACK_H
#define CALLBACK_H

#ifdef __cplusplus
extern "C" {
#endif

#ifndef __APPLE__
#include <cstdint>
#endif
typedef void (*capture_callback)(const void *tx, size_t width, size_t height,
                                 size_t y_line_size, uint8_t *y_buffer_address,
                                 size_t uv_line_size,
                                 uint8_t *uv_buffer_address);

#ifdef __cplusplus
};
#endif

#endif // CALLBACK_H
