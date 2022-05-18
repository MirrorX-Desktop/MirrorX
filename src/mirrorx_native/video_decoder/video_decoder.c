#include "video_decoder.h"

VideoDecoder* video_decoder_create(const char* decoder_name,
                                   DecodeCallback callback) {
  int ret;
  AVCodecContext* codec_ctx = NULL;
  AVPacket* packet = NULL;
  AVFrame* frame = NULL;
  AVCodecParserContext* parse_ctx = NULL;  // for software decode
  AVBufferRef* hw_device_ctx = NULL;       // for hardware decode
  AVFrame* hw_transfer_frame = NULL;

  av_log_set_level(AV_LOG_TRACE);
  av_log_set_flags(AV_LOG_SKIP_REPEATED);
  av_log_set_callback(ffmpeg_log_callback);

  rust_log(LEVEL_TRACE,
           "video_decoder: create video decoder, decoder_name: %s",
           decoder_name);

  enum AVHWDeviceType print_type = AV_HWDEVICE_TYPE_NONE;

  while ((print_type = av_hwdevice_iterate_types(print_type)) !=
         AV_HWDEVICE_TYPE_NONE) {
    rust_log(LEVEL_INFO, "suport devices %s", av_hwdevice_get_type_name(print_type));
  }

  const AVCodec* codec = avcodec_find_decoder_by_name(decoder_name);
  if (NULL == codec) {
    rust_log(LEVEL_ERROR,
             "video_decoder: can't find an decoder named '%s'",
             decoder_name);
    goto CLEAN;
  }

  codec_ctx = avcodec_alloc_context3(codec);
  if (NULL == codec_ctx) {
    rust_log(LEVEL_ERROR, "video_decoder: alloc codec context failed");
    goto CLEAN;
  }

  codec_ctx->flags |= AV_CODEC_CAP_TRUNCATED;

  const AVCodecHWConfig* hw_config = avcodec_get_hw_config(codec, 0);
  if (NULL == hw_config) {
    // software decoder
    parse_ctx = av_parser_init(codec->id);
    if (NULL == parse_ctx) {
      rust_log(LEVEL_ERROR, "video_decoder: init codec parse context failed");
      goto CLEAN;
    }
  } else {
    // handware decoder
    ret = av_hwdevice_ctx_create(&hw_device_ctx,
                                 hw_config->device_type,
                                 NULL,
                                 NULL,
                                 0);
    if (ret < 0) {
      rust_log(LEVEL_ERROR, "video_decoder: create hw_device context failed");
      goto CLEAN;
    }

    codec_ctx->hw_device_ctx = hw_device_ctx;

    hw_transfer_frame = av_frame_alloc();
    if (NULL == hw_transfer_frame) {
      rust_log(LEVEL_ERROR, "video_decoder: alloc hw_copied_frame failed");
      goto CLEAN;
    }
  }

  packet = av_packet_alloc();
  if (NULL == packet) {
    rust_log(LEVEL_ERROR, "video_decoder: alloc packet failed");
    goto CLEAN;
  }

  frame = av_frame_alloc();
  if (NULL == frame) {
    rust_log(LEVEL_ERROR, "video_decoder: alloc frame failed");
    goto CLEAN;
  }

  ret = avcodec_open2(codec_ctx, codec, NULL);
  if (ret != 0) {
    rust_log(LEVEL_ERROR,
             "video_decoder: open codec context failed with code: %d",
             ret);
    goto CLEAN;
  }

  VideoDecoder* video_decoder = (VideoDecoder*)malloc(sizeof(VideoDecoder));
  if (NULL == video_decoder) {
    goto CLEAN;
  }
  memset(video_decoder, 0, sizeof(VideoDecoder));
  video_decoder->codec_ctx = codec_ctx;
  video_decoder->packet = packet;
  video_decoder->frame = frame;
  video_decoder->parser_ctx = parse_ctx;
  video_decoder->hw_transfer_frame = hw_transfer_frame;
  video_decoder->callback = callback;

  return video_decoder;

CLEAN:
  if (NULL != frame) {
    av_frame_free(&frame);
  }

  if (NULL != packet) {
    av_packet_free(&packet);
  }

  if (NULL != hw_transfer_frame) {
    av_frame_free(&hw_transfer_frame);
  }

  if (NULL != hw_device_ctx) {
    av_buffer_unref(&hw_device_ctx);
  }

  if (NULL != parse_ctx) {
    av_parser_close(parse_ctx);
  }

  if (NULL != codec_ctx) {
    avcodec_free_context(&codec_ctx);
  }

  return NULL;
}

