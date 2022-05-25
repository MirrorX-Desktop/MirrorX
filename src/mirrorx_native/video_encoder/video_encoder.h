#ifndef VIDEO_ENCODER_H
#define VIDEO_ENCODER_H

#include <stdbool.h>
#include <stdio.h>
#include "../rust_log/rust_log.h"

#ifdef __cplusplus
extern "C" {
#endif

#include <libavcodec/avcodec.h>
#include <libavutil/imgutils.h>
#include <libavutil/opt.h>
#include <libavutil/pixfmt.h>
#include <libavutil/rational.h>

typedef struct VideoEncoder {
  const AVCodec* codec;
  AVCodecContext* codec_ctx;
  AVFrame* frame;
  AVPacket* packet;
} VideoEncoder;

VideoEncoder* video_encoder_create(const char* encoder_name,
                                   int screen_width,
                                   int screen_height,
                                   int fps);

bool video_encoder_set_opt(VideoEncoder* encoder,
                           const char* opt_name,
                           const char* opt_value);

bool video_encoder_open(VideoEncoder* encoder);

// bool video_encoder_encode(VideoEncoder* video_encoder,
//                           void* tx,
//                           uint16_t width,
//                           uint16_t height,
//                           bool is_full_color_range,
//                           uint8_t* y_plane_buffer_address,
//                           uint32_t y_plane_stride,
//                           uint8_t* uv_plane_buffer_address,
//                           uint32_t uv_plane_stride,
//                           int64_t dts,
//                           int32_t dts_scale,
//                           int64_t pts,
//                           int32_t pts_scale);

void video_encoder_destroy(VideoEncoder* video_encoder);

#ifdef __cplusplus
};
#endif

#endif  // VIDEO_ENCODER_H