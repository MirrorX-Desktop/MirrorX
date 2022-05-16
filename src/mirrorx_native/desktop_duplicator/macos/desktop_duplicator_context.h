#ifndef CAPTURE_SESSION_CONTEXT_H
#define CAPTURE_SESSION_CONTEXT_H

#include <AVFoundation/AVFoundation.h>
#include "../desktop_duplicator_callback.h"
#include "../../rust_log/rust_log.h"

@interface DesktopDuplicatorContext
    : NSObject <AVCaptureVideoDataOutputSampleBufferDelegate>
- (BOOL)init:(int)display_index
         fps:(int)fps
          tx:(void*)tx
    callback:(capture_session_callback)callback;

- (void)startCapture;

- (void)stopCapture;

- (void)captureOutput:(AVCaptureOutput*)captureOutput
    didOutputSampleBuffer:(CMSampleBufferRef)videoFrame
           fromConnection:(AVCaptureConnection*)connection;
@end

#endif  // CAPTURE_SESSION_CONTEXT_H