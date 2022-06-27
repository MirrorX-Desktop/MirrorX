use crate::media::ffmpeg::avcodec::avcodec::avcodec_free_context;
use crate::{
    error::MirrorXError,
    media::{
        ffmpeg::{
            avcodec::{
                avcodec::{
                    avcodec_alloc_context3, avcodec_open2, avcodec_receive_packet,
                    avcodec_send_frame, AVCodecContext, AV_CODEC_FLAG2_LOCAL_HEADER,
                },
                codec::{avcodec_find_encoder_by_name, AVCodec},
                packet::{
                    av_new_packet, av_packet_alloc, av_packet_free, av_packet_unref, AVPacket,
                },
            },
            avutil::{
                error::{AVERROR, AVERROR_EOF, AVERROR_OPTION_NOT_FOUND},
                frame::{
                    av_frame_alloc, av_frame_free, av_frame_get_buffer, av_frame_make_writable,
                    AVFrame,
                },
                imgutils::av_image_get_buffer_size,
                log::{av_log_set_flags, av_log_set_level, AV_LOG_SKIP_REPEATED, AV_LOG_TRACE},
                opt::av_opt_set,
                pixfmt::*,
                rational::AVRational,
            },
        },
        frame::CaptureFrame,
        video_packet::VideoPacket,
    },
};
use anyhow::anyhow;
use crossbeam::channel::{bounded, Receiver, Sender};
use once_cell::sync::OnceCell;
use scopeguard::defer;
use std::{ffi::CString, slice::from_raw_parts};
use tracing::error;

pub struct VideoEncoder {
    codec: *const AVCodec,
    codec_ctx: *mut AVCodecContext,
    frame: *mut AVFrame,
    packet: *mut AVPacket,
    output_tx: OnceCell<Sender<VideoPacket>>,
}

unsafe impl Send for VideoEncoder {}
unsafe impl Sync for VideoEncoder {}

