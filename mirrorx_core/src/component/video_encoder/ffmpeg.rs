use super::ffmpeg_encoder_config::{FFMPEGEncoderType, Libx264Config};
use crate::{
    api::endpoint::message::{EndPointMessage, EndPointVideoFrame},
    component::{capture_frame::CaptureFrame, NALU_HEADER_LENGTH},
    core_error,
    error::{CoreError, CoreResult},
    ffi::ffmpeg::{avcodec::*, avutil::*},
};
use tokio::sync::mpsc::Sender;

pub struct Encoder {
    codec_ctx: *mut AVCodecContext,
    frame: *mut AVFrame,
    packet: *mut AVPacket,
}

unsafe impl Send for Encoder {}

impl Encoder {
    pub fn new(
        encoder_type: FFMPEGEncoderType,
        frame_width: i32,
        frame_height: i32,
    ) -> CoreResult<Encoder> {
        let encoder_config = match encoder_type {
            FFMPEGEncoderType::Libx264 => Libx264Config::new()?,
        };

        let mut encoder = Self {
            codec_ctx: std::ptr::null_mut(),
            frame: std::ptr::null_mut(),
            packet: std::ptr::null_mut(),
        };

        unsafe {
            av_log_set_level(AV_LOG_TRACE);
            av_log_set_flags(AV_LOG_SKIP_REPEATED);

            let codec = avcodec_find_encoder_by_name(encoder_config.ffmpeg_encoder_name());
            if codec.is_null() {
                return Err(core_error!(
                    "avcodec_find_encoder_by_name returns null pointer"
                ));
            }

            encoder.codec_ctx = avcodec_alloc_context3(codec);
            if encoder.codec_ctx.is_null() {
                return Err(core_error!("avcodec_alloc_context3 returns null pointer"));
            }

            (*encoder.codec_ctx).width = frame_width;
            (*encoder.codec_ctx).height = frame_height;
            (*encoder.codec_ctx).time_base = AVRational { num: 1, den: 1 };
            (*encoder.codec_ctx).gop_size = 60 * 2;
            (*encoder.codec_ctx).bit_rate = 4000 * 1000;
            (*encoder.codec_ctx).rc_max_rate = 4000 * 1000;
            (*encoder.codec_ctx).rc_min_rate = 4000 * 1000;
            (*encoder.codec_ctx).rc_buffer_size = 4000 * 1000 * 2;
            (*encoder.codec_ctx).has_b_frames = 0;
            (*encoder.codec_ctx).max_b_frames = 0;
            (*encoder.codec_ctx).pix_fmt = AV_PIX_FMT_NV12;
            (*encoder.codec_ctx).flags |= AV_CODEC_FLAG_GLOBAL_HEADER;
            (*encoder.codec_ctx).color_range = AVCOL_RANGE_JPEG;
            (*encoder.codec_ctx).color_primaries = AVCOL_PRI_BT709;
            (*encoder.codec_ctx).color_trc = AVCOL_TRC_BT709;
            (*encoder.codec_ctx).colorspace = AVCOL_SPC_BT709;

            encoder_config.apply_option(encoder.codec_ctx)?;

            let ret = avcodec_open2(encoder.codec_ctx, codec, std::ptr::null_mut());
            if ret != 0 {
                return Err(core_error!("avcodec_open2 returns null pointer"));
            }

            Ok(encoder)
        }
    }

    pub fn encode(&mut self, frame: CaptureFrame, tx: &Sender<EndPointMessage>) -> CoreResult<()> {
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
                    return Err(core_error!("av_frame_alloc returns null pointer"));
                }

                (*new_frame).width = frame.width as i32;
                (*new_frame).height = frame.height as i32;
                (*new_frame).format = AV_PIX_FMT_NV12;
                (*new_frame).color_range = AVCOL_RANGE_JPEG;

                ret = av_frame_get_buffer(new_frame, 1);
                if ret < 0 {
                    return Err(core_error!(
                        "av_frame_get_buffer returns error code: {}",
                        ret
                    ));
                }

