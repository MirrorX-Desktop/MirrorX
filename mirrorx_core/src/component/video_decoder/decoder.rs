use crate::{
    api::endpoint::message::EndPointVideoFrame,
    component::frame::{DesktopDecodeFrame, DesktopDecodeFrameFormat},
    core_error,
    error::CoreResult,
};
use mirrorx_native::ffmpeg::{
    codecs::{avcodec::*, codec::*, codec_id::*, packet::*},
    utils::{buffer::*, error::*, frame::*, hwcontext::*, pixfmt::*, rational::AVRational},
};
use tokio::sync::mpsc::Sender;

pub struct VideoDecoder {
    decode_context: Option<DecodeContext>,
    render_frame_tx: Sender<DesktopDecodeFrame>,
    _last_pts: i64,
}

impl VideoDecoder {
    pub fn new(render_frame_tx: Sender<DesktopDecodeFrame>) -> VideoDecoder {
        // unsafe {
        //     av_log_set_level(AV_LOG_TRACE);
        //     av_log_set_flags(AV_LOG_SKIP_REPEATED);
        // }

        VideoDecoder {
            decode_context: None,
            render_frame_tx,
            _last_pts: 0,
        }
    }

    pub fn decode(&mut self, mut video_frame: EndPointVideoFrame) -> CoreResult<()> {
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

            let Some(ref decode_context)= self.decode_context else{
                return Err(core_error!("decode context is empty"));
            };

            (*(decode_context).packet).data = video_frame.buffer.as_mut_ptr();
            (*(decode_context).packet).size = video_frame.buffer.len() as i32;
            (*(decode_context).packet).pts = video_frame.pts;
            (*(decode_context).packet).dts = video_frame.pts;

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

                let tmp_frame = if (*decode_context.codec_ctx).hw_device_ctx.is_null() {
                    decode_context.decode_frame
                } else {
                    // let transfer_instant = std::time::Instant::now();
                    let ret = av_hwframe_transfer_data(
                        (decode_context).hw_decode_frame,
                        (decode_context).decode_frame,
                        0,
                    );
                    // let cost = transfer_instant.elapsed();
                    // tracing::info!(?cost, "hardware decode frame transfer cost");

                    if ret < 0 {
                        return Err(core_error!(
                            "av_hwframe_transfer_data returns error code: {}",
                            ret
                        ));
                    }

                    (decode_context).hw_decode_frame
                };

                let (plane_data, line_sizes, format) = match (*tmp_frame).format {
                    AV_PIX_FMT_NV12 | AV_PIX_FMT_VIDEOTOOLBOX => (
                        vec![
                            std::slice::from_raw_parts(
                                (*tmp_frame).data[0],
                                ((*tmp_frame).linesize[0] * (*tmp_frame).height) as usize,
                            )
                            .to_vec(),
                            std::slice::from_raw_parts(
                                (*tmp_frame).data[1],
                                ((*tmp_frame).linesize[1] * (*tmp_frame).height / 2) as usize,
                            )
                            .to_vec(),
                        ],
                        vec![(*tmp_frame).linesize[0], (*tmp_frame).linesize[1]],
                        DesktopDecodeFrameFormat::NV12,
                    ),
                    AV_PIX_FMT_YUV420P | AV_PIX_FMT_YUVJ420P => (
                        vec![
                            std::slice::from_raw_parts(
                                (*tmp_frame).data[0],
                                ((*tmp_frame).linesize[0] * (*tmp_frame).height) as usize,
                            )
                            .to_vec(),
                            std::slice::from_raw_parts(
                                (*tmp_frame).data[1],
                                ((*tmp_frame).linesize[1] * (*tmp_frame).height / 2) as usize,
                            )
                            .to_vec(),
                            std::slice::from_raw_parts(
                                (*tmp_frame).data[2],
                                ((*tmp_frame).linesize[2] * (*tmp_frame).height / 2) as usize,
                            )
                            .to_vec(),
                        ],
                        vec![
                            (*tmp_frame).linesize[0],
                            (*tmp_frame).linesize[1],
                            (*tmp_frame).linesize[2],
                        ],
                        DesktopDecodeFrameFormat::YUV420P,
                    ),
                    _ => {
                        return Err(core_error!(
                            "unsupported format, pix_format: {}",
                            (*tmp_frame).format
                        ));
                    }
                };

                let desktop_decode_frame = DesktopDecodeFrame {
                    width: (*tmp_frame).width,
                    height: (*tmp_frame).height,
                    plane_data,
                    line_sizes,
                    format,
                };

                if self
                    .render_frame_tx
                    .blocking_send(desktop_decode_frame)
                    .is_err()
                {
                    return Err(core_error!("video render tx has closed"));
                }

                av_frame_unref(tmp_frame);
            }
        }
    }
}

