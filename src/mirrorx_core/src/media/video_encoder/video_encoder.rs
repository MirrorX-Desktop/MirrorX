use crate::media::{
    ffmpeg::{
        avcodec::{
            avcodec::*,
            codec::{avcodec_find_encoder_by_name, AVCodec},
            packet::*,
        },
        avutil::{
            error::*,
            frame::*,
            imgutils::*,
            log::{av_log_set_flags, av_log_set_level, AV_LOG_SKIP_REPEATED, AV_LOG_TRACE},
            opt::av_opt_set,
            pixfmt::*,
            rational::AVRational,
        },
    },
    video_packet::VideoPacket,
};
use anyhow::{bail, Ok};
use crossbeam_channel::{bounded, Receiver, Sender};
use std::{
    ffi::CString,
    ptr::{self, null_mut},
    slice::from_raw_parts,
};

pub struct VideoEncoder {
    codec: *const AVCodec,
    codec_ctx: *mut AVCodecContext,
    frame: *mut AVFrame,
    packet: *mut AVPacket,
    output_tx: Option<Sender<VideoPacket>>,
}

unsafe impl Send for VideoEncoder {}
unsafe impl Sync for VideoEncoder {}

impl VideoEncoder {
    pub fn new(
        encoder_name: &str,
        fps: i32,
        screen_width: u16,
        screen_height: u16,
    ) -> anyhow::Result<VideoEncoder> {
        let encoder_name_ptr = CString::new(encoder_name.to_string())?;

        unsafe {
            // let video_encoder_ptr = bindings::video_encoder_create(
            //     encoder_name_ptr.as_ptr(),
            //     screen_width as c_int,
            //     screen_height as c_int,
            //     fps,
            // );

            // if video_encoder_ptr.is_null() {
            //     bail!("create video encoder failed");
            // }

            // bindings::video_encoder_set_opt(
            //     video_encoder_ptr,
            //     CString::new("profile")?.as_ptr(),
            //     CString::new("high")?.as_ptr(),
            // );

            // bindings::video_encoder_set_opt(
            //     video_encoder_ptr,
            //     CString::new("level")?.as_ptr(),
            //     CString::new("5.2")?.as_ptr(),
            // );

            // match encoder_name {
            //     "libx264" => {
            //         bindings::video_encoder_set_opt(
            //             video_encoder_ptr,
            //             CString::new("preset")?.as_ptr(),
            //             CString::new("ultrafast")?.as_ptr(),
            //         );
            //         bindings::video_encoder_set_opt(
            //             video_encoder_ptr,
            //             CString::new("tune")?.as_ptr(),
            //             CString::new("zerolatency")?.as_ptr(),
            //         );

            //         bindings::video_encoder_set_opt(
            //             video_encoder_ptr,
            //             CString::new("sc_threshold")?.as_ptr(),
            //             CString::new("499")?.as_ptr(),
            //         );
            //     }
            //     "h264_videotoolbox" => {
            //         bindings::video_encoder_set_opt(
            //             video_encoder_ptr,
            //             CString::new("realtime")?.as_ptr(),
            //             CString::new("1")?.as_ptr(),
            //         );
            //         bindings::video_encoder_set_opt(
            //             video_encoder_ptr,
            //             CString::new("allow_sw")?.as_ptr(),
            //             CString::new("0")?.as_ptr(),
            //         );
            //     }
            //     _ => {}
            // };

            // if !bindings::video_encoder_open(video_encoder_ptr) {
            //     bail!("open video encoder failed");
            // }

            // Ok((
            //     VideoEncoder {
            //         inner_video_encoder: video_encoder_ptr,
            //         tx,
            //     },
            //     rx,
            // ))

            av_log_set_level(AV_LOG_TRACE);
            av_log_set_flags(AV_LOG_SKIP_REPEATED);

            let codec = avcodec_find_encoder_by_name(encoder_name_ptr.as_ptr());
            if codec.is_null() {
                bail!("find encoder failed");
            }

            let mut codec_ctx = avcodec_alloc_context3(codec);
            if codec_ctx.is_null() {
                bail!("alloc codec context failed");
            }

            (*codec_ctx).width = screen_width as i32;
            (*codec_ctx).height = screen_height as i32;
            (*codec_ctx).time_base = AVRational {
                num: 1,
                den: fps * 100,
            };
            (*codec_ctx).framerate = AVRational { num: fps, den: 1 };
            (*codec_ctx).gop_size = fps * 3;
            (*codec_ctx).has_b_frames = 0;
            (*codec_ctx).max_b_frames = 0;
            (*codec_ctx).qmax = 28;
            (*codec_ctx).qmin = 18;
            (*codec_ctx).thread_count = 2;
            (*codec_ctx).pix_fmt = AV_PIX_FMT_NV12;
            (*codec_ctx).flags |= AV_CODEC_FLAG2_LOCAL_HEADER;
            (*codec_ctx).color_range = AVCOL_RANGE_JPEG;
            (*codec_ctx).color_primaries = AVCOL_PRI_BT709;
            (*codec_ctx).color_trc = AVCOL_TRC_BT709;
            (*codec_ctx).colorspace = AVCOL_SPC_BT709;

            Ok(VideoEncoder {
                codec,
                codec_ctx,
                frame: ptr::null_mut(),
                packet: ptr::null_mut(),
                output_tx: None,
            })
        }
    }

