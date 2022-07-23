use crate::{
    component::desktop::Frame,
    ffi::os::{
        CMSampleBufferGetImageBuffer, CMSampleBufferGetSampleTimingInfo, CMSampleBufferIsValid,
        CMSampleBufferRef, CMSampleTimingInfo, CVPixelBufferGetBaseAddressOfPlane,
        CVPixelBufferGetBytesPerRowOfPlane, CVPixelBufferGetHeight, CVPixelBufferGetWidth,
        CVPixelBufferLockBaseAddress, CVPixelBufferUnlockBaseAddress,
    },
};
use objc::{
    class,
    declare::ClassDecl,
    msg_send,
    runtime::{Class, Object, Protocol, Sel},
    sel, sel_impl,
};
use scopeguard::defer;
use std::{os::raw::c_void, sync::Once};
use tracing::error;

static VIDEO_DATA_OUTPUT_CALLBACK_CLASS_INIT_ONCE: Once = Once::new();

pub struct AVCaptureVideoDataOutputCallback {}

impl AVCaptureVideoDataOutputCallback {
    pub fn initialize(&mut self, capture_frame_tx: tokio::sync::mpsc::Sender<Frame>) {
        unsafe {
            let obj = &mut *(self as *mut _ as *mut Object);
            let capture_frame_tx_ptr = Box::into_raw(Box::new(capture_frame_tx));
            let _: () = msg_send![obj, setCaptureFrameTx: capture_frame_tx_ptr as *mut c_void];
        }
    }
}

unsafe impl objc::Message for AVCaptureVideoDataOutputCallback {}

impl objc_foundation::INSObject for AVCaptureVideoDataOutputCallback {
    fn class() -> &'static Class {
        VIDEO_DATA_OUTPUT_CALLBACK_CLASS_INIT_ONCE.call_once(|| unsafe {
            let super_class = class!(NSObject);
            let mut cls = ClassDecl::new("VideoOutputCallback", super_class).unwrap();

            // ivars
            add_ivar_capture_frame_tx(&mut cls);

            // methods
            add_method_capture_output_callback(&mut cls);
            add_method_capture_drop_callback(&mut cls);

            // protocol

            cls.add_protocol(
                Protocol::get("AVCaptureVideoDataOutputSampleBufferDelegate").unwrap(),
            );

            cls.register();
        });

        Class::get("VideoOutputCallback").unwrap()
    }
}

impl Drop for AVCaptureVideoDataOutputCallback {
    fn drop(&mut self) {
        unsafe {
            let obj = &mut *(self as *mut _ as *mut Object);

            let capture_frame_tx_ptr: *mut c_void = msg_send![obj, captureFrameTx];
            if !capture_frame_tx_ptr.is_null() {
                Box::from_raw(capture_frame_tx_ptr as *mut tokio::sync::mpsc::Sender<Frame>);
            }

            msg_send![obj, release]
        }
    }
}

unsafe fn add_ivar_capture_frame_tx(cls: &mut ClassDecl) {
    // ivar

    cls.add_ivar::<*mut c_void>("_capture_frame_tx");

    // capture_frame_tx setter

    extern "C" fn set_capture_frame_tx(this: &mut Object, _cmd: Sel, tx_ptr: *mut c_void) {
        unsafe { this.set_ivar("_capture_frame_tx", tx_ptr) }
    }

    let set_capture_frame_tx_fn: extern "C" fn(&mut Object, Sel, *mut c_void) =
        set_capture_frame_tx;
    cls.add_method(sel!(setCaptureFrameTx:), set_capture_frame_tx_fn);

    // capture_frame_tx getter

    extern "C" fn get_capture_frame_tx(this: &Object, _cmd: Sel) -> *mut c_void {
        unsafe { *this.get_ivar("_capture_frame_tx") }
    }

    let get_capture_frame_tx_fn: extern "C" fn(&Object, Sel) -> *mut c_void = get_capture_frame_tx;

    cls.add_method(sel!(captureFrameTx), get_capture_frame_tx_fn);
}

