#ifndef VIDEO_ENCODER_H
#define VIDEO_ENCODER_H

#ifdef __cplusplus
extern "C" {
#endif

#include "../ffi_log/ffi_log.h"
#include <libavcodec/avcodec.h>
#include <libavutil/imgutils.h>
#include <libavutil/opt.h>
#include <libavutil/pixfmt.h>
#include <libavutil/rational.h>
#include <stdbool.h>
#include <stdio.h>

typedef void (*VideoEncoderCallback)(const void *tx, const uint8_t *packet_data,
                                     int packet_size);

typedef struct VideoEncoder {
  AVCodecContext *codec_ctx;
  AVFrame *frame;
  AVPacket *packet;
  VideoEncoderCallback encode_callback;
} VideoEncoder;

VideoEncoder *new_video_encoder(const char *encoder_name, int fps,
                                int src_width, int src_height, int dst_width,
                                int dst_height,
                                VideoEncoderCallback encode_callback);

int video_encode(const VideoEncoder *video_encoder, const void *tx, int width,
                 int height, int y_line_size, uint8_t *y_buffer,
                 int uv_line_size, uint8_t *uv_buffer);

void free_video_encoder(VideoEncoder *video_encoder);

#ifdef __cplusplus
};
#endif

#endif // VIDEO_ENCODER_H