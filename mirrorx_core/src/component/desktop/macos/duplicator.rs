use crate::{
    component::{desktop::frame::CaptureFrame, monitor::NSScreen},
    error::MirrorXError,
    ffi::os::macos::{core_graphics::*, core_media::*, core_video::*, io_surface::*},
};
use block::ConcreteBlock;
use core_foundation::base::CFRelease;
use dispatch::ffi::{dispatch_queue_create, dispatch_release, DISPATCH_QUEUE_SERIAL};
use scopeguard::defer;
use std::{cell::Cell, ffi::CString, ops::Deref, rc::Rc};

// pub struct Duplicator {
//     capture_session: AVCaptureSession,
// }

// unsafe impl Send for Duplicator {}

// impl Duplicator {
//     pub fn new(
//         capture_frame_tx: crossbeam::channel::Sender<Frame>,
//         display_id: &str,
//         fps: u8,
//     ) -> anyhow::Result<Self> {
//         let display_id: u32 = match display_id.parse() {
//             Ok(v) => v,
//             Err(_) => return Err(anyhow::anyhow!("convert display id failed")),
//         };

//         let mut capture_session = AVCaptureSession::new();
//         capture_session.begin_configuration();
//         capture_session.set_session_preset(AVCaptureSessionPreset::AVCaptureSessionPresetHigh);

//         let capture_screen_input = AVCaptureScreenInput::new(display_id);
//         capture_screen_input.set_captures_cursor(true);
//         capture_screen_input.set_captures_mouse_clicks(false);
//         capture_screen_input.set_min_frame_duration(unsafe { CMTimeMake(1, fps as i32) });

//         if capture_session.can_add_input(&capture_screen_input) {
//             capture_session.add_input(capture_screen_input);
//         } else {
//             bail!("can't add input");
//         }

//         let capture_video_data_output = AVCaptureVideoDataOutput::new(capture_frame_tx);

//         if capture_session.can_add_output(&capture_video_data_output) {
//             capture_session.add_output(capture_video_data_output);
//         } else {
//             bail!("can't add output");
//         }

//         capture_session.commit_configuration();

//         Ok(Duplicator { capture_session })
//     }

//     pub fn start(&mut self) -> anyhow::Result<()> {
//         self.capture_session.start_running();
//         Ok(())
//     }

//     pub fn stop(&mut self) {
//         self.capture_session.stop_running();
//     }
// }

// impl Drop for Duplicator {
//     fn drop(&mut self) {
//         self.capture_session.stop_running();
//         info!("DesktopDuplicator dropped");
//     }
// }

pub struct Duplicator {
    display_stream: CGDisplayStreamRef,
}

unsafe impl Send for Duplicator {}

impl Duplicator {
    pub fn new(
        display: core_graphics::display::CGDirectDisplayID,
        capture_frame_tx: crossbeam::channel::Sender<CaptureFrame>,
    ) -> Result<Self, MirrorXError> {
        unsafe {
            let screens = NSScreen::screens()?;
            let screen = match screens.iter().find(|s| s.screenNumber() == display) {
                Some(screen) => screen,
                None => &screens[0],
            };

            let queue_label = CString::new("queue.duplicator.mirrorx")
                .map_err(|err| MirrorXError::Other(anyhow::anyhow!(err)))?;

            let dispatch_queue = dispatch_queue_create(queue_label.as_ptr(), DISPATCH_QUEUE_SERIAL);

            defer! {
                dispatch_release(dispatch_queue);
            }

            let screen_size = screen.frame().size;

            let start_time: Rc<Cell<Option<std::time::Instant>>> = Rc::new(Cell::new(None));

            let capture_frame_tx_ptr = Box::into_raw(Box::new(capture_frame_tx));

            let block = ConcreteBlock::new(
                move |status: CGDisplayStreamFrameStatus,
                      display_time: u64,
                      frame_surface: IOSurfaceRef,
                      update_ref: CGDisplayStreamUpdateRef| {
                    frame_available_handler(
                        start_time.clone(),
                        capture_frame_tx_ptr,
                        status,
                        display_time,
                        frame_surface,
                        update_ref,
                    )
                },
            );

            let block = block.copy();

            let display_stream = CGDisplayStreamCreateWithDispatchQueue(
                display,
                screen_size.width as usize,
                screen_size.height as usize,
                kCVPixelFormatType_420YpCbCr8BiPlanarFullRange as i32,
                std::ptr::null_mut(),
                dispatch_queue,
                block.deref(),
            );

            Ok(Duplicator { display_stream })
        }
    }

    pub fn start(&self) -> Result<(), MirrorXError> {
        unsafe {
            let error_code = CGDisplayStreamStart(self.display_stream);
            if error_code == 0 {
                Ok(())
            } else {
                Err(MirrorXError::Other(anyhow::anyhow!(
                    "CGDisplayStreamStart returns error({})",
                    error_code
                )))
            }
        }
    }

    pub fn stop(&self) -> Result<(), MirrorXError> {
        unsafe {
            let error_code = CGDisplayStreamStop(self.display_stream);
            if error_code == 0 {
                Ok(())
            } else {
                Err(MirrorXError::Other(anyhow::anyhow!(
                    "CGDisplayStreamStop returns error({})",
                    error_code
                )))
            }
        }
    }
}

unsafe fn frame_available_handler(
    start_time: Rc<Cell<Option<std::time::Instant>>>,
    capture_frame_tx: *mut crossbeam::channel::Sender<CaptureFrame>,
    status: CGDisplayStreamFrameStatus,
    display_time: u64,
    frame_surface: IOSurfaceRef,
    update_ref: CGDisplayStreamUpdateRef,
) {
    if status == kCGDisplayStreamFrameStatusStopped {
        let _ = Box::from_raw(capture_frame_tx);
        return;
    }

    let elapsed_time = match start_time.get() {
        Some(epoch) => epoch.elapsed().as_secs_f64(),
        None => {
            start_time.replace(Some(std::time::Instant::now()));
            0f64
        }
    };

    let mut pixel_buffer = std::ptr::null_mut();
    let ret = CVPixelBufferCreateWithIOSurface(
        std::ptr::null(),
        frame_surface,
        std::ptr::null(),
        &mut pixel_buffer,
    );

    if ret != 0 {
        tracing::error!(?ret, "CVPixelBufferCreateWithIOSurface failed");
        return;
    }

    let pts = CMTimeMakeWithSeconds(elapsed_time, 1000);

    if let Err(err) = (*capture_frame_tx).send(CaptureFrame { pts, pixel_buffer }) {
        let capture_frame = err.into_inner();
        CFRelease(capture_frame.pixel_buffer);
    }

    let dropped_frames = CGDisplayStreamUpdateGetDropCount(update_ref);
    if dropped_frames > 0 {
        tracing::warn!(count = dropped_frames, "drop frames");
    }
}
