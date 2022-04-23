#ifndef DUPLICATOR_H
#define DUPLICATOR_H

#include "../../ffi_log/ffi_log.h"
#include "callback.h"

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

#include <AVFoundation/AVFoundation.h>

@interface Duplicator : NSObject <AVCaptureVideoDataOutputSampleBufferDelegate>
- (id)init:(int)display_index
         fps:(int)fps
          tx:(const void*)tx
    callback:(capture_callback)callback;

- (void)startCapture;

- (void)stopCapture;

- (void)captureOutput:(AVCaptureOutput*)captureOutput
    didOutputSampleBuffer:(CMSampleBufferRef)videoFrame
           fromConnection:(AVCaptureConnection*)connection;
@end

typedef struct DuplicationContext {
  Duplicator* duplicator;
} DuplicationContext;
#elif __linux

#endif

#ifdef __cplusplus
extern "C" {
#endif

const DuplicationContext* create_duplication_context(int display_index,
                                                     void* tx,
                                                     capture_callback callback);

void release_duplication_context(DuplicationContext* context);

void start_capture(DuplicationContext* context);

void stop_capture(DuplicationContext* context);

#ifdef __cplusplus
};
#endif

#endif  // DUPLICATOR_H
