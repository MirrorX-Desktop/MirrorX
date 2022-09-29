use crate::{
    api::endpoint::flutter_message::FlutterMediaMessage,
    core_error,
    error::{CoreError, CoreResult},
    ffi::ffmpeg::{avcodec::*, avutil::*},
};
use bytes::BufMut;
use flutter_rust_bridge::{StreamSink, ZeroCopyBuffer};
use std::ffi::{CStr, CString};

pub struct VideoDecoder {
    decode_context: Option<DecodeContext>,
    texture_id: i64,
    stream: StreamSink<FlutterMediaMessage>,
}

unsafe impl Send for VideoDecoder {}

impl VideoDecoder {
    pub fn new(texture_id: i64, stream: StreamSink<FlutterMediaMessage>) -> VideoDecoder {
        unsafe {
            // av_log_set_level(AV_LOG_TRACE);
            // av_log_set_flags(AV_LOG_SKIP_REPEATED);

            VideoDecoder {
                decode_context: None,
                texture_id,
                stream,
            }
        }
    }

    pub async fn decode(
        &mut self,
        mut video_frame: crate::api::endpoint::message::EndPointVideoFrame,
    ) -> CoreResult<()> {
        unsafe {
            if let Some(decode_context) = self.decode_context.as_ref() {
                if (*decode_context.codec_ctx).width != video_frame.width
                    || (*decode_context.codec_ctx).height != video_frame.height
                {
                    self.decode_context = None;
                }
            }

            if self.decode_context.is_none() {
                self.decode_context =
                    Some(DecodeContext::new(video_frame.width, video_frame.height)?);
            }

            let decode_context = if let Some(decode_context) = self.decode_context.as_ref() {
                decode_context
            } else {
                return Err(core_error!("decode context is null"));
            };

            if !decode_context.parser_ctx.is_null() {
                let ret = av_parser_parse2(
                    (decode_context).parser_ctx,
                    (decode_context).codec_ctx,
                    &mut (*(decode_context).packet).data,
                    &mut (*(decode_context).packet).size,
                    video_frame.buffer.as_ptr(),
                    video_frame.buffer.len() as i32,
                    AV_NOPTS_VALUE,
                    AV_NOPTS_VALUE,
                    0,
                );

                if ret < 0 {
                    return Err(core_error!("av_parser_parse2 returns error code: {}", ret));
                }
            } else {
                (*(decode_context).packet).data = video_frame.buffer.as_mut_ptr();
                (*(decode_context).packet).size = video_frame.buffer.len() as i32;
                (*(decode_context).packet).pts = video_frame.pts;
            }

            let mut ret = avcodec_send_packet((decode_context).codec_ctx, (decode_context).packet);

            if ret == AVERROR(libc::EAGAIN) {
                return Err(core_error!("avcodec_send_packet returns EAGAIN"));
            } else if ret == AVERROR_EOF {
                return Err(core_error!("avcodec_send_packet returns AVERROR_EOF"));
            } else if ret < 0 {
                return Err(core_error!(
                    "avcodec_send_packet returns error code: {}",
                    ret
                ));
            }

            loop {
                ret = avcodec_receive_frame(
                    (decode_context).codec_ctx,
                    (decode_context).decode_frame,
                );
                if ret == AVERROR(libc::EAGAIN) || ret == AVERROR_EOF {
                    return Ok(());
                } else if ret < 0 {
                    return Err(core_error!(
                        "avcodec_receive_frame returns error code: {}",
                        ret
                    ));
                }

                let tmp_frame = if !(decode_context).parser_ctx.is_null() {
                    (decode_context).decode_frame
                } else {
                    let ret = av_hwframe_transfer_data(
                        (decode_context).hw_decode_frame,
                        (decode_context).decode_frame,
                        0,
                    );
                    if ret < 0 {
                        return Err(core_error!(
                            "av_hwframe_transfer_data returns error code: {}",
                            ret
                        ));
                    }

                    (decode_context).hw_decode_frame
                };

                tracing::info!("frame pts: {}", (*tmp_frame).pts);

                // 8: id
                // 4: width
                // 4: height
                // 4: lumina stride
                // 4: chroma stride
                // 4: lumina body length
                // n: lumina body
                // 4: chroma body length
                // n: chroma body

                let width = (*tmp_frame).width;
                let height = (*tmp_frame).height;
                let luminance_stride = (*tmp_frame).linesize[0];
                let chrominance_stride = (*tmp_frame).linesize[1];
                let luminance_bytes_length = height * luminance_stride;
                let chrominance_bytes_length = height * chrominance_stride / 2;
                let luminance_bytes = std::slice::from_raw_parts(
                    (*tmp_frame).data[0],
                    luminance_bytes_length as usize,
                );
                let chrominance_bytes = std::slice::from_raw_parts(
                    (*tmp_frame).data[1],
                    chrominance_bytes_length as usize,
                );

                let mut video_frame_buffer = Vec::<u8>::with_capacity(
                    24 + 4
                        + (luminance_bytes_length as usize)
                        + 4
                        + (chrominance_bytes_length as usize),
                );

                video_frame_buffer.put_i64_le(self.texture_id);
                video_frame_buffer.put_i32_le(width);
                video_frame_buffer.put_i32_le(height);
                video_frame_buffer.put_i32_le(luminance_stride);
                video_frame_buffer.put_i32_le(chrominance_stride);
                video_frame_buffer.put_i32_le(luminance_bytes_length);
                video_frame_buffer.put_slice(luminance_bytes);
                video_frame_buffer.put_i32_le(chrominance_bytes_length);
                video_frame_buffer.put_slice(chrominance_bytes);

                av_frame_unref(tmp_frame);

                if !self.stream.add(FlutterMediaMessage::Video(ZeroCopyBuffer(
                    video_frame_buffer,
                ))) {
                    return Err(core_error!("decoded frame tx is disconnected"));
                }
            }
        }
    }
}