unsafe fn add_method_capture_output_callback(cls: &mut ClassDecl) {
    let capture_out_callback_fn: extern "C" fn(
        &mut Object,
        Sel,
        *mut Object,
        CMSampleBufferRef,
        *mut Object,
    ) = capture_out_callback;

    cls.add_method(
        sel!(captureOutput:didOutputSampleBuffer:fromConnection:),
        capture_out_callback_fn,
    );

    extern "C" fn capture_out_callback(
        this: &mut Object,
        _: Sel,
        _: *mut Object,
        #[allow(non_snake_case)] didOutputSampleBuffer: CMSampleBufferRef,
        _: *mut Object,
    ) {
        unsafe {
            if !CMSampleBufferIsValid(didOutputSampleBuffer) {
                error!("CMSampleBufferRef is invalid");
                return;
            }

            // capture_frame_tx

            let capture_frame_tx_ptr: *mut c_void = msg_send![this, captureFrameTx];
            if capture_frame_tx_ptr.is_null() {
                error!("capture_frame_tx_ptr is null");
                return;
            }

            let capture_frame_tx = std::mem::transmute::<
                *mut c_void,
                &mut tokio::sync::mpsc::Sender<Frame>,
            >(capture_frame_tx_ptr);

            let mut timing_info: CMSampleTimingInfo = std::mem::zeroed();
            let ret = CMSampleBufferGetSampleTimingInfo(didOutputSampleBuffer, 0, &mut timing_info);
            if ret != 0 {
                error!(
                    ret = ret,
                    "get sample timing info from CMSampleBufferRef failed"
                );
                return;
            }

            let image_buffer = CMSampleBufferGetImageBuffer(didOutputSampleBuffer);
            if image_buffer.is_null() {
                tracing::error!("get image buffer from CMSampleBufferRef failed");
                return;
            }

            let lock_result = CVPixelBufferLockBaseAddress(image_buffer, 1);
            if lock_result != 0 {
                tracing::error!("CVPixelBufferLockBaseAddress failed");
                return;
            }

            defer! {
                CVPixelBufferUnlockBaseAddress(image_buffer, 1);
            }

            let width = CVPixelBufferGetWidth(image_buffer);
            if width == 0 {
                error!("width is 0");
                return;
            }

            let height = CVPixelBufferGetHeight(image_buffer);
            if height == 0 {
                error!("height is 0");
                return;
            }

            let luminance_buffer_ptr = CVPixelBufferGetBaseAddressOfPlane(image_buffer, 0);
            if luminance_buffer_ptr.is_null() {
                error!("get base address of plane 0 from CVPixelBufferRef failed");
                return;
            }

            let chrominance_buffer = CVPixelBufferGetBaseAddressOfPlane(image_buffer, 1);
            if chrominance_buffer.is_null() {
                error!("get base address of plane 1 from CVPixelBufferRef failed");
                return;
            }

            let luminance_stride = CVPixelBufferGetBytesPerRowOfPlane(image_buffer, 0);
            let luminance_buffer = std::slice::from_raw_parts(
                luminance_buffer_ptr as *const u8,
                luminance_stride * height,
            )
            .to_vec();

            let chrominance_stride = CVPixelBufferGetBytesPerRowOfPlane(image_buffer, 1);
            let chrominance_buffer = std::slice::from_raw_parts(
                chrominance_buffer as *const u8,
                chrominance_stride * height / 2,
            )
            .to_vec();

            let capture_frame = Frame::new(
                width as u16,
                height as u16,
                luminance_buffer,
                luminance_stride as u16,
                chrominance_buffer,
                chrominance_stride as u16,
                0,
            );

            if let Err(err) = capture_frame_tx.blocking_send(capture_frame) {
                error!(?err, "send capture frame failed");
                return;
            }
        }
    }
}

unsafe fn add_method_capture_drop_callback(cls: &mut ClassDecl) {
    let capture_drop_callback_fn: extern "C" fn(
        &mut Object,
        Sel,
        *mut Object,
        *mut Object,
        *mut Object,
    ) = capture_drop_callback;

    cls.add_method(
        sel!(captureOutput:didDropSampleBuffer:fromConnection:),
        capture_drop_callback_fn,
    );

    extern "C" fn capture_drop_callback(
        _: &mut Object,
        _: Sel,
        _: *mut Object,
        _: *mut Object,
        _: *mut Object,
    ) {
    }
}
