use crate::{
    component::{desktop::monitor::NSScreen, frame::DesktopEncodeFrame},
    core_error,
    error::CoreResult,
};
use block::ConcreteBlock;
use dispatch::ffi::{dispatch_queue_create, dispatch_release, DISPATCH_QUEUE_SERIAL};
use mirrorx_native::os::macos::{core_graphics::*, core_video::*, io_surface::*};
use once_cell::unsync::OnceCell;
use scopeguard::defer;
use std::{ffi::CString, ops::Deref, time::Duration};
use tokio::sync::mpsc::Sender;

pub struct Duplicator {
    display_stream: CGDisplayStreamRef,
}

unsafe impl Send for Duplicator {}
unsafe impl Sync for Duplicator {}

impl Duplicator {
    pub fn new(
        monitor_id: Option<String>,
        capture_frame_tx: Sender<DesktopEncodeFrame>,
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

            let capture_frame_tx_ptr = Box::into_raw(Box::new(capture_frame_tx));

            let epoch: OnceCell<std::time::Instant> = OnceCell::new();

            let block = ConcreteBlock::new(
                move |status: CGDisplayStreamFrameStatus,
                      display_time: u64,
                      frame_surface: IOSurfaceRef,
                      update_ref: CGDisplayStreamUpdateRef| {
                    let capture_time = if let Some(instant) = epoch.get() {
                        instant.elapsed()
                    } else {
                        let _ = epoch.set(std::time::Instant::now());
                        Duration::ZERO
                    };

                    frame_available_handler(
                        capture_time,
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
    capture_time: Duration,
    capture_frame_tx: *mut Sender<DesktopEncodeFrame>,
    status: CGDisplayStreamFrameStatus,
    _display_time: u64,
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

    CVPixelBufferLockBaseAddress(pixel_buffer, 0);

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

    CVPixelBufferUnlockBaseAddress(pixel_buffer, 0);

    let capture_frame = DesktopEncodeFrame {
        capture_time,
        width: width as i32,
        height: height as i32,
        luminance_bytes,
        luminance_stride: luminance_stride as i32,
        chrominance_bytes,
        chrominance_stride: chrominance_stride as i32,
    };

    if (*capture_frame_tx).blocking_send(capture_frame).is_err() {
        tracing::error!("desktop capture frame tx send failed");
    }

    let dropped_frames = CGDisplayStreamUpdateGetDropCount(update_ref);
    if dropped_frames > 0 {
        tracing::warn!(count = dropped_frames, "drop frames");
    }
}
