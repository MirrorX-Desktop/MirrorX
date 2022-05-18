#ifndef LOG_H
#define LOG_H

#define MAX_LOG_STR_SIZE 512

#include <stdarg.h>
#include <stdio.h>

#ifdef __cplusplus
extern "C" {
#endif

#include <libavutil/log.h>

// log function from Rust
extern void log_to_rust(int level, const char* message);

enum RUST_LOGGER_LEVEL {
  LEVEL_TRACE = 1,
  LEVEL_DEBUG = 2,
  LEVEL_INFO = 3,
  LEVEL_WARN = 4,
  LEVEL_ERROR = 5,
};

void rust_log(enum RUST_LOGGER_LEVEL level, const char* format, ...);

void ffmpeg_log_callback(void* avcl, int level, const char* fmt, va_list vl);

#ifdef __cplusplus
};
#endif

#endif  // LOG_H
