#include "desktop_duplicator_context.h"
#include "../desktop_duplicator.h"

DesktopDuplicator *desktop_duplicator_create(
        int display_index,
        int fps,
        void *tx,
        capture_session_callback callback) {

    DesktopDuplicator *desktop_duplicator =
            (DesktopDuplicator *) malloc(sizeof(DesktopDuplicator));
    memset(desktop_duplicator, 0, sizeof(DesktopDuplicator));
    desktop_duplicator->ctx = [DesktopDuplicatorContext alloc];

    BOOL success = [desktop_duplicator->ctx init:display_index fps:fps tx:tx callback:callback];

    if (!success) {
        desktop_duplicator_destroy(desktop_duplicator);
        return NULL;
    }

    return desktop_duplicator;
}

void desktop_duplicator_destroy(DesktopDuplicator *desktop_duplicator) {
    if (NULL == desktop_duplicator) {
        return;
    }

    if (NULL != desktop_duplicator->ctx) {
        [desktop_duplicator->ctx release];
        desktop_duplicator->ctx = NULL;
    }

    free(desktop_duplicator);
}

void desktop_duplicator_start(DesktopDuplicator *desktop_duplicator) {
    if (NULL == desktop_duplicator || NULL == desktop_duplicator->ctx) {
        return;
    }

    [desktop_duplicator->ctx startCapture];
}

void desktop_duplicator_stop(DesktopDuplicator *desktop_duplicator) {
    if (NULL == desktop_duplicator || NULL == desktop_duplicator->ctx) {
        return;
    }

    [desktop_duplicator->ctx stopCapture];
}