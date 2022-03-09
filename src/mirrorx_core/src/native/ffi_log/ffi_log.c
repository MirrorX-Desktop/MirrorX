#include "ffi_log.h"

void ffi_log(enum FFI_LOG_LEVEL level, const char *format, ...) {
  char message_buffer[MAX_LOG_STR_SIZE] = {0};

  va_list args_ptr;
  va_start(args_ptr, format);

#if defined(__APPLE__)
  vsprintf(message_buffer, format, args_ptr);
#else
  vsprintf_s(message_buffer, MAX_LOG_STR_SIZE, format, args_ptr);
#endif

  log_to_rust(level, message_buffer);
  va_end(args_ptr);
}

void ffmpeg_log_callback(void *_, int level, const char *fmt, va_list vl) {
  if (level > av_log_get_level())
    return;

  char message_buffer[MAX_LOG_STR_SIZE] = {0};

#if defined(__APPLE__)
  vsprintf(message_buffer, fmt, vl);
#else
  vsprintf_s(message_buffer, MAX_LOG_STR_SIZE, fmt, vl);
#endif

  switch (level) {
  case AV_LOG_QUIET:
    return;
  case AV_LOG_TRACE:
    log_to_rust(FFI_LOG_TRACE, message_buffer);
    break;
  case AV_LOG_VERBOSE:
  case AV_LOG_DEBUG:
    log_to_rust(FFI_LOG_DEBUG, message_buffer);
    break;
  case AV_LOG_INFO:
    log_to_rust(FFI_LOG_INFO, message_buffer);
    break;
  case AV_LOG_WARNING:
    log_to_rust(FFI_LOG_WARN, message_buffer);
    break;
  case AV_LOG_ERROR:
  case AV_LOG_PANIC:
  case AV_LOG_FATAL:
    log_to_rust(FFI_LOG_ERROR, message_buffer);
    break;
  default:
    break;
  }
}
