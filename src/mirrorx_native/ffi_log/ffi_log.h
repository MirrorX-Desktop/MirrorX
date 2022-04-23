#ifndef LOG_H
#define LOG_H

#ifdef __cplusplus
extern "C" {
#endif

#include <libavutil/log.h>
#include <stdarg.h>
#include <stdio.h>

#define MAX_LOG_STR_SIZE 512

enum FFI_LOG_LEVEL {
  FFI_LOG_TRACE = 1,
  FFI_LOG_DEBUG = 2,
  FFI_LOG_INFO = 3,
  FFI_LOG_WARN = 4,
  FFI_LOG_ERROR = 5,
};

// log function from Rust
extern void log_to_rust(int level, const char *message);

void ffi_log(enum FFI_LOG_LEVEL level, const char *format, ...);

void ffmpeg_log_callback(void *avcl, int level, const char *fmt, va_list vl);

#ifdef __cplusplus
};
#endif

#endif // LOG_H
