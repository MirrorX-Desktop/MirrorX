#include "../include/duplicator.h"

@implementation Duplicator {
  AVCaptureSession* _capture_session;
  AVCaptureScreenInput* _capture_screen_input;
  AVCaptureVideoDataOutput* _capture_video_data_output;
  int _display_index;
  int _fps;
  capture_callback _callback;
  const void* _tx;
}

- (id)init:(int)display_index
         fps:(int)fps
          tx:(const void*)tx
    callback:(capture_callback)callback {
  self = [super init];
  if (self) {
    _display_index = display_index;
    _fps = fps;
    _tx = tx;
    _callback = callback;
  }

  AVCaptureSession* capture_session = [[AVCaptureSession alloc] init];
  [capture_session beginConfiguration];
  [capture_session setSessionPreset:AVCaptureSessionPresetHigh];

  AVCaptureScreenInput* capture_screen_input =
      [[AVCaptureScreenInput alloc] initWithDisplayID:(_display_index)];
  capture_screen_input.capturesCursor = YES;
  capture_screen_input.capturesMouseClicks = YES;
  capture_screen_input.minFrameDuration = CMTimeMake(1, _fps);
  //  capture_screen_input.scaleFactor = 0.25;
  if ([capture_session canAddInput:capture_screen_input]) {
    [capture_session addInput:capture_screen_input];
  } else {
    return nil;
  }

  AVCaptureVideoDataOutput* capture_video_data_output =
      [[AVCaptureVideoDataOutput alloc] init];

  dispatch_queue_t delegate_queue = dispatch_queue_create("duplicator", NULL);
  [capture_video_data_output setSampleBufferDelegate:self queue:delegate_queue];
  dispatch_release(delegate_queue);

  capture_video_data_output.alwaysDiscardsLateVideoFrames = YES;
  capture_video_data_output.videoSettings = @{
    (NSString*)kCVPixelBufferPixelFormatTypeKey :
        @(kCVPixelFormatType_420YpCbCr8BiPlanarFullRange),
    (NSString*)kCVPixelBufferWidthKey : @(1920),
    (NSString*)kCVPixelBufferHeightKey : @(1080)
  };
  if ([capture_session canAddOutput:(capture_video_data_output)]) {
    [capture_session addOutput:(capture_video_data_output)];
  } else {
    return nil;
  }

  [capture_session commitConfiguration];

  _capture_session = capture_session;
  _capture_screen_input = capture_screen_input;
  _capture_video_data_output = capture_video_data_output;

  return self;
}

- (void)startCapture {
  dispatch_async(dispatch_get_global_queue(DISPATCH_QUEUE_PRIORITY_HIGH, 0), ^{
    [_capture_session startRunning];
  });
}

- (void)stopCapture {
  [_capture_session stopRunning];
}

- (void)captureOutput:(AVCaptureOutput*)captureOutput
    didOutputSampleBuffer:(CMSampleBufferRef)videoFrame
           fromConnection:(AVCaptureConnection*)connection {
  if (!CMSampleBufferIsValid(videoFrame)) {
    return;
  }

  CVImageBufferRef image_buffer = CMSampleBufferGetImageBuffer(videoFrame);
  if (image_buffer == nil) {
    return;
  }

  CVReturn lock_result =
      CVPixelBufferLockBaseAddress(image_buffer, kCVPixelBufferLock_ReadOnly);
  if (lock_result != kCVReturnSuccess) {
    ffi_log(FFI_LOG_ERROR, "CVPixelBufferLockBaseAddress failed: %d",
            lock_result);
    return;
  }

  size_t width = CVPixelBufferGetWidth(image_buffer);
  size_t height = CVPixelBufferGetHeight(image_buffer);
  size_t y_line_size = CVPixelBufferGetBytesPerRowOfPlane(image_buffer, 0);
  uint8_t* y_buffer = CVPixelBufferGetBaseAddressOfPlane(image_buffer, 0);
  size_t uv_line_size = CVPixelBufferGetBytesPerRowOfPlane(image_buffer, 1);
  uint8_t* uv_buffer = CVPixelBufferGetBaseAddressOfPlane(image_buffer, 1);

  _callback(_tx, width, height, y_line_size, y_buffer, uv_line_size, uv_buffer);
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

const DuplicationContext* create_duplication_context(
    int display_index,
    void* tx,
    capture_callback callback) {
  Duplicator* duplicator = [[Duplicator alloc] init:display_index
                                                fps:60
                                                 tx:tx
                                           callback:callback];

  DuplicationContext* context =
      (DuplicationContext*)malloc(sizeof(DuplicationContext));

  context->duplicator = duplicator;

  return context;
}

void release_duplication_context(DuplicationContext* context) {
  [context->duplicator stopCapture];
  [context->duplicator release];
  free(context);
}

void start_capture(DuplicationContext* context) {
  if (context) {
    [context->duplicator startCapture];
  }
}

void stop_capture(DuplicationContext* context) {
  if (context) {
    [context->duplicator stopCapture];
  }
}