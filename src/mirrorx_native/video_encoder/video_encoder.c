#include "video_encoder.h"

VideoEncoder* video_encoder_create(const char* encoder_name,
                                   int screen_width,
                                   int screen_height,
                                   int fps,
                                   EncodeCallback callback) {
  int ret;
  AVCodecContext* codec_ctx = NULL;

  av_log_set_level(AV_LOG_TRACE);
  av_log_set_flags(AV_LOG_SKIP_REPEATED);
  av_log_set_callback(ffmpeg_log_callback);

  rust_log(TRACE, "video_encoder: new video encoder, name: %s", encoder_name);

  const AVCodec* codec = avcodec_find_encoder_by_name(encoder_name);
  if (NULL == codec) {
    rust_log(ERROR,
             "video_encoder: can't find an encoder named: %s",
             encoder_name);
    goto CLEAN;
  }

  codec_ctx = avcodec_alloc_context3(codec);
  if (NULL == codec_ctx) {
    rust_log(ERROR, "video_encoder: alloc codec context failed");
    goto CLEAN;
  }

  codec_ctx->width = screen_width;
  codec_ctx->height = screen_height;
  codec_ctx->time_base = (AVRational){1, fps * 100};
  codec_ctx->framerate = (AVRational){fps, 1};
  codec_ctx->gop_size = fps * 3;
  codec_ctx->has_b_frames = 0;
  codec_ctx->max_b_frames = 0;
  codec_ctx->qmax = 28;
  codec_ctx->qmin = 18;
  // codec_ctx->rc_max_rate = fps * screen_width * screen_height * 2 * 1000;
  // codec_ctx->rc_min_rate = screen_width * screen_height * 2 * 1000;
  // codec_ctx->rc_buffer_size = fps * screen_width * screen_height * 2 * 1024;
  codec_ctx->thread_count = 0;
  codec_ctx->pix_fmt = AV_PIX_FMT_NV12;
  codec_ctx->flags |= AV_CODEC_FLAG2_LOCAL_HEADER;
  // codec_ctx->flags |= AV_CODEC_FLAG_PASS2;

  if (strcmp(encoder_name, "libx264") == 0) {
    ret = av_opt_set(codec_ctx->priv_data, "preset", "ultrafast", 0);
    if (ret < 0) {
      rust_log(ERROR,
               "video_encoder: set codec_ctx opt <preset,fast> failed "
               "with code: %d",
               ret);
      goto CLEAN;
    }

    ret = av_opt_set(codec_ctx->priv_data, "profile", "baseline", 0);
    if (ret < 0) {
      rust_log(ERROR,
               "video_encoder: set codec_ctx opt <profile,high> failed "
               "with code: %d",
               ret);
      goto CLEAN;
    }

    ret = av_opt_set(codec_ctx->priv_data, "level", "5.2", 0);
    if (ret < 0) {
      rust_log(ERROR,
               "video_encoder: set codec_ctx opt <profile,high> failed "
               "with code: %d",
               ret);
      goto CLEAN;
    }

    ret = av_opt_set(codec_ctx->priv_data, "tune", "zerolatency", 0);
    if (ret < 0) {
      rust_log(ERROR,
               "video_encoder: set codec_ctx opt <tune,zerolatency> failed "
               "with code: %d",
               ret);
      goto CLEAN;
    }

    ret = av_opt_set(codec_ctx->priv_data, "sc_threshold", "499", 0);
    if (ret < 0) {
      rust_log(ERROR,
               "video_encoder: set codec_ctx opt <tune,zerolatency> failed "
               "with code: %d",
               ret);
      goto CLEAN;
    }
  }

  if (strcmp(encoder_name, "h264_videotoolbox") == 0) {
    ret = av_opt_set(codec_ctx->priv_data, "profile", "high", 0);
    if (ret < 0) {
      rust_log(ERROR,
               "video_encoder: set codec_ctx opt profile=high failed "
               "with code: %d",
               ret);
      goto CLEAN;
    }

    ret = av_opt_set(codec_ctx->priv_data, "level", "5.2", 0);
    if (ret < 0) {
      rust_log(ERROR,
               "video_encoder: set codec_ctx opt level=5.2 failed "
               "with code: %d",
               ret);
      goto CLEAN;
    }

    // ret = av_opt_set(codec_ctx->priv_data, "allow_sw", "0", 0);
    // if (ret < 0) {
    //   rust_log(ERROR,
    //            "video_encoder: set codec_ctx opt level=5.2 failed "
    //            "with code: %d",
    //            ret);
    //   goto CLEAN;
    // }
  }

  ret = avcodec_open2(codec_ctx, codec, NULL);
  if (ret != 0) {
    rust_log(ERROR, "video_encoder: open codec failed with code: %d", ret);
    goto CLEAN;
  }

  VideoEncoder* video_encoder = (VideoEncoder*)malloc(sizeof(VideoEncoder));
  if (NULL == video_encoder) {
    rust_log(ERROR, "video_encoder: malloc video encoder failed");
    goto CLEAN;
  }
  memset(video_encoder, 0, sizeof(VideoEncoder));
  video_encoder->codec_ctx = codec_ctx;
  video_encoder->callback = callback;

  return video_encoder;

CLEAN:
  if (NULL != codec_ctx) {
    avcodec_free_context(&codec_ctx);
  }

  return NULL;
}

