use crate::{
    error::MirrorXError,
    ffi::ffmpeg::{avcodec::*, avutil::*},
    service::endpoint::message::{
        EndPointMessage, EndPointMessagePacket, EndPointMessagePacketType, VideoFrame,
    },
};
use anyhow::anyhow;
use std::{collections::HashMap, ffi::CString};
use tokio::sync::mpsc::Sender;
use tracing::warn;

pub struct VideoEncoder {
    codec: *const AVCodec,
    codec_ctx: *mut AVCodecContext,
    frame: *mut AVFrame,
    packet: *mut AVPacket,
}

unsafe impl Send for VideoEncoder {}
unsafe impl Sync for VideoEncoder {}

impl VideoEncoder {
    pub fn new(
        encoder_name: &str,
        fps: i32,
        width: i32,
        height: i32,
        options: HashMap<&str, &str>,
    ) -> Result<VideoEncoder, MirrorXError> {
        let encoder_name_ptr = CString::new(encoder_name.to_string())
            .map_err(|err| MirrorXError::Other(anyhow!(err)))?;

        unsafe {
            av_log_set_level(AV_LOG_TRACE);
            av_log_set_flags(AV_LOG_SKIP_REPEATED);

            let codec = avcodec_find_encoder_by_name(encoder_name_ptr.as_ptr());
            if codec.is_null() {
                return Err(MirrorXError::MediaVideoEncoderNotFound(
                    encoder_name.to_string(),
                ));
            }

            let codec_ctx = avcodec_alloc_context3(codec);
            if codec_ctx.is_null() {
                return Err(MirrorXError::MediaVideoEncoderAllocContextFailed);
            }

            (*codec_ctx).width = width;
            (*codec_ctx).height = height;
            (*codec_ctx).time_base = AVRational { num: 1, den: fps };
            (*codec_ctx).gop_size = fps * 2;
            (*codec_ctx).bit_rate = 4000 * 1000;
            (*codec_ctx).rc_max_rate = 4000 * 1000;
            (*codec_ctx).rc_min_rate = 4000 * 1000;
            (*codec_ctx).rc_buffer_size = 4000 * 1000 * 2;
            (*codec_ctx).has_b_frames = 0;
            (*codec_ctx).max_b_frames = 0;
            (*codec_ctx).pix_fmt = AV_PIX_FMT_NV12;
            (*codec_ctx).flags |= AV_CODEC_FLAG2_LOCAL_HEADER;
            (*codec_ctx).color_range = AVCOL_RANGE_JPEG;
            (*codec_ctx).color_primaries = AVCOL_PRI_BT709;
            (*codec_ctx).color_trc = AVCOL_TRC_BT709;
            (*codec_ctx).colorspace = AVCOL_SPC_BT709;

            for (k, v) in options {
                Self::set_opt(codec_ctx, k, v, 0)?;
            }

            let ret = avcodec_open2(codec_ctx, codec, std::ptr::null_mut());
            if ret != 0 {
                return Err(MirrorXError::MediaVideoEncoderOpenFailed(ret));
            }

            Ok(VideoEncoder {
                codec,
                codec_ctx,
                frame: std::ptr::null_mut(),
                packet: std::ptr::null_mut(),
            })
        }
    }

    fn set_opt(
        codec_ctx: *mut AVCodecContext,
        key: &str,
        value: &str,
        search_flags: i32,
    ) -> Result<(), MirrorXError> {
        let opt_name =
            CString::new(key.to_string()).map_err(|err| MirrorXError::Other(anyhow!(err)))?;
        let opt_value =
            CString::new(value.to_string()).map_err(|err| MirrorXError::Other(anyhow!(err)))?;

        unsafe {
            let ret = av_opt_set(
                (*codec_ctx).priv_data,
                opt_name.as_ptr(),
                opt_value.as_ptr(),
                search_flags,
            );

            if ret == AVERROR_OPTION_NOT_FOUND {
                return Err(MirrorXError::MediaVideoEncoderOptionNotFound(
                    key.to_string(),
                ));
            } else if ret == AVERROR(libc::ERANGE) {
                return Err(MirrorXError::MediaVideoEncoderOptionValueOutOfRange {
                    key: key.to_string(),
                    value: value.to_string(),
                });
            } else if ret == AVERROR(libc::EINVAL) {
                return Err(MirrorXError::MediaVideoEncoderOptionValueInvalid {
                    key: key.to_string(),
                    value: value.to_string(),
                });
            } else if ret != 0 {
                return Err(MirrorXError::MediaVideoEncoderOptionSetFailed {
                    key: key.to_string(),
                    value: value.to_string(),
                    error_code: ret,
                });
            } else {
                Ok(())
            }
        }
    }

