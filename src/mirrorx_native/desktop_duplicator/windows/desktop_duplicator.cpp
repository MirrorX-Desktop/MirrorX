#include "../desktop_duplicator.h"

DesktopDuplicator* desktop_duplicator_create(
    int display_index,
    int fps,
    void* tx,
    capture_session_callback callback) {
  DesktopDuplicator* duplicator =
      (DesktopDuplicator*)malloc(sizeof(DesktopDuplicator));
  if (NULL == duplicator) {
    return NULL;
  }

  DesktopDuplicatorContext* ctx =
      desktop_duplicator_context_create(display_index, tx, callback);
  if (NULL == ctx) {
    desktop_duplicator_context_destory(ctx);
    free(duplicator);
    return NULL;
  }

  duplicator->ctx = ctx;

  return duplicator;
}

void desktop_duplicator_destroy(DesktopDuplicator* desktop_duplicator) {
  if (NULL == desktop_duplicator) {
    return;
  }

  if (NULL != desktop_duplicator->ctx) {
    desktop_duplicator_context_destory(desktop_duplicator->ctx);
    desktop_duplicator->ctx = NULL;
  }

  free(desktop_duplicator);
  return;
}

void desktop_duplicator_start(DesktopDuplicator* desktop_duplicator) {
  if (NULL == desktop_duplicator) {
    return;
  }

  desktop_duplicator_context_start_capture(desktop_duplicator->ctx);
  return;
}

void desktop_duplicator_stop(DesktopDuplicator* desktop_duplicator) {
  if (NULL == desktop_duplicator) {
    return;
  }

  desktop_duplicator_context_stop_capture(desktop_duplicator->ctx);
  return;
}