impl Drop for VideoDecoder {
    fn drop(&mut self) {
        // if !self.decode_context.is_null() {
        //     unsafe {
        //         let _ = Box::from_raw(self.decode_context);
        //     }
        // }
    }
}

struct DecodeContext {
    codec_ctx: *mut AVCodecContext,
    parser_ctx: *mut AVCodecParserContext,
    packet: *mut AVPacket,
    decode_frame: *mut AVFrame,
    hw_decode_frame: *mut AVFrame,
}

impl DecodeContext {
    fn new(width: i32, height: i32) -> CoreResult<DecodeContext> {
        unsafe {
            let mut decode_ctx = DecodeContext::default();

            let codec = avcodec_find_decoder(AV_CODEC_ID_H264);
            if codec.is_null() {
                return Err(core_error!("avcodec_find_decoder returns null"));
            }

            decode_ctx.codec_ctx = avcodec_alloc_context3(codec);
            if decode_ctx.codec_ctx.is_null() {
                return Err(core_error!("avcodec_alloc_context3 returns null"));
            }

            (*decode_ctx.codec_ctx).width = width;
            (*decode_ctx.codec_ctx).height = height;
            (*decode_ctx.codec_ctx).framerate = AVRational { num: 60, den: 1 };
            (*decode_ctx.codec_ctx).time_base = AVRational {
                num: 1,
                den: 1000000,
            };
            (*decode_ctx.codec_ctx).pix_fmt = AV_PIX_FMT_NV12;
            (*decode_ctx.codec_ctx).color_range = AVCOL_RANGE_JPEG;
            (*decode_ctx.codec_ctx).color_primaries = AVCOL_PRI_BT709;
            (*decode_ctx.codec_ctx).color_trc = AVCOL_TRC_BT709;
            (*decode_ctx.codec_ctx).colorspace = AVCOL_SPC_BT709;

            let mut hw_device_type = av_hwdevice_find_type_by_name(
                CString::new(if cfg!(target_os = "windows") {
                    "d3d11va"
                } else {
                    "videotoolbox"
                })?
                .as_ptr(),
            );

            if hw_device_type == AV_HWDEVICE_TYPE_NONE {
                tracing::error!("current environment does't support 'd3d11va'");

                let mut devices = Vec::new();
                loop {
                    hw_device_type = av_hwdevice_iterate_types(hw_device_type);
                    if hw_device_type == AV_HWDEVICE_TYPE_NONE {
                        break;
                    }

                    let device_name = av_hwdevice_get_type_name(hw_device_type);

                    devices.push(
                        CStr::from_ptr(device_name)
                            .to_str()
                            .map_or("unknown", |v| v),
                    );
                }

                tracing::info!(?devices, "support hw device");
                tracing::info!("init software decoder");

                decode_ctx.parser_ctx = av_parser_init((*codec).id);
                if decode_ctx.parser_ctx.is_null() {
                    return Err(core_error!("av_parser_init returns null"));
                }
            } else {
                let mut hwdevice_ctx = std::ptr::null_mut();

                let ret = av_hwdevice_ctx_create(
                    &mut hwdevice_ctx,
                    hw_device_type,
                    std::ptr::null(),
                    std::ptr::null_mut(),
                    0,
                );

                if ret < 0 {
                    return Err(core_error!(
                        "av_hwdevice_ctx_create returns error code: {}",
                        ret,
                    ));
                }

                (*decode_ctx.codec_ctx).hw_device_ctx = av_buffer_ref(hwdevice_ctx);
            }

            decode_ctx.packet = av_packet_alloc();
            if decode_ctx.packet.is_null() {
                return Err(core_error!("av_packet_alloc returns null"));
            }

            decode_ctx.decode_frame = av_frame_alloc();
            if decode_ctx.decode_frame.is_null() {
                return Err(core_error!("av_frame_alloc returns null"));
            }

            decode_ctx.hw_decode_frame = av_frame_alloc();
            if decode_ctx.hw_decode_frame.is_null() {
                return Err(core_error!("av_frame_alloc returns null"));
            }

            let ret = avcodec_open2(decode_ctx.codec_ctx, codec, std::ptr::null_mut());
            if ret != 0 {
                return Err(core_error!("avcodec_open2 returns error code: {}", ret));
            }

            Ok(decode_ctx)
        }
    }
}

