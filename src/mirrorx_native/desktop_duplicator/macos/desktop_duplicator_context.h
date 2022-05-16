#ifndef CAPTURE_SESSION_CONTEXT_H
#define CAPTURE_SESSION_CONTEXT_H

#include <AVFoundation/AVFoundation.h>
#include "../../rust_log/rust_log.h"

typedef void (^capture_callback)(uint16_t width,
                                 uint16_t height,
                                 bool is_full_color_range,
                                 uint8_t* y_plane_buffer_address,
                                 uint32_t y_plane_stride,
                                 uint8_t* uv_plane_buffer_address,
                                 uint32_t uv_plane_stride,
                                 int64_t dts,
                                 int32_t dts_scale,
                                 int64_t pts,
                                 int32_t pts_scale);

@interface CaptureSessionContext
    : NSObject <AVCaptureVideoDataOutputSampleBufferDelegate>
- (id)init:(int)display_index fps:(int)fps callback:(capture_callback)callback;

- (void)startCapture;

- (void)stopCapture;

- (void)captureOutput:(AVCaptureOutput*)captureOutput
    didOutputSampleBuffer:(CMSampleBufferRef)videoFrame
           fromConnection:(AVCaptureConnection*)connection;
@end

#endif  // CAPTURE_SESSION_CONTEXT_H