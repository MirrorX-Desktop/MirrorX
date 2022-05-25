use super::bindings;
use crate::media::{
    ffmpeg::{
        avcodec::{
            avcodec::{avcodec_receive_packet, avcodec_send_frame},
            packet::{av_new_packet, av_packet_alloc, av_packet_free, av_packet_unref},
        },
        avutil::{
            error::AVERROR_EOF,
            frame::{av_frame_alloc, av_frame_free, av_frame_get_buffer, av_frame_make_writable},
            imgutils::av_image_get_buffer_size,
            pixfmt::{
                AVCOL_PRI_BT709, AVCOL_RANGE_JPEG, AVCOL_SPC_BT709, AVCOL_TRC_BT709,
                AV_PIX_FMT_NV12,
            },
        },
    },
    video_packet::VideoPacket,
};
use anyhow::{bail, Ok};
use crossbeam_channel::{bounded, Receiver, Sender};
use log::{error, trace};
use std::{ffi::CString, os::raw::c_int, ptr::null_mut, slice::from_raw_parts};

pub struct VideoEncoder {
    video_encoder_ptr: *mut bindings::VideoEncoder,
    tx: Sender<VideoPacket>,
}

unsafe impl Send for VideoEncoder {}
unsafe impl Sync for VideoEncoder {}

