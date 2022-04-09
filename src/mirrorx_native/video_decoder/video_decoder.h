#ifndef VIDEO_DECODER_H
#define VIDEO_DECODER_H

#ifdef __cplusplus
extern "C" {
#endif

#include "../ffi_log/ffi_log.h"
#include <libavcodec/avcodec.h>
#include <libavutil/pixdesc.h>
#include <stdbool.h>

typedef void (*VideoDecoderCallback)(const void *tx, int width, int height,
                                     enum AVPixelFormat pix_fmt,
                                     uint8_t *plane_buffer_address[8],
                                     int plane_linesize[8]);

typedef struct VideoDecoder {
  AVCodecContext *codec_ctx;
  AVFrame *frame;
  AVPacket *packet;
  AVCodecParserContext *parser_ctx;
  AVBufferRef *hw_device_ctx;
  AVFrame *hw_copied_frame;
  VideoDecoderCallback decode_callback;
} VideoDecoder;

VideoDecoder *new_video_decoder(const char *decoder_name,
                                enum AVHWDeviceType device_type,
                                VideoDecoderCallback decode_callback);

int video_decode(const VideoDecoder *video_decoder, const void *tx,
                 const uint8_t *packet_data, int packet_size);

void free_video_decoder(VideoDecoder *video_decoder);

#ifdef __cplusplus
};
#endif

#endif // VIDEO_DECODER_H