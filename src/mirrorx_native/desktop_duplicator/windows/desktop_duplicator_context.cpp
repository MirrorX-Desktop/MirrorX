#include "desktop_duplicator_context.h"

void capture_frame( DesktopDuplicatorContext* context) {
  if (!context) {
    return;
  }

  context->manager->CaptureFrame(context->tx, context->callback);
}

void capture_frame_loop(DesktopDuplicatorContext* context) {
  while (context->running_sig.load(std::memory_order_seq_cst)) {
    capture_frame(context);
    this_thread::sleep_for(chrono::milliseconds(1));
  }
}

DesktopDuplicatorContext* desktop_duplicator_context_create(
    int display_index,
    void* tx,
    capture_session_callback callback) {
  auto* ctx = (DesktopDuplicatorContext*)malloc(sizeof(DesktopDuplicatorContext));
  auto* manager = new DuplicationManager();
  bool success = manager->Init(display_index);
  if (!success) {
    free(ctx);
    delete manager;
    return nullptr;
  }

  ctx->tx = tx;
  ctx->manager = manager;
  ctx->callback = callback;
  ctx->running_sig.store(true, std::memory_order_seq_cst);

  return ctx;
}

void desktop_duplicator_context_destory(DesktopDuplicatorContext* context) {
  if (!context) {
    return;
  }

  desktop_duplicator_context_stop_capture(context);
  delete context->manager;
  free(context);
}

void desktop_duplicator_context_start_capture(DesktopDuplicatorContext* context) {
  std::thread t(capture_frame_loop, context);
  t.detach();
}

void desktop_duplicator_context_stop_capture(DesktopDuplicatorContext* context) {
  context->running_sig.store(false, std::memory_order_seq_cst);
}