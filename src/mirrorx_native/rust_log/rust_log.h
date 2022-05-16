#ifndef LOG_H
#define LOG_H

#ifdef __cplusplus
extern "C" {
#endif

#include <libavutil/log.h>
#include <stdarg.h>
#include <stdio.h>

#define MAX_LOG_STR_SIZE 512

enum RUST_LOGGER_LEVEL {
  TRACE = 1,
  DEBUG = 2,
  INFO = 3,
  WARN = 4,
  ERROR = 5,
};

// log function from Rust
extern void log_to_rust(int level, const char* message);

void rust_log(enum RUST_LOGGER_LEVEL level, const char* format, ...);

void ffmpeg_log_callback(void* avcl, int level, const char* fmt, va_list vl);

#ifdef __cplusplus
};
#endif

#endif  // LOG_H
