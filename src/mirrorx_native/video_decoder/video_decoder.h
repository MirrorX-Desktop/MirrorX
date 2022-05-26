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

typedef struct VideoDecoder {
  AVCodecContext* codec_ctx;
  AVFrame* frame;
  AVPacket* packet;
  AVCodecParserContext* parser_ctx;
  AVFrame* hw_transfer_frame;
} VideoDecoder;

VideoDecoder* video_decoder_create(const char* decoder_name);

void video_decoder_destroy(VideoDecoder* video_decoder);

// void video_decoder_decode(VideoDecoder* video_decoder,
//                           void* tx,
//                           uint8_t* packet_data,
//                           int packet_size,
//                           int64_t dts,
//                           int64_t pts);

#ifdef __cplusplus
};
#endif

#endif  // VIDEO_DECODER_H