impl VideoEncoder {
    pub fn new(
        encoder_name: &str,
        fps: i32,
        screen_width: u16,
        screen_height: u16,
    ) -> anyhow::Result<(VideoEncoder, Receiver<VideoPacket>)> {
        let encoder_name_ptr = CString::new(encoder_name.to_string())?;

        unsafe {
            let video_encoder_ptr = bindings::video_encoder_create(
                encoder_name_ptr.as_ptr(),
                screen_width as c_int,
                screen_height as c_int,
                fps,
            );

            if video_encoder_ptr.is_null() {
                bail!("create video encoder failed");
            }

            bindings::video_encoder_set_opt(
                video_encoder_ptr,
                CString::new("profile")?.as_ptr(),
                CString::new("high")?.as_ptr(),
            );

            bindings::video_encoder_set_opt(
                video_encoder_ptr,
                CString::new("level")?.as_ptr(),
                CString::new("5.2")?.as_ptr(),
            );

            match encoder_name {
                "libx264" => {
                    bindings::video_encoder_set_opt(
                        video_encoder_ptr,
                        CString::new("preset")?.as_ptr(),
                        CString::new("ultrafast")?.as_ptr(),
                    );
                    bindings::video_encoder_set_opt(
                        video_encoder_ptr,
                        CString::new("tune")?.as_ptr(),
                        CString::new("zerolatency")?.as_ptr(),
                    );

                    bindings::video_encoder_set_opt(
                        video_encoder_ptr,
                        CString::new("sc_threshold")?.as_ptr(),
                        CString::new("499")?.as_ptr(),
                    );
                }
                "h264_videotoolbox" => {
                    bindings::video_encoder_set_opt(
                        video_encoder_ptr,
                        CString::new("realtime")?.as_ptr(),
                        CString::new("1")?.as_ptr(),
                    );
                    bindings::video_encoder_set_opt(
                        video_encoder_ptr,
                        CString::new("allow_sw")?.as_ptr(),
                        CString::new("0")?.as_ptr(),
                    );
                }
                _ => {}
            };

            if !bindings::video_encoder_open(video_encoder_ptr) {
                bail!("open video encoder failed");
            }

            let (tx, rx) = bounded::<VideoPacket>(600);

            Ok((
                VideoEncoder {
                    video_encoder_ptr,
                    tx,
                },
                rx,
            ))
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
            if (*self.video_encoder_ptr).frame.is_null()
                || (*self.video_encoder_ptr).packet.is_null()
                || (*(*self.video_encoder_ptr).frame).width != width
                || (*(*self.video_encoder_ptr).frame).height != height
            {
                // release old frame
                if !(*self.video_encoder_ptr).frame.is_null() {
                    av_frame_free(&mut (*self.video_encoder_ptr).frame);
                    (*self.video_encoder_ptr).frame = null_mut();
                }

                // release old packet
                if !(*self.video_encoder_ptr).packet.is_null() {
                    av_packet_free(&mut (*self.video_encoder_ptr).packet);
                    (*self.video_encoder_ptr).packet = null_mut();
                }

                // re-allocate new frame
                (*self.video_encoder_ptr).frame = av_frame_alloc();
                if (*self.video_encoder_ptr).frame.is_null() {
                    error!("video_encoder: av_frame_alloc failed");
                    return;
                }

                (*(*self.video_encoder_ptr).frame).width = width;
                (*(*self.video_encoder_ptr).frame).height = height;
                (*(*self.video_encoder_ptr).frame).format = AV_PIX_FMT_NV12;
                (*(*self.video_encoder_ptr).frame).color_range = AVCOL_RANGE_JPEG;
                (*(*self.video_encoder_ptr).frame).color_primaries = AVCOL_PRI_BT709;
                (*(*self.video_encoder_ptr).frame).color_trc = AVCOL_TRC_BT709;
                (*(*self.video_encoder_ptr).frame).color_space = AVCOL_SPC_BT709;

                ret = av_frame_get_buffer((*self.video_encoder_ptr).frame, 32);
                if ret < 0 {
                    error!(
                        r#"video_encoder: av_frame_get_buffer failed {{"ret": "{}"}}"#,
                        ret
                    );
                    return;
                }

                // re-allocate new packet
                (*self.video_encoder_ptr).packet = av_packet_alloc();
                if (*self.video_encoder_ptr).packet.is_null() {
                    error!("video_encoder: av_packet_alloc failed");
                    return;
                }

                let packet_size = av_image_get_buffer_size(
                    (*(*self.video_encoder_ptr).frame).format,
                    (*(*self.video_encoder_ptr).frame).width,
                    (*(*self.video_encoder_ptr).frame).height,
                    32,
                );

                ret = av_new_packet((*self.video_encoder_ptr).packet, packet_size);
                if ret < 0 {
                    error!(
                        r#"video_encoder: av_new_packet failed {{"ret": "{}"}}"#,
                        ret
                    );
                    return;
                }
            }

            ret = av_frame_make_writable((*self.video_encoder_ptr).frame);
            if ret < 0 {
                error!(
                    r#"video_encoder: av_frame_make_writable failed {{"ret": "{}"}}"#,
                    ret
                );
            }

            (*(*self.video_encoder_ptr).frame).width = width;
            (*(*self.video_encoder_ptr).frame).height = height;

            (*(*self.video_encoder_ptr).frame).data[0] = lumina_plane_bytes_address;
            (*(*self.video_encoder_ptr).frame).linesize[0] = lumina_plane_stride;

            (*(*self.video_encoder_ptr).frame).data[1] = chrominance_plane_bytes_address;
            (*(*self.video_encoder_ptr).frame).linesize[1] = chrominance_plane_stride;

            (*(*self.video_encoder_ptr).frame).time_base.num = 1;
            (*(*self.video_encoder_ptr).frame).time_base.den = pts_scale;
            (*(*self.video_encoder_ptr).frame).pts = pts;

            ret = avcodec_send_frame(
                (*self.video_encoder_ptr).codec_ctx,
                (*self.video_encoder_ptr).frame,
            );

            if ret != 0 {
                if ret == -35 {
                    error!("video_encoder: can not send more frame, should receive more packet");
                } else if ret == AVERROR_EOF {
                    error!("video_encoder: encoder closed, shouldn't send new frame");
                } else {
                    error!(
                        r#"video_encoder: avcodec_send_frame failed {{"ret": "{}"}}"#,
                        ret
                    );
                }
                return;
            }

            loop {
                ret = avcodec_receive_packet(
                    (*self.video_encoder_ptr).codec_ctx,
                    (*self.video_encoder_ptr).packet,
                );

                // todo: AVERROR(EAGAIN) is -35 in unix but windwos?
                if ret == -libc::EAGAIN || ret == AVERROR_EOF {
                    return;
                } else if ret < 0 {
                    error!(
                        r#"video_encoder: avcodec_receive_packet failed {{"ret": "{}"}}"#,
                        ret
                    );
                    return;
                }

                if let Err(err) = self.tx.try_send(VideoPacket {
                    data: from_raw_parts(
                        (*(*self.video_encoder_ptr).packet).data,
                        (*(*self.video_encoder_ptr).packet).size as usize,
                    )
                    .to_vec(),
                    dts: (*(*self.video_encoder_ptr).packet).dts,
                    pts: (*(*self.video_encoder_ptr).packet).pts,
                }) {
                    error!(
                        r#"video_encoder: send video packet failed {{"err": "{}"}}"#,
                        err
                    );
                }

                av_packet_unref((*self.video_encoder_ptr).packet);
            }
        }
    }
}

impl Drop for VideoEncoder {
    fn drop(&mut self) {
        if !self.video_encoder_ptr.is_null() {
            unsafe {
                trace!("video_encoder: free_video_encoder");
                bindings::video_encoder_destroy(self.video_encoder_ptr);
                trace!("video_encoder: free_video_encoder done");
            }
        }
    }
}