    pub fn set_opt(&self, key: &str, value: &str, search_flags: i32) -> anyhow::Result<()> {
        let opt_name = CString::new(key.to_string())?;
        let opt_value = CString::new(value.to_string())?;

        unsafe {
            let ret = av_opt_set(
                (*self.codec_ctx).priv_data,
                opt_name.as_ptr(),
                opt_name.as_ptr(),
                search_flags,
            );

            if ret == AVERROR_OPTION_NOT_FOUND {
                bail!("option not found")
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
            let ret = avcodec_open2(self.codec_ctx, self.codec, ptr::null_mut());
            if ret != 0 {
                bail!("open encoder failed ret={}", ret)
            }

            let (tx, rx) = bounded::<VideoPacket>(600);
            self.output_tx = Some(tx);
            Ok(rx)
        }
    }

    pub fn encode(
        &self,
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
            let mut ret = 0;

            // when video rect changed, we need to reallocate frame and packet
            if self.frame.is_null()
                || self.packet.is_null()
                || (*self.frame).width != width
                || (*self.frame).height != height
            {
                // release old frame
                if !self.frame.is_null() {
                    av_frame_free(&mut self.frame);
                    self.frame = null_mut();
                }

                // release old packet
                if !self.packet.is_null() {
                    av_packet_free(&mut self.packet);
                    self.packet = null_mut();
                }

                // re-allocate new frame
                self.frame = av_frame_alloc();
                if self.frame.is_null() {
                    tracing::error!("av_frame_alloc failed");
                    return;
                }

                (*self.frame).width = width;
                (*self.frame).height = height;
                (*self.frame).format = AV_PIX_FMT_NV12;
                (*self.frame).color_range = AVCOL_RANGE_JPEG;
                (*self.frame).color_primaries = AVCOL_PRI_BT709;
                (*self.frame).color_trc = AVCOL_TRC_BT709;
                (*self.frame).color_space = AVCOL_SPC_BT709;

                ret = av_frame_get_buffer(self.frame, 32);
                if ret < 0 {
                    tracing::error!(ret = ret, "av_frame_get_buffer failed");
                    return;
                }

                // re-allocate new packet
                self.packet = av_packet_alloc();
                if self.packet.is_null() {
                    tracing::error!("av_packet_alloc failed");
                    return;
                }

                let packet_size = av_image_get_buffer_size(
                    (*self.frame).format,
                    (*self.frame).width,
                    (*self.frame).height,
                    32,
                );

                ret = av_new_packet(self.packet, packet_size);
                if ret < 0 {
                    tracing::error!(ret = ret, "av_new_packet failed");
                    return;
                }
            }

            ret = av_frame_make_writable(self.frame);
            if ret < 0 {
                tracing::error!(ret = ret, "av_frame_make_writable failed");
            }

            (*self.frame).width = width;
            (*self.frame).height = height;
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

                if let Some(tx) = self.output_tx {
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
            if !self.frame.is_null() {
                av_frame_free(&mut self.frame);
            }

            if !self.packet.is_null() {
                av_packet_free(&mut self.packet);
            }

            if !self.codec_ctx.is_null() {
                avcodec_send_frame(self.codec_ctx, ptr::null_mut());
                avcodec_free_context(&mut self.codec_ctx);
            }
        }
    }
}
