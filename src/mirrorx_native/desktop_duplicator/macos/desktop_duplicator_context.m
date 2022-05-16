#include "desktop_duplicator_context.h"

@implementation DesktopDuplicatorContext {
    AVCaptureSession *_capture_session;
    AVCaptureScreenInput *_capture_screen_input;
    AVCaptureVideoDataOutput *_capture_video_data_output;
    int _display_index;
    int _fps;
    void *_tx;
    capture_session_callback _callback;
}

- (BOOL)init:(int)display_index
         fps:(int)fps
          tx:(void *)tx
    callback:(capture_session_callback)callback {
    self = [super init];
    if (self) {
        _display_index = display_index;
        _fps = fps;
        _tx = tx;
        _callback = callback;
    }

    AVCaptureSession *capture_session = [[AVCaptureSession alloc] init];
    [capture_session beginConfiguration];
    [capture_session setSessionPreset:AVCaptureSessionPresetHigh];

    AVCaptureScreenInput *capture_screen_input =
            [[AVCaptureScreenInput alloc] initWithDisplayID:(_display_index)];
    capture_screen_input.capturesCursor = YES;
    capture_screen_input.capturesMouseClicks = YES;
    capture_screen_input.minFrameDuration = CMTimeMake(1, _fps);
    //  capture_screen_input.scaleFactor = 0.25;
    if ([capture_session canAddInput:capture_screen_input]) {
        [capture_session addInput:capture_screen_input];
    } else {
        return NO;
    }

    AVCaptureVideoDataOutput *capture_video_data_output =
            [[AVCaptureVideoDataOutput alloc] init];

    dispatch_queue_t delegate_queue =
            dispatch_queue_create("cloud.mirrorx.dispatch.capture_session", NULL);
    [capture_video_data_output setSampleBufferDelegate:self queue:delegate_queue];
    dispatch_release(delegate_queue);

    capture_video_data_output.alwaysDiscardsLateVideoFrames = YES;
    capture_video_data_output.videoSettings = @{
            (NSString *) kCVPixelBufferPixelFormatTypeKey:
            @(kCVPixelFormatType_420YpCbCr8BiPlanarFullRange),
            (NSString *) kCVPixelBufferWidthKey: @(1920),
            (NSString *) kCVPixelBufferHeightKey: @(1080)
    };
    if ([capture_session canAddOutput:(capture_video_data_output)]) {
        [capture_session addOutput:(capture_video_data_output)];
    } else {
        return NO;
    }

    [capture_session commitConfiguration];

    _capture_session = capture_session;
    _capture_screen_input = capture_screen_input;
    _capture_video_data_output = capture_video_data_output;

    return YES;
}

- (void)startCapture {
    dispatch_async(dispatch_get_global_queue(DISPATCH_QUEUE_PRIORITY_HIGH, 0), ^{
        [_capture_session startRunning];
    });
}

- (void)stopCapture {
    [_capture_session stopRunning];
}

- (void)captureOutput:(AVCaptureOutput *)captureOutput
didOutputSampleBuffer:(CMSampleBufferRef)videoFrame
       fromConnection:(AVCaptureConnection *)connection {
    if (!CMSampleBufferIsValid(videoFrame)) {
        return;
    }

    CMSampleTimingInfo timing_info;
    if (0 != CMSampleBufferGetSampleTimingInfo(videoFrame, 0, &timing_info)) {
        return;
    }

    CVImageBufferRef image_buffer = CMSampleBufferGetImageBuffer(videoFrame);
    if (image_buffer == nil) {
        return;
    }

    OSType pix_fmt = CVPixelBufferGetPixelFormatType(image_buffer);

    CVReturn lock_result =
            CVPixelBufferLockBaseAddress(image_buffer, kCVPixelBufferLock_ReadOnly);
    if (lock_result != kCVReturnSuccess) {
        rust_log(ERROR, "CVPixelBufferLockBaseAddress failed: %d", lock_result);
        return;
    }

    size_t width = CVPixelBufferGetWidth(image_buffer);
    size_t height = CVPixelBufferGetHeight(image_buffer);
    size_t y_plane_stride = CVPixelBufferGetBytesPerRowOfPlane(image_buffer, 0);
    uint8_t *y_plane_buffer_address =
            CVPixelBufferGetBaseAddressOfPlane(image_buffer, 0);
    size_t uv_plane_stride = CVPixelBufferGetBytesPerRowOfPlane(image_buffer, 1);
    uint8_t *uv_plane_buffer_address =
            CVPixelBufferGetBaseAddressOfPlane(image_buffer, 1);

    _callback(_tx,
              width,
              height,
              pix_fmt == kCVPixelFormatType_420YpCbCr8BiPlanarFullRange,
              y_plane_buffer_address,
              y_plane_stride,
              uv_plane_buffer_address,
              uv_plane_stride,
              timing_info.decodeTimeStamp.value,
              timing_info.decodeTimeStamp.timescale,
              timing_info.presentationTimeStamp.value,
              timing_info.presentationTimeStamp.timescale);

    CVPixelBufferUnlockBaseAddress(image_buffer, kCVPixelBufferLock_ReadOnly);
}

- (void)dealloc {
    [_capture_session stopRunning];
    [_capture_session release];
    [_capture_video_data_output release];
    [_capture_screen_input release];
    [super dealloc];
}
@end
