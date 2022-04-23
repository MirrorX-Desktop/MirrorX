#include "video_decoder.h"

VideoDecoder *new_video_decoder(const char *decoder_name,
                                enum AVHWDeviceType device_type,
                                VideoDecoderCallback decode_callback) {
  int ret;
  AVCodecContext *codec_ctx = NULL;
  AVPacket *packet = NULL;
  AVFrame *frame = NULL;
  // for software decode
  AVCodecParserContext *parse_ctx = NULL;
  // for hardware decode
  AVBufferRef *hw_device_ctx = NULL;
  AVFrame *hw_copied_frame = NULL;
  VideoDecoder *video_decoder = NULL;

  av_log_set_level(AV_LOG_TRACE);
  av_log_set_flags(AV_LOG_SKIP_REPEATED);
  av_log_set_callback(ffmpeg_log_callback);

  ffi_log(FFI_LOG_TRACE, "video_decoder: new video decoder, name: %s",
          decoder_name);

  const AVCodec *codec = avcodec_find_decoder_by_name(decoder_name);
  if (NULL == codec) {
    ffi_log(FFI_LOG_ERROR, "video_decoder: can't find an decoder named '%s'",
            decoder_name);
    goto CLEAN;
  }

  codec_ctx = avcodec_alloc_context3(codec);
  if (NULL == codec_ctx) {
    ffi_log(FFI_LOG_ERROR, "video_decoder: alloc codec context failed");
    goto CLEAN;
  }

  codec_ctx->flags |= AV_CODEC_CAP_TRUNCATED;

  if (device_type == AV_HWDEVICE_TYPE_NONE) {
    parse_ctx = av_parser_init(codec->id);
    if (NULL == parse_ctx) {
      ffi_log(FFI_LOG_ERROR, "video_decoder: init codec parse context failed");
      goto CLEAN;
    }
  } else {
    ret = av_hwdevice_ctx_create(&hw_device_ctx, device_type, NULL, NULL, 0);
    if (ret < 0) {
      ffi_log(FFI_LOG_ERROR,
              "video_decoder: create hw m_device context failed");
      goto CLEAN;
    }

    codec_ctx->hw_device_ctx = hw_device_ctx;

    hw_copied_frame = av_frame_alloc();
    if (NULL == hw_copied_frame) {
      ffi_log(FFI_LOG_ERROR, "video_decoder: alloc hw_copied_frame failed");
      goto CLEAN;
    }
  }

  packet = av_packet_alloc();
  if (NULL == packet) {
    ffi_log(FFI_LOG_ERROR, "video_decoder: alloc packet failed");
    goto CLEAN;
  }

  frame = av_frame_alloc();
  if (NULL == frame) {
    ffi_log(FFI_LOG_ERROR, "video_decoder: alloc frame failed");
    goto CLEAN;
  }

  ret = avcodec_open2(codec_ctx, codec, NULL);
  if (ret != 0) {
    ffi_log(FFI_LOG_ERROR,
            "video_decoder: open codec context failed with code: %d", ret);
    goto CLEAN;
  }

  video_decoder = (VideoDecoder *)malloc(sizeof(VideoDecoder));
  video_decoder->codec_ctx = codec_ctx;
  video_decoder->packet = packet;
  video_decoder->frame = frame;
  video_decoder->parser_ctx = parse_ctx;
  video_decoder->hw_device_ctx = hw_device_ctx;
  video_decoder->hw_copied_frame = hw_copied_frame;
  video_decoder->decode_callback = decode_callback;

  return video_decoder;

CLEAN:
  if (NULL != frame) {
    av_frame_free(&frame);
  }

  if (NULL != packet) {
    av_packet_free(&packet);
  }

  if (NULL != hw_copied_frame) {
    av_frame_free(&hw_copied_frame);
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

int video_decode(const VideoDecoder *video_decoder, const void *tx,
                 const uint8_t *packet_data, int packet_size) {
  int ret;

  if (!packet_data || !packet_size) {
    ffi_log(FFI_LOG_ERROR,
            "video_decoder: video decode packet is nil or size is 0");
    return 0;
  }

  // before software decode, we need to ensure the input packet data is enough
  // for AVPacket parsing.
  if (video_decoder->parser_ctx != NULL) {
    ret = av_parser_parse2(video_decoder->parser_ctx, video_decoder->codec_ctx,
                           &video_decoder->packet->data,
                           &video_decoder->packet->size, packet_data,
                           packet_size, AV_NOPTS_VALUE, AV_NOPTS_VALUE, 0);
    if (ret < 0) {
      ffi_log(FFI_LOG_ERROR, "video_decoder: call av_parser_parse2 failed");
      return ret;
    }

    if (video_decoder->packet->size == 0) {
      ffi_log(FFI_LOG_TRACE,
              "video_decoder: parsed packet size is 0, need more data");
      return 0;
    }
  } else {
    video_decoder->packet->data = (uint8_t *)packet_data;
    video_decoder->packet->size = packet_size;
  }

  ret = avcodec_send_packet(video_decoder->codec_ctx, video_decoder->packet);
  if (ret < 0) {
    if (ret == AVERROR(EAGAIN)) {
      ffi_log(
          FFI_LOG_ERROR,
          "video_decoder: can not send more frame, should receive more packet");
    } else if (ret == AVERROR_EOF) {
      ffi_log(FFI_LOG_ERROR,
              "video_decoder: encoder closed, shouldn't send new frame");
    } else {
      ffi_log(FFI_LOG_ERROR, "video_decoder: send frame failed with code: %d",
              ret);
    }

    return ret;
  }

  // tmp_frame just a pointer to video_encoder->frame or
  // video_encoder->hw_copied_frame, no need to alloc and free.
  AVFrame *tmp_frame = NULL;

  while (true) {
    ret = avcodec_receive_frame(video_decoder->codec_ctx, video_decoder->frame);
    if (ret == AVERROR(EAGAIN) || ret == AVERROR_EOF) {
      ret = 0;
      break;
    } else if (ret < 0) {
      break;
    }

    if (video_decoder->hw_device_ctx != NULL) {
      ret = av_hwframe_transfer_data(video_decoder->hw_copied_frame,
                                     video_decoder->frame, 0);
      if (ret < 0) {
        ffi_log(
            FFI_LOG_ERROR,
            "video_decoder: call av_hwframe_transfer_data failed with code: %d",
            ret);
        break;
      }

      tmp_frame = video_decoder->hw_copied_frame;
    } else {
      tmp_frame = video_decoder->frame;
    }

    video_decoder->decode_callback(tx, tmp_frame->width, tmp_frame->height,
                                   (enum AVPixelFormat)tmp_frame->format,
                                   tmp_frame->data, tmp_frame->linesize);
  }

  av_packet_unref(video_decoder->packet);
  return ret;
}

void free_video_decoder(VideoDecoder *video_decoder) {
  if (video_decoder == NULL) {
    return;
  }

  avcodec_send_packet(video_decoder->codec_ctx, NULL);

  if (NULL != video_decoder->frame) {
    av_frame_free(&video_decoder->frame);
  }

  if (NULL != video_decoder->packet) {
    av_packet_free(&video_decoder->packet);
  }

  if (NULL != video_decoder->hw_copied_frame) {
    av_frame_free(&video_decoder->hw_copied_frame);
  }

  if (NULL != video_decoder->hw_device_ctx) {
    av_buffer_unref(&video_decoder->hw_device_ctx);
  }

  if (NULL != video_decoder->parser_ctx) {
    av_parser_close(video_decoder->parser_ctx);
  }

  if (NULL != video_decoder->codec_ctx) {
    avcodec_free_context(&video_decoder->codec_ctx);
  }

  free(video_decoder);
}