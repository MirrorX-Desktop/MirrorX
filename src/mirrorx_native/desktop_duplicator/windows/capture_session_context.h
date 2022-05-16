#ifndef CAPTURER_CONTEXT_H
#define CAPTURER_CONTEXT_H

#include "../ffi_log/ffi_log.h"
#include "capture_session_callback.h"

#if _WIN32 || _WIN64 || _MSC_VER || __MINGW32__ || __MINGW64__ || _WINDOWS

#include <atomic>
#include <thread>
#include "../windows/DuplicationManager.h"

typedef struct DuplicationContext {
  DuplicationManager* manager;
  void* tx;
  capture_callback callback;
  std::atomic<bool> running_sig;
} DuplicationContext;

#elif __APPLE__

#elif __linux
// todo:
#endif

#endif  // CAPTURER_CONTEXT_H
