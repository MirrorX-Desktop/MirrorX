#ifndef VIDEO_DECODER_H
#define VIDEO_DECODER_H

#include <stdbool.h>
#include "../rust_log/rust_log.h"

#ifdef __cplusplus
extern "C" {
#endif

#include <libavcodec/avcodec.h>
#include <libavutil/opt.h>
#include <libavutil/pixdesc.h>

typedef void (*DecodeCallback)(void* tx,
                               uint16_t width,
                               uint16_t height,
                               bool is_full_color_range,
                               const uint8_t* y_plane_buffer_address,
                               uint32_t y_plane_stride,
                               const uint8_t* uv_plane_buffer_address,
                               uint32_t uv_plane_stride,
                               int64_t dts,
                               int64_t pts);

typedef struct VideoDecoder {
  AVCodecContext* codec_ctx;
  AVFrame* frame;
  AVPacket* packet;
  AVCodecParserContext* parser_ctx;
  AVFrame* hw_transfer_frame;
  DecodeCallback callback;
} VideoDecoder;

VideoDecoder* video_decoder_create(const char* decoder_name,
                                   DecodeCallback callback);

void video_decoder_destroy(VideoDecoder* video_decoder);

void video_decoder_decode(VideoDecoder* video_decoder,
                          void* tx,
                          uint8_t* packet_data,
                          int packet_size,
                          int64_t dts,
                          int64_t pts);

#ifdef __cplusplus
};
#endif

#endif  // VIDEO_DECODER_H