impl Default for DecodeContext {
    fn default() -> Self {
        Self {
            codec_ctx: std::ptr::null_mut(),
            parser_ctx: std::ptr::null_mut(),
            packet: std::ptr::null_mut(),
            decode_frame: std::ptr::null_mut(),
            hw_decode_frame: std::ptr::null_mut(),
        }
    }
}

impl Drop for DecodeContext {
    fn drop(&mut self) {
        unsafe {
            if !self.codec_ctx.is_null() {
                avcodec_send_packet(self.codec_ctx, std::ptr::null());
            }

            if !self.hw_decode_frame.is_null() {
                av_frame_free(&mut self.hw_decode_frame);
            }

            if !self.parser_ctx.is_null() {
                av_parser_close(self.parser_ctx);
            }

            if !self.decode_frame.is_null() {
                av_frame_free(&mut self.decode_frame);
            }

            if !self.packet.is_null() {
                av_packet_free(&mut self.packet);
            }

            if !self.codec_ctx.is_null() {
                if !(*self.codec_ctx).hw_device_ctx.is_null() {
                    av_buffer_ref((*self.codec_ctx).hw_device_ctx);
                }
                avcodec_free_context(&mut self.codec_ctx);
            }
        }
    }
}

// unsafe fn convert_yuv_to_rgb(frame: *mut AVFrame) -> CoreResult<Vec<u8>> {
//     let argb_stride = 4 * ((32 * (*frame).width + 31) / 32);
//     let argb_frame_size = (argb_stride as usize) * ((*frame).height as usize) * 4;
//     let mut argb_frame_buffer = Vec::<u8>::with_capacity(argb_frame_size);

//     let ret = NV21ToARGBMatrix(
//         (*frame).data[0],
//         (*frame).linesize[0] as isize,
//         (*frame).data[1],
//         (*frame).linesize[1] as isize,
//         argb_frame_buffer.as_mut_ptr(),
//         argb_stride as isize,
//         &kYvuF709Constants,
//         (*frame).width as isize,
//         (*frame).height as isize,
//     );

//     if ret != 0 {
//         return Err(core_error!("NV21ToARGBMatrix returns error code: {}", ret));
//     }

//     argb_frame_buffer.set_len(argb_frame_size);

//     Ok(argb_frame_buffer)
// }
