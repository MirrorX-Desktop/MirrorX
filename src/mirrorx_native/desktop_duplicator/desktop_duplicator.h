#ifndef DESKTOP_DUPLICATOR_H
#define DESKTOP_DUPLICATOR_H

#include "../rust_log/rust_log.h"
#include "desktop_duplicator_callback.h"

#ifdef __APPLE__

#include "macos/desktop_duplicator_context.h"

#elif defined(_MSC_VER)

#include "windows/desktop_duplicator_context.h"

#endif

#ifdef __cplusplus
extern "C" {
#endif

typedef struct DesktopDuplicator {
  DesktopDuplicatorContext* ctx;
} DesktopDuplicator;

DesktopDuplicator* desktop_duplicator_create(int display_index,
                                             int fps,
                                             void* tx,
                                             capture_session_callback callback);

void desktop_duplicator_destroy(DesktopDuplicator* desktop_duplicator);

void desktop_duplicator_start(DesktopDuplicator* desktop_duplicator);

void desktop_duplicator_stop(DesktopDuplicator* desktop_duplicator);

#ifdef __cplusplus
};
#endif

#endif  // DESKTOP_DUPLICATOR_H