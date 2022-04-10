use log::{error, trace, warn};
use std::{
    os::raw::{c_char, c_int, c_void},
    slice::from_raw_parts,
    sync::{
        mpsc::{channel, sync_channel, Receiver, Sender, SyncSender, TryRecvError},
        Arc,
    },
};

use crate::factory::frame::Frame;

use super::{bindings, errors::*};

pub struct VideoDecoder {
    frame_rx: Receiver<Frame>,
    nalu_tx: Sender<Vec<u8>>,
    exit_signal_tx: SyncSender<()>,
    exit_finish_rx: Receiver<()>,
    inner_video_decoder: Arc<c_void>,
}

impl VideoDecoder {
    pub fn new(decoder_name_ptr: *const c_char) -> Result<VideoDecoder, VideoDecoderError> {
        let (exit_signal_tx, exit_signal_rx) = sync_channel::<()>(1);
        let (exit_finish_tx, exit_finish_rx) = sync_channel::<()>(1);
        let (frame_tx, frame_rx) = channel();
        let (nalu_tx, nalu_rx) = channel();

        unsafe {
            let inner_video_decoder_ptr = bindings::new_video_decoder(
                decoder_name_ptr,
                bindings::AVHWDeviceType::AV_HWDEVICE_TYPE_NONE,
                VideoDecoder::callback,
            );

            if inner_video_decoder_ptr.is_null() {
                return Err(VideoDecoderError::CreateDecoderFailed);
            }

            let inner_video_decoder = Arc::from_raw(inner_video_decoder_ptr);

            VideoDecoder::decode_loop(
                exit_signal_rx,
                exit_finish_tx,
                inner_video_decoder.clone(),
                frame_tx,
                nalu_rx,
            );

            Ok(VideoDecoder {
                frame_rx,
                nalu_tx,
                exit_signal_tx,
                exit_finish_rx,
                inner_video_decoder,
            })
        }
    }

    pub fn send_nalu(&self, nalu: Vec<u8>) {
        if let Err(e) = self.nalu_tx.send(nalu) {
            error!("{:?}", e);
        }
    }

    pub fn recv_frame(&self) -> Option<Frame> {
        match self.frame_rx.recv() {
            Ok(frame) => Some(frame),
            Err(e) => {
                error!("{:?}", e);
                None
            }
        }
    }

    fn decode_loop(
        exit_signal_rx: Receiver<()>,
        exit_finish_tx: SyncSender<()>,
        inner_video_decoder: Arc<c_void>,
        mut frame_tx: Sender<Frame>,
        nalu_rx: Receiver<Vec<u8>>,
    ) {
        std::thread::spawn(move || unsafe {
            loop {
                match exit_signal_rx.try_recv() {
                    Ok(_) => break,
                    Err(e) => {
                        if let TryRecvError::Disconnected = e {
                            return;
                        }
                    }
                }

                match nalu_rx.recv() {
                    Ok(nalu) => {
                        let result = bindings::video_decode(
                            Arc::into_raw(inner_video_decoder.clone()),
                            &mut frame_tx as *mut Sender<Frame>,
                            nalu.as_ptr(),
                            nalu.len() as c_int,
                        );

                        if result != 0 {
                            error!("Error decoding frame: {}", result);
                            break;
                        }
                    }
                    Err(err) => {
                        error!("Error receiving nalu: {:?}", err);
                        break;
                    }
                };
            }

            if let Err(e) = exit_finish_tx.try_send(()) {
                warn!("{:?}", e);
            }
        });
    }

    unsafe extern "C" fn callback(
        tx: *mut Sender<Frame>,
        width: c_int,
        height: c_int,
        pix_fmt: bindings::AVPixelFormat,
        plane_linesize: *const c_int,
        plane_buffer_address: *const *const u8,
    ) {
        trace!(
            "video_decoder: callback triggered, width: {}, height: {}, pix_fmt: {:?}, linesize: {:?}, buffer: {:?}",
            width,
            height,
            pix_fmt,
            plane_linesize,
            plane_buffer_address
        );

        let tx = &mut *tx;

        let y_line_size = *plane_linesize.offset(0) as c_int;
        let y_buffer_address = *plane_buffer_address.offset(0);
        let y_buffer = from_raw_parts(y_buffer_address, (y_line_size * height) as usize).to_vec();

        let uv_line_size = *plane_linesize.offset(1) as c_int;
        let uv_buffer_address = *plane_buffer_address.offset(1);
        let uv_buffer =
            from_raw_parts(uv_buffer_address, (uv_line_size * height / 2) as usize).to_vec();

        let frame = Frame::new(
            width,
            height,
            y_line_size,
            y_buffer,
            uv_line_size,
            uv_buffer,
        );

        if let Err(e) = tx.send(frame) {
            error!("decode_callback: send error: {}", e);
        }
    }
}

impl Drop for VideoDecoder {
    fn drop(&mut self) {
        unsafe {
            if self.exit_signal_tx.send(()).is_ok() {
                let _ = self.exit_finish_rx.recv();
            }
            bindings::free_video_decoder(Arc::into_raw(self.inner_video_decoder.clone()));
        }
    }
}
