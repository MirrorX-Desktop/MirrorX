use crate::{
    component::{desktop::monitor::NSScreen, frame::DesktopEncodeFrame},
    core_error,
    error::{CoreError, CoreResult},
    ffi::os::macos::{core_graphics::*, core_media::*, core_video::*, io_surface::*},
};
use block::ConcreteBlock;
use dispatch::ffi::{dispatch_queue_create, dispatch_release, DISPATCH_QUEUE_SERIAL};
use scopeguard::defer;
use std::{cell::Cell, ffi::CString, ops::Deref, rc::Rc};

pub struct Duplicator {
    display_stream: CGDisplayStreamRef,
}

unsafe impl Send for Duplicator {}

impl Duplicator {
    pub fn new(
        monitor_id: Option<String>,
        capture_frame_tx: crossbeam::channel::Sender<DesktopEncodeFrame>,
    ) -> CoreResult<(Self, String)> {
        unsafe {
            let screens = NSScreen::screens()?;
            if screens.is_empty() {
                return Err(core_error!("no screen exist"));
            }

            let screen = match monitor_id {
                Some(monitor_id) => {
                    let monitor_id = monitor_id.parse::<u32>()?;
                    match screens.iter().find(|s| s.screenNumber() == monitor_id) {
                        Some(screen) => screen,
                        None => &screens[0],
                    }
                }
                None => &screens[0],
            };

            let queue_label = CString::new("queue.duplicator.mirrorx")?;

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
                screen.screenNumber(),
                screen_size.width as usize,
                screen_size.height as usize,
                kCVPixelFormatType_420YpCbCr8BiPlanarFullRange as i32,
                std::ptr::null_mut(),
                dispatch_queue,
                block.deref(),
            );

            Ok((
                Duplicator { display_stream },
                screen.screenNumber().to_string(),
            ))
        }
    }

    pub fn start(&self) -> CoreResult<()> {
        unsafe {
            let error_code = CGDisplayStreamStart(self.display_stream);
            if error_code == 0 {
                Ok(())
            } else {
                Err(core_error!(
                    "CGDisplayStreamStart returns error code: {}",
                    error_code
                ))
            }
        }
    }

    pub fn stop(&self) -> CoreResult<()> {
        unsafe {
            let error_code = CGDisplayStreamStop(self.display_stream);
            if error_code == 0 {
                Ok(())
            } else {
                Err(core_error!(
                    "CGDisplayStreamStop returns error code: {}",
                    error_code
                ))
            }
        }
    }
}

unsafe fn frame_available_handler(
    start_time: Rc<Cell<Option<std::time::Instant>>>,
    capture_frame_tx: *mut crossbeam::channel::Sender<DesktopEncodeFrame>,
    status: CGDisplayStreamFrameStatus,
    display_time: u64,
    frame_surface: IOSurfaceRef,
    update_ref: CGDisplayStreamUpdateRef,
) {
    if status == kCGDisplayStreamFrameStatusStopped {
        let _ = Box::from_raw(capture_frame_tx);
        return;
    }

    if capture_frame_tx.is_null() {
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

    defer! {
        if !pixel_buffer.is_null(){
            CVPixelBufferRelease(pixel_buffer);
        }
    }

    CVPixelBufferLockBaseAddress(pixel_buffer, 1);

    let width = CVPixelBufferGetWidth(pixel_buffer);
    let height = CVPixelBufferGetHeight(pixel_buffer);
    let luminance_bytes_address = CVPixelBufferGetBaseAddressOfPlane(pixel_buffer, 0);
    let luminance_stride = CVPixelBufferGetBytesPerRowOfPlane(pixel_buffer, 0);
    let luminance_bytes = std::slice::from_raw_parts(
        luminance_bytes_address as *mut u8,
        height * luminance_stride,
    )
    .to_vec();
    let chrominance_bytes_address = CVPixelBufferGetBaseAddressOfPlane(pixel_buffer, 1);
    let chrominance_stride = CVPixelBufferGetBytesPerRowOfPlane(pixel_buffer, 1);
    let chrominance_bytes = std::slice::from_raw_parts(
        chrominance_bytes_address as *mut u8,
        height * chrominance_stride / 2,
    )
    .to_vec();

    CVPixelBufferUnlockBaseAddress(pixel_buffer, 1);

    let pts = CMTimeMakeWithSeconds(elapsed_time, 1000);

    let capture_frame = DesktopEncodeFrame {
        width: width as i32,
        height: height as i32,
        luminance_bytes,
        luminance_stride: luminance_stride as i32,
        chrominance_bytes,
        chrominance_stride: chrominance_stride as i32,
    };

    if let Err(err) = (*capture_frame_tx).send(capture_frame) {
        tracing::error!(?err, "capture frame tx send failed");
    }

    let dropped_frames = CGDisplayStreamUpdateGetDropCount(update_ref);
    if dropped_frames > 0 {
        tracing::warn!(count = dropped_frames, "drop frames");
    }
}
