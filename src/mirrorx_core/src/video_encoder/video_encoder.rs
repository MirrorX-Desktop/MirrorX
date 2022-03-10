use crate::frame::frame::Frame;
use crate::video_encoder::bindings;
use crate::video_encoder::errors::VideoEncoderError;
use log::{error, trace, warn};
use std::{
    os::raw::{c_char, c_int, c_void},
    ptr,
    sync::{
        mpsc::{channel, sync_channel, Receiver, Sender, SyncSender, TryRecvError},
        Arc,
    },
};

pub struct VideoEncoder {
    nalu_rx: Receiver<Vec<u8>>,
    frame_tx: Sender<Box<Frame>>,
    exit_signal_tx: SyncSender<()>,
    exit_finish_rx: Receiver<()>,
    inner_video_encoder: Arc<c_void>,
}

impl VideoEncoder {
    pub fn new(encoder_name_ptr: *const c_char) -> Result<Self, VideoEncoderError> {
        let (exit_signal_tx, exit_signal_rx) = sync_channel::<()>(1);
        let (exit_finish_tx, exit_finish_rx) = sync_channel::<()>(1);
        let (frame_tx, frame_rx) = channel();
        let (nalu_tx, nalu_rx) = channel();

        unsafe {
            let inner_video_encoder_ptr = bindings::new_video_encoder(
                encoder_name_ptr,
                60,
                1920,
                1080,
                1920,
                1080,
                VideoEncoder::callback,
            );

            if inner_video_encoder_ptr.is_null() {
                return Err(VideoEncoderError::CreateEncoderFailed);
            }

            let inner_video_encoder = Arc::from_raw(inner_video_encoder_ptr);

            VideoEncoder::encode_loop(
                exit_signal_rx,
                exit_finish_tx,
                inner_video_encoder.clone(),
                nalu_tx,
                frame_rx,
            );

            Ok(VideoEncoder {
                nalu_rx,
                frame_tx,
                exit_signal_tx,
                exit_finish_rx,
                inner_video_encoder,
            })
        }
    }

    pub fn send_frame(&self, frame: Box<Frame>) {
        if let Err(e) = self.frame_tx.send(frame) {
            error!("{:?}", e);
        }
    }

    pub fn recv_nalu(&self) -> Option<Vec<u8>> {
        match self.nalu_rx.recv() {
            Ok(nalu) => Some(nalu),
            Err(e) => {
                error!("{:?}", e);
                None
            }
        }
    }

    fn encode_loop(
        exit_signal_rx: Receiver<()>,
        exit_finish_tx: SyncSender<()>,
        inner_video_encoder: Arc<c_void>,
        mut nalu_tx: Sender<Vec<u8>>,
        frame_rx: Receiver<Box<Frame>>,
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

                match frame_rx.recv() {
                    Ok(frame) => {
                        let result = bindings::video_encode(
                            Arc::into_raw(inner_video_encoder.clone()),
                            &mut nalu_tx as *mut Sender<Vec<u8>>,
                            frame.width,
                            frame.height,
                            frame.y_line_size,
                            frame.y_buffer.as_ptr(),
                            frame.uv_line_size,
                            frame.uv_buffer.as_ptr(),
                        );

                        if result != 0 {
                            error!("Error encoding frame: {}", result);
                            break;
                        }
                    }
                    Err(err) => {
                        error!("Error receiving frame: {:?}", err);
                        break;
                    }
                };
            }

            if let Err(e) = exit_finish_tx.try_send(()) {
                warn!("{:?}", e);
            }
        });
    }

    unsafe extern "C" fn callback(tx: *mut Sender<Vec<u8>>, nalu_ptr: *const u8, nalu_size: c_int) {
        trace!(
            "video_encoder: callback triggered, nalu_size: {}",
            nalu_size
        );

        if tx.is_null() {
            error!("video_encoder: callback tx ptr is nil");
            return;
        }

        let tx = &mut *tx;

        let mut nalu = Vec::with_capacity(nalu_size as usize);
        ptr::copy_nonoverlapping(nalu_ptr, nalu.as_mut_ptr(), nalu_size as usize);
        nalu.set_len(nalu_size as usize);

        if let Err(err) = tx.send(nalu) {
            error!("video_encoder: callback send nalu: {:?}", err);
        }
    }
}

impl Drop for VideoEncoder {
    fn drop(&mut self) {
        unsafe {
            if self.exit_signal_tx.send(()).is_ok() {
                let _ = self.exit_finish_rx.recv();
            }
            bindings::free_video_encoder(Arc::into_raw(self.inner_video_encoder.clone()));
        }
    }
}
