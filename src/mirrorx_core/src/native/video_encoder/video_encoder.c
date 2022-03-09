#include "video_encoder.h"

VideoEncoder *new_video_encoder(const char *encoder_name, int fps,
                                int src_width, int src_height, int dst_width,
                                int dst_height,
                                VideoEncoderCallback encode_callback) {
  int ret;
  AVCodecContext *codec_ctx = NULL;
  AVFrame *frame = NULL;
  AVPacket *packet = NULL;
  VideoEncoder *video_encoder = NULL;

  av_log_set_level(AV_LOG_TRACE);
  av_log_set_flags(AV_LOG_SKIP_REPEATED);
  av_log_set_callback(ffmpeg_log_callback);

  ffi_log(FFI_LOG_TRACE, "video_encoder: new video encoder, name: %s",
          encoder_name);

  const AVCodec *codec = avcodec_find_encoder_by_name(encoder_name);
  if (NULL == codec) {
    ffi_log(FFI_LOG_ERROR, "video_encoder: can't find an encoder named '%s'",
            encoder_name);
    goto CLEAN;
  }

  codec_ctx = avcodec_alloc_context3(codec);
  if (NULL == codec_ctx) {
    ffi_log(FFI_LOG_ERROR, "video_encoder: alloc codec context failed");
    goto CLEAN;
  }

  codec_ctx->width = dst_width;
  codec_ctx->height = dst_height;
  codec_ctx->time_base = (AVRational){1, fps};
  codec_ctx->framerate = (AVRational){fps, 1};
  codec_ctx->gop_size = fps * 2;
  codec_ctx->has_b_frames = 0;
  codec_ctx->max_b_frames = 0;
  codec_ctx->pix_fmt = AV_PIX_FMT_NV12;
  codec_ctx->flags |= AV_CODEC_FLAG2_LOCAL_HEADER;

  if (strcmp(encoder_name, "libx264") == 0) {
    ret = av_opt_set(codec_ctx->priv_data, "preset", "fast", 0);
    if (ret < 0) {
      ffi_log(FFI_LOG_ERROR,
              "video_encoder: set codec_ctx opt <preset,fast> failed "
              "with code: %d",
              ret);
      goto CLEAN;
    }

    ret = av_opt_set(codec_ctx->priv_data, "profile", "high", 0);
    if (ret < 0) {
      ffi_log(FFI_LOG_ERROR,
              "video_encoder: set codec_ctx opt <profile,high> failed "
              "with code: %d",
              ret);
      goto CLEAN;
    }

    ret = av_opt_set(codec_ctx->priv_data, "tune", "zerolatency", 0);
    if (ret < 0) {
      ffi_log(FFI_LOG_ERROR,
              "video_encoder: set codec_ctx opt <tune,zerolatency> failed "
              "with code: %d",
              ret);
      goto CLEAN;
    }
  }

  frame = av_frame_alloc();
  if (NULL == frame) {
    ffi_log(FFI_LOG_ERROR, "video_encoder: alloc frame failed");
    goto CLEAN;
  }

  frame->format = AV_PIX_FMT_NV12;
  frame->width = src_width;
  frame->height = src_height;

  ret = av_frame_get_buffer(frame, 32);
  if (ret < 0) {
    ffi_log(FFI_LOG_ERROR,
            "video_encoder: alloc frame buffer failed with code: %d", ret);
    goto CLEAN;
  }

  packet = av_packet_alloc();
  if (NULL == packet) {
    ffi_log(FFI_LOG_ERROR, "video_encoder: alloc packet failed");
    goto CLEAN;
  }

  ret = av_new_packet(
      packet, av_image_get_buffer_size((enum AVPixelFormat)frame->format,
                                       frame->width, frame->height, 1));
  if (ret < 0) {
    ffi_log(FFI_LOG_ERROR,
            "video_encoder: alloc packet buffer failed with code: %d", ret);
    goto CLEAN;
  }

  ret = avcodec_open2(codec_ctx, codec, NULL);
  if (ret != 0) {
    ffi_log(FFI_LOG_ERROR, "video_encoder: open codec failed with code: %d",
            ret);
    goto CLEAN;
  }

  video_encoder = (VideoEncoder *)malloc(sizeof(VideoEncoder));
  video_encoder->codec_ctx = codec_ctx;
  video_encoder->frame = frame;
  video_encoder->packet = packet;
  video_encoder->encode_callback = encode_callback;

  return video_encoder;

CLEAN:
  if (NULL != packet) {
    av_packet_free(&packet);
  }

  if (NULL != frame) {
    av_frame_free(&frame);
  }

  if (NULL != codec_ctx) {
    avcodec_free_context(&codec_ctx);
  }

  return NULL;
}

int video_encode(const VideoEncoder *video_encoder, const void *tx, int width,
                 int height, int y_line_size, uint8_t *y_buffer,
                 int uv_line_size, uint8_t *uv_buffer) {
  int ret;

  ret = av_frame_make_writable(video_encoder->frame);
  if (ret != 0) {
    ffi_log(FFI_LOG_ERROR,
            "video_encoder: make frame writable failed with code = %d", ret);
    return ret;
  }

  video_encoder->frame->width = width;
  video_encoder->frame->height = height;
  video_encoder->frame->linesize[0] = y_line_size;
  video_encoder->frame->data[0] = y_buffer;
  video_encoder->frame->linesize[1] = uv_line_size;
  video_encoder->frame->data[1] = uv_buffer;

  ret = avcodec_send_frame(video_encoder->codec_ctx, video_encoder->frame);
  if (ret < 0) {
    if (ret == AVERROR(EAGAIN)) {
      ffi_log(
          FFI_LOG_ERROR,
          "video_encoder: can not send more frame, should receive more packet");
    } else if (ret == AVERROR_EOF) {
      ffi_log(FFI_LOG_ERROR,
              "video_encoder: encoder closed, shouldn't send new frame");
    } else {
      ffi_log(FFI_LOG_ERROR, "video_encoder: send frame failed with code: %d",
              ret);
    }

    return ret;
  }

  while (true) {
    ret =
        avcodec_receive_packet(video_encoder->codec_ctx, video_encoder->packet);
    if (ret == AVERROR(EAGAIN) || ret == AVERROR_EOF) {
      ffi_log(FFI_LOG_TRACE, "video_encoder: no more packet can receive");
      return 0;
    } else if (ret < 0) {
      ffi_log(FFI_LOG_ERROR,
              "video_encoder: receive packet failed with code: %d", ret);
      return ret;
    }

    video_encoder->encode_callback(tx, video_encoder->packet->data,
                                   video_encoder->packet->size);

    av_packet_unref(video_encoder->packet);
  }
}

void free_video_encoder(VideoEncoder *video_encoder) {
  if (NULL == video_encoder) {
    return;
  }

  avcodec_send_frame(video_encoder->codec_ctx, NULL);

  if (NULL != video_encoder->codec_ctx) {
    av_packet_free(&video_encoder->packet);
  }

  if (NULL != video_encoder->frame) {
    av_frame_free(&video_encoder->frame);
  }

  if (NULL != video_encoder->codec_ctx) {
    avcodec_free_context(&video_encoder->codec_ctx);
  }

  free(video_encoder);
}