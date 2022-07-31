use crate::{
    component::monitor::NSScreen,
    error::MirrorXError,
    ffi::os::macos::{core_graphics::*, core_video::*, io_surface::*},
};
use block::ConcreteBlock;
use core_foundation::base::{CFRelease, CFRetain, ToVoid};
use dispatch::ffi::{dispatch_queue_create, dispatch_release, DISPATCH_QUEUE_SERIAL};
use scopeguard::defer;
use std::{ffi::CString, ops::Deref};

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

impl Duplicator {
    pub fn new(
        display: core_graphics::display::CGDirectDisplayID,
        capture_frame_tx: crossbeam::channel::Sender<IOSurfaceRefWrapper>,
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

            let capture_frame_tx_ptr = Box::into_raw(Box::new(capture_frame_tx));

            let block = ConcreteBlock::new(
                move |status: CGDisplayStreamFrameStatus,
                      display_time: u64,
                      frame_surface: IOSurfaceRef,
                      update_ref: CGDisplayStreamUpdateRef| {
                    frame_available_handler(
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
    capture_frame_tx: *mut crossbeam::channel::Sender<IOSurfaceRefWrapper>,
    status: CGDisplayStreamFrameStatus,
    display_time: u64,
    frame_surface: IOSurfaceRef,
    update_ref: CGDisplayStreamUpdateRef,
) {
    if status == kCGDisplayStreamFrameStatusStopped {
        let _ = Box::from_raw(capture_frame_tx);
        return;
    }

    CFRetain(frame_surface);
    IOSurfaceIncrementUseCount(frame_surface);

    if let Err(err) = (*capture_frame_tx).try_send(IOSurfaceRefWrapper {
        surface: frame_surface,
    }) {
        let surface_wrapper = err.into_inner();
        IOSurfaceDecrementUseCount(surface_wrapper.surface);
        CFRelease(surface_wrapper.surface);
    }

    let dropped_frames = CGDisplayStreamUpdateGetDropCount(update_ref);
    if dropped_frames > 0 {
        tracing::warn!(count = dropped_frames, "drop frames");
    }
}
