#include "video_encoder.h"

VideoEncoder* video_encoder_create(const char* encoder_name,
                                   int screen_width,
                                   int screen_height,
                                   int fps) {
  av_log_set_level(AV_LOG_TRACE);
  av_log_set_flags(AV_LOG_SKIP_REPEATED);
  av_log_set_callback(ffmpeg_log_callback);

  AVCodecContext* codec_ctx = NULL;

  rust_log(LEVEL_TRACE,
           "video_encoder: create video encoder, name: %s",
           encoder_name);

  const AVCodec* codec = avcodec_find_encoder_by_name(encoder_name);
  if (NULL == codec) {
    rust_log(LEVEL_ERROR,
             "video_encoder: can't find an encoder named: %s",
             encoder_name);
    return NULL;
  }

  codec_ctx = avcodec_alloc_context3(codec);
  if (NULL == codec_ctx) {
    rust_log(LEVEL_ERROR, "video_encoder: alloc codec context failed");
    return NULL;
  }

  codec_ctx->width = screen_width;
  codec_ctx->height = screen_height;
  codec_ctx->time_base.num = 1;
  codec_ctx->time_base.den = fps * 100;
  codec_ctx->framerate.num = fps;
  codec_ctx->framerate.den = 1;
  codec_ctx->gop_size = fps * 3;
  codec_ctx->has_b_frames = 0;
  codec_ctx->max_b_frames = 0;
  codec_ctx->qmax = 28;
  codec_ctx->qmin = 18;
  codec_ctx->thread_count = 2;
  codec_ctx->pix_fmt = AV_PIX_FMT_NV12;
  codec_ctx->flags |= AV_CODEC_FLAG2_LOCAL_HEADER;

  codec_ctx->color_range = AVCOL_RANGE_JPEG;
  codec_ctx->color_primaries = AVCOL_PRI_BT709;
  codec_ctx->color_trc = AVCOL_TRC_BT709;
  codec_ctx->colorspace = AVCOL_SPC_BT709;

  VideoEncoder* video_encoder = (VideoEncoder*)malloc(sizeof(VideoEncoder));
  if (NULL == video_encoder) {
    avcodec_free_context(&codec_ctx);

    rust_log(LEVEL_ERROR, "video_encoder: malloc video encoder failed");
    return NULL;
  }

  memset(video_encoder, 0, sizeof(VideoEncoder));

  video_encoder->codec = codec;
  video_encoder->codec_ctx = codec_ctx;

  return video_encoder;
}

bool video_encoder_set_opt(VideoEncoder* encoder,
                           const char* opt_name,
                           const char* opt_value) {
  if (NULL == encoder || NULL == opt_name || NULL == opt_value) {
    rust_log(LEVEL_ERROR,
             "video_encoder: set option failed, invalid parameter"
             "encoder: %p, "
             "opt_name: %s, "
             "opt_value: %s",
             encoder,
             opt_name,
             opt_value);
    return false;
  }

  int ret = av_opt_set(encoder->codec_ctx->priv_data,
                       opt_name,
                       opt_value,
                       AV_OPT_SEARCH_CHILDREN);

  if (ret == 0) {
    return true;
  }

  if (ret == AVERROR_OPTION_NOT_FOUND) {
    rust_log(LEVEL_ERROR,
             "video_encoder: set option failed, option not found"
             "encoder: %p, "
             "opt_name: %s, "
             "opt_value: %s",
             encoder,
             opt_name,
             opt_value);

  } else if (ret == AVERROR(ERANGE)) {
    rust_log(LEVEL_ERROR,
             "video_encoder: set option failed, value is out of range"
             "encoder: %p, "
             "opt_name: %s, "
             "opt_value: %s",
             encoder,
             opt_name,
             opt_value);

  } else if (ret == AVERROR(EINVAL)) {
    rust_log(LEVEL_ERROR,
             "video_encoder: set option failed, invalid value"
             "encoder: %p, "
             "opt_name: %s, "
             "opt_value: %s",
             encoder,
             opt_name,
             opt_value);
  } else {
    rust_log(LEVEL_ERROR,
             "video_encoder: set option failed, unknown error"
             "encoder: %p, "
             "opt_name: %s, "
             "opt_value: %s, "
             "ret: %d",
             encoder,
             opt_name,
             opt_value,
             ret);
  }

  return false;
}

bool video_encoder_open(VideoEncoder* encoder) {
  if (NULL == encoder) {
    rust_log(LEVEL_ERROR, "video_encoder: open failed");
    return false;
  }

  int ret = avcodec_open2(encoder->codec_ctx, encoder->codec, NULL);
  if (ret != 0) {
    rust_log(LEVEL_ERROR,
             "video_encoder: open codec failed with code: ret: %d",
             ret);
    return false;
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