struct DecodeContext {
    codec_ctx: *mut AVCodecContext,
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
            (*decode_ctx.codec_ctx).pix_fmt = AV_PIX_FMT_NV12;
            // (*decode_ctx.codec_ctx).color_range = AVCOL_RANGE_JPEG;
            // (*decode_ctx.codec_ctx).color_primaries = AVCOL_PRI_BT709;
            // (*decode_ctx.codec_ctx).color_trc = AVCOL_TRC_BT709;
            // (*decode_ctx.codec_ctx).colorspace = AVCOL_SPC_BT709;
            // (*decode_ctx.codec_ctx).flags2 |= AV_CODEC_FLAG2_LOCAL_HEADER;

            // let mut hw_device_type = av_hwdevice_find_type_by_name(
            //     CString::new(if cfg!(target_os = "windows") {
            //         "d3d11va"
            //     } else {
            //         "videotoolbox"
            //     })?
            //     .as_ptr(),
            // );

            // if hw_device_type == AV_HWDEVICE_TYPE_NONE {
            //     tracing::error!("current environment does't support hardware decode");

            //     let mut devices = Vec::new();
            //     loop {
            //         hw_device_type = av_hwdevice_iterate_types(hw_device_type);
            //         if hw_device_type == AV_HWDEVICE_TYPE_NONE {
            //             break;
            //         }

            //         let device_name = av_hwdevice_get_type_name(hw_device_type);

            //         devices.push(
            //             CStr::from_ptr(device_name)
            //                 .to_str()
            //                 .map_or("unknown", |v| v),
            //         );
            //     }

            //     tracing::info!(?devices, "current environment support hw device");
            //     tracing::info!("use software decoder");
            // } else {
            //     let mut hwdevice_ctx = std::ptr::null_mut();

            //     let ret = av_hwdevice_ctx_create(
            //         &mut hwdevice_ctx,
            //         hw_device_type,
            //         std::ptr::null(),
            //         std::ptr::null_mut(),
            //         0,
            //     );

            //     if ret < 0 {
            //         return Err(core_error!(
            //             "av_hwdevice_ctx_create returns error code: {}",
            //             ret,
            //         ));
            //     }

            //     (*decode_ctx.codec_ctx).hw_device_ctx = av_buffer_ref(hwdevice_ctx);
            // }

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

// unsafe fn convert_yuv_to_rgb(frame: *mut AVFrame) -> CoreResult<Vec<Color32>> {
//     let argb_stride = 4 * ((32 * (*frame).width + 31) / 32);
//     let argb_frame_size = (argb_stride as usize) * ((*frame).height as usize);
//     let mut color32_buffer = Vec::<Color32>::with_capacity(argb_frame_size / 4);

//     let ret = NV21ToARGBMatrix(
//         (*frame).data[0],
//         (*frame).linesize[0] as isize,
//         (*frame).data[1],
//         (*frame).linesize[1] as isize,
//         color32_buffer.as_mut_ptr() as *mut u8,
//         argb_stride as isize,
//         &kYvuF709Constants,
//         (*frame).width as isize,
//         (*frame).height as isize,
//     );

//     if ret != 0 {
//         return Err(core_error!("NV21ToARGBMatrix returns error code: {}", ret));
//     }

//     color32_buffer.set_len(argb_frame_size / 4);

//     Ok(color32_buffer)
// }