    pub fn encode(
        &mut self,
        frame: crate::component::desktop::Frame,
        tx: &Sender<EndPointMessagePacket>,
    ) -> Result<(), MirrorXError> {
        unsafe {
            let mut ret: i32;

            if self.frame.is_null()
                || (*self.frame).width != frame.width as i32
                || (*self.frame).height != frame.height as i32
            {
                if !self.frame.is_null() {
                    av_frame_free(&mut self.frame);
                }

                if !self.packet.is_null() {
                    av_packet_free(&mut self.packet);
                }

                let new_frame = av_frame_alloc();
                if new_frame.is_null() {
                    return Err(MirrorXError::MediaVideoEncoderAVFrameAllocFailed);
                }

                (*new_frame).width = frame.width as i32;
                (*new_frame).height = frame.height as i32;
                (*new_frame).format = AV_PIX_FMT_NV12;
                (*new_frame).color_range = AVCOL_RANGE_JPEG;

                ret = av_frame_get_buffer(new_frame, 1);
                if ret < 0 {
                    return Err(MirrorXError::MediaVideoEncoderAVFrameGetBufferFailed(ret));
                }

                let packet = av_packet_alloc();
                if packet.is_null() {
                    return Err(MirrorXError::MediaVideoEncoderAVPacketAllocFailed);
                }

                let packet_size = av_image_get_buffer_size(
                    (*new_frame).format,
                    frame.width as i32,
                    frame.height as i32,
                    32,
                );

                ret = av_new_packet(packet, packet_size);
                if ret < 0 {
                    return Err(MirrorXError::MediaVideoEncoderAVPacketCreateFailed(ret));
                }

                self.frame = new_frame;
                self.packet = packet;
            }

            ret = av_frame_make_writable(self.frame);
            if ret < 0 {
                return Err(MirrorXError::MediaVideoEncoderAVFrameMakeWritableFailed(
                    ret,
                ));
            }

            (*self.frame).data[0] = frame.luminance_buffer.as_ptr() as *mut _;
            (*self.frame).linesize[0] = frame.luminance_stride as i32;
            (*self.frame).data[1] = frame.chrominance_buffer.as_ptr() as *mut _;
            (*self.frame).linesize[1] = frame.chrominance_stride as i32;
            // (*self.frame).pts = av_rescale_q(
            //     frame.capture_time,
            //     AV_TIME_BASE_Q,
            //     (*self.codec_ctx).time_base,
            // );
            (*self.frame).pts = chrono::Utc::now().timestamp_millis();

            ret = avcodec_send_frame(self.codec_ctx, self.frame);

            if ret != 0 {
                if ret == AVERROR(libc::EAGAIN) {
                    return Err(MirrorXError::MediaVideoEncoderFrameUnacceptable);
                } else if ret == AVERROR_EOF {
                    return Err(MirrorXError::MediaVideoEncoderClosed);
                }
                return Err(MirrorXError::MediaVideoEncoderSendFrameFailed(ret));
            }

            loop {
                ret = avcodec_receive_packet(self.codec_ctx, self.packet);
                if ret == AVERROR(libc::EAGAIN) || ret == AVERROR_EOF {
                    return Ok(());
                } else if ret < 0 {
                    return Err(MirrorXError::MediaVideoDecoderReceiveFrameFailed(ret));
                }

                let buffer =
                    std::slice::from_raw_parts((*self.packet).data, (*self.packet).size as usize)
                        .to_vec();

                let packet = EndPointMessagePacket {
                    typ: EndPointMessagePacketType::Push,
                    call_id: None,
                    message: EndPointMessage::VideoFrame(VideoFrame {
                        buffer,
                        pts: (*self.packet).pts,
                    }),
                };

                if let Err(err) = tx.try_send(packet) {
                    match err {
                        tokio::sync::mpsc::error::TrySendError::Full(_) => {
                            warn!("network send channel is full")
                        }
                        tokio::sync::mpsc::error::TrySendError::Closed(_) => {
                            return Err(MirrorXError::Other(anyhow::anyhow!(
                                "network send channel is closed"
                            )));
                        }
                    }
                }

                av_packet_unref(self.packet);
            }
        }
    }
}

impl Drop for VideoEncoder {
    fn drop(&mut self) {
        unsafe {
            if !self.codec_ctx.is_null() {
                avcodec_send_frame(self.codec_ctx, std::ptr::null_mut());
                avcodec_free_context(&mut self.codec_ctx);
            }

            if !self.frame.is_null() {
                av_frame_free(&mut self.frame);
            }

            if !self.packet.is_null() {
                av_packet_free(&mut self.packet);
            }
        }
    }
}