                let packet = av_packet_alloc();
                if packet.is_null() {
                    return Err(core_error!("av_packet_alloc returns null pointer"));
                }

                let packet_size = av_image_get_buffer_size(
                    (*new_frame).format,
                    frame.width as i32,
                    frame.height as i32,
                    32,
                );

                ret = av_new_packet(packet, packet_size);
                if ret < 0 {
                    return Err(core_error!("av_new_packet returns error code: {}", ret));
                }

                self.frame = new_frame;
                self.packet = packet;
            }

            ret = av_frame_make_writable(self.frame);
            if ret < 0 {
                return Err(core_error!(
                    "av_frame_make_writable returns error code: {}",
                    ret
                ));
            }

            (*self.frame).data[0] = frame.luminance_bytes.as_ptr() as *mut _;
            (*self.frame).linesize[0] = frame.luminance_stride as i32;
            (*self.frame).data[1] = frame.chrominance_bytes.as_ptr() as *mut _;
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
                    return Err(core_error!("avcodec_send_frame returns EAGAIN"));
                } else if ret == AVERROR_EOF {
                    return Err(core_error!("avcodec_send_frame returns AVERROR_EOF"));
                }
                return Err(core_error!(
                    "avcodec_send_frame returns error code: {}",
                    ret
                ));
            }

            loop {
                ret = avcodec_receive_packet(self.codec_ctx, self.packet);
                if ret == AVERROR(libc::EAGAIN) || ret == AVERROR_EOF {
                    return Ok(());
                } else if ret < 0 {
                    return Err(core_error!(
                        "avcodec_receive_packet returns error code: {}",
                        ret
                    ));
                }

                let mut sps = once_cell::unsync::OnceCell::new();
                let mut pps = once_cell::unsync::OnceCell::new();

                let buffer =
                    std::slice::from_raw_parts((*self.packet).data, (*self.packet).size as usize)
                        .to_vec();

                // if packet has key frame, copy sps and pps from AVCodecContext
                if (*self.packet).flags & 0x0001 != 0 && (*self.codec_ctx).extradata_size > 0 {
                    let extra_data = std::slice::from_raw_parts(
                        (*self.codec_ctx).extradata,
                        (*self.codec_ctx).extradata_size as usize,
                    );

                    let mut delimiter_positions = Vec::new();

                    for i in 0..extra_data.len() - NALU_HEADER_LENGTH {
                        if extra_data[i + 0] == 0
                            && extra_data[i + 1] == 0
                            && extra_data[i + 2] == 0
                            && extra_data[i + 3] == 1
                        {
                            delimiter_positions.push(i);
                        }
                    }

                    delimiter_positions.push(extra_data.len());

                    let mut wd = delimiter_positions.windows(2);

                    while let Some(indexes) = wd.next() {
                        let (start, end) = (indexes[0], indexes[1]);
                        let nalu_type = extra_data[start + 4] & 0x1F;
                        let cell = match nalu_type {
                            7 => Some(&sps),
                            8 => Some(&pps),
                            _ => None,
                        };

                        if let Some(cell) = cell {
                            let payload = extra_data[start + 4..end].to_vec();
                            if let Err(_) = cell.set(payload) {
                                return Err(core_error!("set SPS or PPS cell repeatly"));
                            }
                        }
                    }
                }

                let packet = EndPointMessage::VideoFrame(EndPointVideoFrame {
                    width: (*self.codec_ctx).width,
                    height: (*self.codec_ctx).height,
                    buffer,
                });

                if let Err(err) = tx.try_send(packet) {
                    match err {
                        tokio::sync::mpsc::error::TrySendError::Full(_) => {
                            tracing::warn!("network send channel is full")
                        }
                        tokio::sync::mpsc::error::TrySendError::Closed(_) => {
                            return Err(core_error!("channel closed"));
                        }
                    }
                }

                av_packet_unref(self.packet);
            }
        }
    }
}

impl Drop for Encoder {
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
