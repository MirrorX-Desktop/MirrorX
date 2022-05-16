#include "../capture_session.h"
#include "capture_session_context.h"

CaptureSession* capture_session_create(int display_index,
                                       int fps,
                                       void* tx,
                                       capture_session_callback callback) {
  CaptureSessionContext* ctx =
      [[CaptureSessionContext alloc] init:display_index
                                      fps:fps
                                 callback:^(uint16_t width,
                                            uint16_t height,
                                            bool is_full_color_range,
                                            uint8_t* y_plane_buffer_address,
                                            uint32_t y_plane_stride,
                                            uint8_t* uv_plane_buffer_address,
                                            uint32_t uv_plane_stride,
                                            int64_t dts,
                                            int32_t dts_scale,
                                            int64_t pts,
                                            int32_t pts_scale) {
                                   callback(tx,
                                            width,
                                            height,
                                            is_full_color_range,
                                            y_plane_buffer_address,
                                            y_plane_stride,
                                            uv_plane_buffer_address,
                                            uv_plane_stride,
                                            dts,
                                            pts);
                                 }];

  if (NULL == ctx) {
    return NULL;
  }

  CaptureSession* session = (CaptureSession*)malloc(sizeof(CaptureSession));
  memset(session, 0, sizeof(CaptureSession));
  session->ctx = ctx;
  return session;
}

void capture_session_destroy(CaptureSession* capture_session) {
  if (NULL == capture_session) {
    return;
  }

  if (NULL != capture_session->ctx) {
    [capture_session->ctx release];
    capture_session->ctx = NULL;
  }

  free(capture_session);
}

void capture_session_start(CaptureSession* capture_session) {
  if (NULL == capture_session || NULL == capture_session->ctx) {
    return;
  }

  [capture_session->ctx startCapture];
}

void capture_session_stop(CaptureSession* capture_session) {
  if (NULL == capture_session || NULL == capture_session->ctx) {
    return;
  }

  [capture_session->ctx stopCapture];
}