impl VideoEncoder {
    pub fn new(
        encoder_name: &str,
        fps: i32,
        width: i32,
        height: i32,
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
            (*codec_ctx).time_base = AVRational {
                num: 1,
                den: fps * 100,
            };
            (*codec_ctx).framerate = AVRational { num: fps, den: 1 };
            (*codec_ctx).gop_size = fps * 3;
            (*codec_ctx).bit_rate = 80000000;
            (*codec_ctx).rc_max_rate = 80000000;
            (*codec_ctx).rc_min_rate = 80000000;
            (*codec_ctx).rc_buffer_size = 80000000;
            (*codec_ctx).rc_initial_buffer_occupancy = (*codec_ctx).rc_buffer_size * 3 / 4;
            (*codec_ctx).has_b_frames = 0;
            (*codec_ctx).max_b_frames = 0;
            (*codec_ctx).bit_rate_tolerance = 1;
            (*codec_ctx).thread_count = 2;
            (*codec_ctx).pix_fmt = AV_PIX_FMT_NV12;
            (*codec_ctx).flags |= AV_CODEC_FLAG2_LOCAL_HEADER;
            (*codec_ctx).color_range = AVCOL_RANGE_JPEG;
            (*codec_ctx).color_primaries = AVCOL_PRI_BT709;
            (*codec_ctx).color_trc = AVCOL_PRI_BT709;
            (*codec_ctx).colorspace = AVCOL_SPC_BT709;

            Ok(VideoEncoder {
                codec,
                codec_ctx,
                frame: std::ptr::null_mut(),
                packet: std::ptr::null_mut(),
                output_tx: OnceCell::new(),
            })
        }
    }

    pub fn set_opt(&self, key: &str, value: &str, search_flags: i32) -> Result<(), MirrorXError> {
        let opt_name =
            CString::new(key.to_string()).map_err(|err| MirrorXError::Other(anyhow!(err)))?;
        let opt_value =
            CString::new(value.to_string()).map_err(|err| MirrorXError::Other(anyhow!(err)))?;

        unsafe {
            let ret = av_opt_set(
                (*self.codec_ctx).priv_data,
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

    pub fn open(&mut self) -> Result<Receiver<VideoPacket>, MirrorXError> {
        if self.output_tx.get().is_some() {
            return Err(MirrorXError::MediaVideoEncoderAlreadyOpened);
        }

        unsafe {
            let ret = avcodec_open2(self.codec_ctx, self.codec, std::ptr::null_mut());
            if ret != 0 {
                return Err(MirrorXError::MediaVideoEncoderOpenFailed(ret));
            }

            let (tx, rx) = bounded::<VideoPacket>(600);
            self.output_tx.set(tx);
            Ok(rx)
        }
    }

    pub fn encode(&mut self, capture_frame: CaptureFrame) -> Result<(), MirrorXError> {
        unsafe {
            let mut ret: i32;

            if self.frame.is_null()
                || (*self.frame).width != capture_frame.width() as i32
                || (*self.frame).height != capture_frame.height() as i32
            {
                if !self.frame.is_null() {
                    av_frame_free(&mut self.frame);
                }

                if !self.packet.is_null() {
                    av_packet_free(&mut self.packet);
                }

                let frame = av_frame_alloc();
                if frame.is_null() {
                    return Err(MirrorXError::MediaVideoEncoderAVFrameAllocFailed);
                }

                (*frame).width = capture_frame.width() as i32;
                (*frame).height = capture_frame.height() as i32;
                (*frame).format = AV_PIX_FMT_NV12;
                (*frame).color_range = AVCOL_RANGE_JPEG;

                ret = av_frame_get_buffer(frame, 1);
                if ret < 0 {
                    return Err(MirrorXError::MediaVideoEncoderAVFrameGetBufferFailed(ret));
                }

                let packet = av_packet_alloc();
                if packet.is_null() {
                    return Err(MirrorXError::MediaVideoEncoderAVPacketAllocFailed);
                }

                let packet_size = av_image_get_buffer_size(
                    (*frame).format,
                    capture_frame.width() as i32,
                    capture_frame.height() as i32,
                    32,
                );

                ret = av_new_packet(packet, packet_size);
                if ret < 0 {
                    return Err(MirrorXError::MediaVideoEncoderAVPacketCreateFailed(ret));
                }

                self.frame = frame;
                self.packet = packet;
            }

            ret = av_frame_make_writable(self.frame);
            if ret < 0 {
                return Err(MirrorXError::MediaVideoEncoderAVFrameMakeWritableFailed(
                    ret,
                ));
            }

            (*self.frame).data[0] = capture_frame.luminance_buffer().as_ptr() as *mut _;
            (*self.frame).linesize[0] = capture_frame.luminance_stride() as i32;
            (*self.frame).data[1] = capture_frame.chrominance_buffer().as_ptr() as *mut _;
            (*self.frame).linesize[1] = capture_frame.chrominance_stride() as i32;
            // (*self.frame).time_base.num = 1;
            // (*self.frame).time_base.den = pts_scale;
            // (*self.frame).pts = pts;

            ret = avcodec_send_frame(self.codec_ctx, self.frame);

            capture_frame.notify_frame_release();

            if ret != 0 {
                if ret == AVERROR(libc::EAGAIN) {
                    return Err(MirrorXError::MediaVideoEncoderFrameUnacceptable);
                } else if ret == AVERROR_EOF {
                    return Err(MirrorXError::MediaVideoEncoderClosed);
                }
                return Err(MirrorXError::MediaVideoEncoderSendFrameFailed(ret));
            }

            let mut err = None;
            while ret >= 0 && err.is_none() {
                ret = avcodec_receive_packet(self.codec_ctx, self.packet);
                if ret < 0 {
                    break;
                }

                if let Some(tx) = self.output_tx.get() {
                    let video_packet = VideoPacket {
                        data: std::slice::from_raw_parts(
                            (*self.packet).data,
                            (*self.packet).size as usize,
                        )
                        .to_vec(),
                        dts: (*self.packet).dts,
                        pts: (*self.packet).pts,
                    };

                    if let Err(_) = tx.try_send(video_packet) {
                        err = Some(MirrorXError::MediaVideoEncoderOutputTxSendFailed);
                    }
                }

                av_packet_unref(self.packet);
            }

            if ret == AVERROR(libc::EAGAIN) || ret == AVERROR_EOF {
                Ok(())
            } else if ret < 0 {
                Err(MirrorXError::MediaVideoEncoderReceivePacketFailed(ret))
            } else if let Some(err) = err {
                Err(err)
            } else {
                Ok(())
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
