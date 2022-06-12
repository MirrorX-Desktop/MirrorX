use crate::media::{
    ffmpeg::{
        avcodec::{
            avcodec::{
                avcodec_alloc_context3, avcodec_open2, avcodec_receive_packet, avcodec_send_frame,
                AVCodecContext, AV_CODEC_FLAG2_LOCAL_HEADER,
            },
            codec::{avcodec_find_encoder_by_name, AVCodec},
            packet::{av_new_packet, av_packet_alloc, av_packet_free, av_packet_unref, AVPacket},
        },
        avutil::{
            error::{AVERROR, AVERROR_EOF, AVERROR_OPTION_NOT_FOUND},
            frame::{
                av_frame_alloc, av_frame_free, av_frame_get_buffer, av_frame_make_writable, AVFrame,
            },
            imgutils::av_image_get_buffer_size,
            log::{av_log_set_flags, av_log_set_level, AV_LOG_SKIP_REPEATED, AV_LOG_TRACE},
            opt::av_opt_set,
            pixfmt::{
                AVCOL_PRI_BT709, AVCOL_RANGE_JPEG, AVCOL_RANGE_MPEG, AVCOL_SPC_BT709,
                AVCOL_TRC_BT709, AVCOL_TRC_IEC61966_2_1, AV_PIX_FMT_NV12,
            },
            rational::AVRational,
        },
    },
    video_packet::VideoPacket,
};
use anyhow::{bail, Ok};
use crossbeam_channel::{bounded, Receiver, Sender};
use std::{ffi::CString, slice::from_raw_parts};
use tracing::error;

pub struct VideoEncoder {
    codec: *const AVCodec,
    codec_ctx: *mut AVCodecContext,
    frame: *mut AVFrame,
    packet: *mut AVPacket,
    output_tx: Option<Sender<VideoPacket>>,
    frame_height: i32,
    frame_width: i32,
}

unsafe impl Send for VideoEncoder {}
unsafe impl Sync for VideoEncoder {}

impl VideoEncoder {
    pub fn new(
        encoder_name: &str,
        fps: i32,
        frame_width: i32,
        frame_height: i32,
    ) -> anyhow::Result<VideoEncoder> {
        let encoder_name_ptr = CString::new(encoder_name.to_string())?;

        unsafe {
            av_log_set_level(AV_LOG_TRACE);
            av_log_set_flags(AV_LOG_SKIP_REPEATED);

            let mut ret: i32;

            let codec = avcodec_find_encoder_by_name(encoder_name_ptr.as_ptr());
            if codec.is_null() {
                bail!("find encoder failed");
            }

            let codec_ctx = avcodec_alloc_context3(codec);
            if codec_ctx.is_null() {
                bail!("alloc codec context failed");
            }

            (*codec_ctx).width = frame_width;
            (*codec_ctx).height = frame_height;
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
                output_tx: None,
                frame_height,
                frame_width,
            })
        }
    }

    pub fn frame_height(&self) -> i32 {
        self.frame_height
    }

    pub fn frame_width(&self) -> i32 {
        self.frame_width
    }

    pub fn set_opt(&self, key: &str, value: &str, search_flags: i32) -> anyhow::Result<()> {
        let opt_name = CString::new(key.to_string())?;
        let opt_value = CString::new(value.to_string())?;

        unsafe {
            let ret = av_opt_set(
                (*self.codec_ctx).priv_data,
                opt_name.as_ptr(),
                opt_value.as_ptr(),
                search_flags,
            );

            if ret == AVERROR_OPTION_NOT_FOUND {
                bail!("option not found key={} value={}", key, value);
            } else if ret == AVERROR(libc::ERANGE) {
                bail!("option value out of range")
            } else if ret == AVERROR(libc::EINVAL) {
                bail!("option value is invalid")
            } else if ret != 0 {
                bail!("set option failed ret={}", ret)
            } else {
                Ok(())
            }
        }
    }

    pub fn open(&mut self) -> anyhow::Result<Receiver<VideoPacket>> {
        if self.output_tx.is_some() {
            bail!("video encoder already opened");
        }

        unsafe {
            let ret = avcodec_open2(self.codec_ctx, self.codec, std::ptr::null_mut());
            if ret != 0 {
                bail!("open encoder failed ret={}", ret)
            }

            let (tx, rx) = bounded::<VideoPacket>(600);
            self.output_tx = Some(tx);
            Ok(rx)
        }
    }

    pub fn encode(
        &mut self,
        width: i32,
        height: i32,
        lumina_plane_bytes_address: *mut u8,
        lumina_plane_stride: i32,
        chrominance_plane_bytes_address: *mut u8,
        chrominance_plane_stride: i32,
        dts: i64,
        dts_scale: i32,
        pts: i64,
        pts_scale: i32,
    ) {
        unsafe {
            let mut ret: i32;

            if self.frame.is_null()
                || (*self.frame).width != width
                || (*self.frame).height != height
            {
                if !self.frame.is_null() {
                    av_frame_free(&mut self.frame);
                }

                if !self.packet.is_null() {
                    av_packet_free(&mut self.packet);
                }

                let frame = av_frame_alloc();
                if frame.is_null() {
                    error!("av frame alloc failed");
                    return;
                }

                (*frame).width = width;
                (*frame).height = height;
                (*frame).format = AV_PIX_FMT_NV12;

                ret = av_frame_get_buffer(frame, 1);
                if ret < 0 {
                    error!(ret = ret, "av_frame_get_buffer failed");
                    return;
                }

                let packet = av_packet_alloc();
                if packet.is_null() {
                    error!("av_packet_alloc failed");
                    return;
                }

                let packet_size = av_image_get_buffer_size((*frame).format, width, height, 32);

                ret = av_new_packet(packet, packet_size);
                if ret < 0 {
                    error!(ret = ret, "av_new_packet failed");
                    return;
                }

                self.frame = frame;
                self.packet = packet;
            }

            ret = av_frame_make_writable(self.frame);
            if ret < 0 {
                tracing::error!(ret = ret, "av_frame_make_writable failed");
            }

            (*self.frame).data[0] = lumina_plane_bytes_address;
            (*self.frame).linesize[0] = lumina_plane_stride;
            (*self.frame).data[1] = chrominance_plane_bytes_address;
            (*self.frame).linesize[1] = chrominance_plane_stride;
            (*self.frame).time_base.num = 1;
            (*self.frame).time_base.den = pts_scale;
            (*self.frame).pts = pts;

            ret = avcodec_send_frame(self.codec_ctx, self.frame);

            if ret != 0 {
                if ret == AVERROR(libc::EAGAIN) {
                    tracing::error!("can not send more frame to encoder");
                } else if ret == AVERROR_EOF {
                    tracing::error!("encoder closed");
                } else {
                    tracing::error!(ret = ret, "avcodec_send_frame failed");
                }
                return;
            }

            loop {
                ret = avcodec_receive_packet(self.codec_ctx, self.packet);

                if ret == AVERROR(libc::EAGAIN) || ret == AVERROR_EOF {
                    return;
                } else if ret < 0 {
                    tracing::error!(ret = ret, "avcodec_receive_packet failed");
                    return;
                }

                if let Some(tx) = &self.output_tx {
                    if let Err(err) = tx.try_send(VideoPacket {
                        data: from_raw_parts((*self.packet).data, (*self.packet).size as usize)
                            .to_vec(),
                        dts: (*self.packet).dts,
                        pts: (*self.packet).pts,
                    }) {
                        tracing::error!(err = ?err, "send encoded video packet failed");
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
            avcodec_send_frame(self.codec_ctx, std::ptr::null_mut());
        }
    }
}