void video_decoder_decode(VideoDecoder* video_decoder,
                          void* tx,
                          uint8_t* packet_data,
                          int packet_size,
                          int64_t dts,
                          int64_t pts) {
  int ret;

  if (!video_decoder) {
    rust_log(LEVEL_ERROR, "video_decoder: video_decoder pointer is null");
    return;
  }

  if (!tx) {
    rust_log(LEVEL_ERROR, "video_decoder: tx pointer is null");
    return;
  }

  if (!packet_data) {
    rust_log(LEVEL_ERROR, "video_decoder: packet_data pointer is null");
    return;
  }

  if (!packet_size) {
    rust_log(LEVEL_ERROR, "video_decoder: packet_size is zero");
    return;
  }

  // before software decode, we need to ensure the input packet data is enough
  // for AVPacket parsing.
  if (video_decoder->parser_ctx != NULL) {
    ret = av_parser_parse2(video_decoder->parser_ctx,
                           video_decoder->codec_ctx,
                           &video_decoder->packet->data,
                           &video_decoder->packet->size,
                           packet_data,
                           packet_size,
                           pts,
                           dts,
                           0);
    if (ret < 0) {
      rust_log(LEVEL_ERROR, "video_decoder: call av_parser_parse2 failed");
      return;
    }

    if (video_decoder->packet->size == 0) {
      rust_log(LEVEL_TRACE, "video_decoder: parsed packet size is 0, need more data");
      return;
    }
  } else {
    video_decoder->packet->data = packet_data;
    video_decoder->packet->size = packet_size;
    video_decoder->packet->dts = dts;
    video_decoder->packet->pts = pts;
  }

  ret = avcodec_send_packet(video_decoder->codec_ctx, video_decoder->packet);
  if (ret < 0) {
    if (ret == AVERROR(EAGAIN)) {
      rust_log(
          LEVEL_ERROR,
          "video_decoder: can not send more frame, should receive more packet");
    } else if (ret == AVERROR_EOF) {
      rust_log(LEVEL_ERROR,
               "video_decoder: encoder closed, shouldn't send new frame");
    } else {
      rust_log(LEVEL_ERROR, "video_decoder: send frame failed with code: %d", ret);
    }

    return;
  }

  // frame is just a pointer to video_encoder->frame or
  // video_encoder->hw_transfer_frame, no need to alloc and free.
  AVFrame* frame = NULL;

  while (true) {
    ret = avcodec_receive_frame(video_decoder->codec_ctx, video_decoder->frame);
    if (ret == AVERROR(EAGAIN) || ret == AVERROR_EOF) {
      break;
    } else if (ret < 0) {
      rust_log(LEVEL_ERROR, "video_decoder: receive frame failed with code: %d", ret);
      break;
    }

    if (video_decoder->codec_ctx->hw_device_ctx != NULL) {
      ret = av_hwframe_transfer_data(video_decoder->hw_transfer_frame,
                                     video_decoder->frame,
                                     0);
      if (ret < 0) {
        rust_log(
            LEVEL_ERROR,
            "video_decoder: call av_hwframe_transfer_data failed with code: %d",
            ret);
        break;
      }

      frame = video_decoder->hw_transfer_frame;
    } else {
      frame = video_decoder->frame;
    }

    if (video_decoder->callback) {
      video_decoder->callback(tx,
                              frame->width,
                              frame->height,
                              frame->color_range == AVCOL_RANGE_JPEG,
                              frame->data[0],
                              frame->linesize[0],
                              frame->data[1],
                              frame->linesize[1],
                              frame->pkt_dts,
                              frame->pts);
    } else {
      rust_log(LEVEL_ERROR, "video_decoder: callback is null");
    }
  }

  av_packet_unref(video_decoder->packet);
}

void video_decoder_destroy(VideoDecoder* video_decoder) {
  if (video_decoder == NULL) {
    return;
  }

  avcodec_send_packet(video_decoder->codec_ctx, NULL);

  if (NULL != video_decoder->hw_transfer_frame) {
    av_frame_free(&video_decoder->hw_transfer_frame);
  }

  if (NULL != video_decoder->parser_ctx) {
    av_parser_close(video_decoder->parser_ctx);
  }

  if (NULL != video_decoder->codec_ctx) {
    avcodec_free_context(&(video_decoder->codec_ctx));
  }

  if (NULL != video_decoder->frame) {
    av_frame_free(&video_decoder->frame);
  }

  if (NULL != video_decoder->packet) {
    av_packet_free(&video_decoder->packet);
  }

  free(video_decoder);
}