bool video_encoder_encode(VideoEncoder* video_encoder,
                          void* tx,
                          uint16_t width,
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
  int ret;
  bool update_parameters = false;

  if (!video_encoder) {
    rust_log(ERROR, "video_encoder: video_encoder pointer is null");
    return false;
  }

  if (!tx) {
    rust_log(ERROR, "video_encoder: tx pointer is null");
    return false;
  }

  if (!y_plane_buffer_address) {
    rust_log(ERROR, "video_encoder: y_plane_buffer_address pointer is null");
    return false;
  }

  if (!uv_plane_buffer_address) {
    rust_log(ERROR, "video_encoder: uv_plane_buffer_address pointer is null");
    return false;
  }

  if ((NULL == video_encoder->frame) ||
      (video_encoder->frame->width != width) ||
      (video_encoder->frame->height != height)) {
    // release old frame
    if (NULL != video_encoder->frame) {
      av_frame_free(&video_encoder->frame);
      video_encoder->frame = NULL;
    }

    AVFrame* frame = av_frame_alloc();
    if (NULL == frame) {
      rust_log(ERROR, "video_encoder: alloc frame failed");
      return false;
    }

    frame->color_range =
        is_full_color_range ? AVCOL_RANGE_JPEG : AVCOL_RANGE_MPEG;
    frame->format = AV_PIX_FMT_NV12;
    frame->width = width;
    frame->height = height;

    ret = av_frame_get_buffer(frame, 32);
    if (ret < 0) {
      rust_log(ERROR,
               "video_encoder: alloc frame buffer failed with code: %d",
               ret);
      return false;
    }

    video_encoder->frame = frame;
    update_parameters = true;
  }

  ret = av_frame_make_writable(video_encoder->frame);
  if (ret != 0) {
    rust_log(ERROR,
             "video_encoder: make frame writable failed with code: %d",
             ret);
    return false;
  }

  video_encoder->frame->width = width;
  video_encoder->frame->height = height;
  video_encoder->frame->data[0] = y_plane_buffer_address;
  video_encoder->frame->linesize[0] = (int)y_plane_stride;
  video_encoder->frame->data[1] = uv_plane_buffer_address;
  video_encoder->frame->linesize[1] = (int)uv_plane_stride;
  video_encoder->frame->time_base.num = 1;
  video_encoder->frame->time_base.den = pts_scale;
  video_encoder->frame->pts = pts;

  ret = avcodec_send_frame(video_encoder->codec_ctx, video_encoder->frame);
  if (ret != 0) {
    if (ret == AVERROR(EAGAIN)) {
      rust_log(
          ERROR,
          "video_encoder: can not send more frame, should receive more packet");
    } else if (ret == AVERROR_EOF) {
      rust_log(ERROR,
               "video_encoder: encoder closed, shouldn't send new frame");
    } else {
      rust_log(ERROR, "video_encoder: send frame failed with code: %d", ret);
    }

    return false;
  }

  if (NULL == video_encoder->packet || update_parameters) {
    // release old packet
    if (NULL != video_encoder->packet) {
      av_packet_free(&video_encoder->packet);
      video_encoder->packet = NULL;
    }

    AVPacket* packet = av_packet_alloc();
    if (NULL == packet) {
      rust_log(ERROR, "video_encoder: alloc packet failed");
      return false;
    }

    int packet_size = av_image_get_buffer_size(
        (enum AVPixelFormat)video_encoder->frame->format,
        video_encoder->frame->width,
        video_encoder->frame->height,
        1);

    ret = av_new_packet(packet, packet_size);
    if (ret < 0) {
      rust_log(ERROR,
               "video_encoder: alloc packet buffer failed with code: %d",
               ret);
      return false;
    }

    video_encoder->packet = packet;
  }

  while (true) {
    ret =
        avcodec_receive_packet(video_encoder->codec_ctx, video_encoder->packet);
    if (ret == AVERROR(EAGAIN) || ret == AVERROR_EOF) {
      rust_log(TRACE, "video_encoder: no more packet can receive");
      return true;
    } else if (ret < 0) {
      rust_log(ERROR,
               "video_encoder: receive packet failed with code: %d",
               ret);
      return false;
    }

    if (video_encoder->callback) {
      video_encoder->callback(tx,
                              video_encoder->packet->data,
                              video_encoder->packet->size,
                              video_encoder->packet->dts,
                              video_encoder->packet->pts);
    }

    av_packet_unref(video_encoder->packet);
  }

  return true;
}

void video_encoder_destroy(VideoEncoder* video_encoder) {
  if (NULL == video_encoder) {
    return;
  }

  avcodec_send_frame(video_encoder->codec_ctx, NULL);

  if (NULL != video_encoder->frame) {
    av_frame_free(&video_encoder->frame);
  }

  if (NULL != video_encoder->packet) {
    av_packet_free(&video_encoder->packet);
  }

  if (NULL != video_encoder->codec_ctx) {
    // avcodec_close(video_encoder->codec_ctx);
    avcodec_free_context(&video_encoder->codec_ctx);
  }

  free(video_encoder);
}