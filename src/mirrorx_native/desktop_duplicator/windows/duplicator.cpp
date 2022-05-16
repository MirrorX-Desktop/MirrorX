#include "../include/duplicator.h"

const DuplicationContext* create_duplication_context(
    int display_index,
    void* tx,
    capture_callback callback) {
  auto* ctx = (DuplicationContext*)malloc(sizeof(DuplicationContext));
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

void release_duplication_context(DuplicationContext* context) {
  if (!context) {
    return;
  }

  stop_capture(context);
  delete context->manager;
  free(context);
}

void capture_frame(const DuplicationContext* context) {
  if (!context) {
    return;
  }

  context->manager->CaptureFrame(context->tx, context->callback);
}

void capture_frame_loop(DuplicationContext* context) {
  while (context->running_sig.load(std::memory_order_seq_cst)) {
    capture_frame(context);
    this_thread::sleep_for(chrono::milliseconds(1));
  }
}

void start_capture(DuplicationContext* context) {
  std::thread t(capture_frame_loop, context);
  t.detach();
}

void stop_capture(DuplicationContext* context) {
  context->running_sig.store(false, std::memory_order_seq_cst);
}