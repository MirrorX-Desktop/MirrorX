#ifndef DESKTOP_DUPLICATOR_CONTEXT
#define DESKTOP_DUPLICATOR_CONTEXT

#include <atomic>
#include <thread>
#include "DuplicationManager.h"
#include "../../rust_log/rust_log.h"
#include "../desktop_duplicator_callback.h"

using namespace std;

typedef struct DesktopDuplicatorContext {
  DuplicationManager* manager;
  void* tx;
  capture_session_callback callback;
  std::atomic<bool> running_sig;
} DesktopDuplicatorContext;

DesktopDuplicatorContext* desktop_duplicator_context_create(
    int display_index,
    void* tx,
    capture_session_callback callback);

void desktop_duplicator_context_destory(DesktopDuplicatorContext* context);

void desktop_duplicator_context_start_capture(
    DesktopDuplicatorContext* context);

void desktop_duplicator_context_stop_capture(DesktopDuplicatorContext* context);

#endif  // DESKTOP_DUPLICATOR_CONTEXT
