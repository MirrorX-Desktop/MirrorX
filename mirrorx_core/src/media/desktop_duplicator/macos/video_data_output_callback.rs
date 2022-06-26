use crate::media::{bindings::macos::*, frame::CaptureFrame};
use crossbeam::channel::{Receiver, Sender};
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

pub struct VideoDataOutputCallback {}

impl VideoDataOutputCallback {
    pub fn initialize(&mut self, capture_frame_tx: Sender<CaptureFrame>) {
        unsafe {
            let obj = &mut *(self as *mut _ as *mut Object);
            let capture_frame_tx_ptr = Box::into_raw(Box::new(capture_frame_tx));
            let _: () = msg_send![obj, setCaptureFrameTx: capture_frame_tx_ptr as *mut c_void];

            let (capture_frame_release_notify_tx, capture_frame_release_notify_rx) =
                crossbeam::channel::bounded::<()>(1);

            let capture_frame_release_notify_tx_ptr =
                Box::into_raw(Box::new(capture_frame_release_notify_tx));

            let capture_frame_release_notify_rx_ptr =
                Box::into_raw(Box::new(capture_frame_release_notify_rx));

            let _: () = msg_send![
                obj,
                setCaptureFrameReleaseNotifyTx: capture_frame_release_notify_tx_ptr as *mut c_void
            ];

            let _: () = msg_send![
                obj,
                setCaptureFrameReleaseNotifyRx: capture_frame_release_notify_rx_ptr as *mut c_void
            ];
        }
    }
}

unsafe impl objc::Message for VideoDataOutputCallback {}

impl objc_foundation::INSObject for VideoDataOutputCallback {
    fn class() -> &'static Class {
        VIDEO_DATA_OUTPUT_CALLBACK_CLASS_INIT_ONCE.call_once(|| unsafe {
            let super_class = class!(NSObject);
            let mut cls = ClassDecl::new("VideoOutputCallback", super_class).unwrap();

            // ivars
            add_ivar_capture_frame_tx(&mut cls);
            add_ivar_capture_frame_release_notify_tx(&mut cls);
            add_ivar_capture_frame_release_notify_rx(&mut cls);

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

impl Drop for VideoDataOutputCallback {
    fn drop(&mut self) {
        unsafe {
            let obj = &mut *(self as *mut _ as *mut Object);

            let capture_frame_tx_ptr: *mut c_void = msg_send![obj, captureFrameTx];
            if !capture_frame_tx_ptr.is_null() {
                Box::from_raw(capture_frame_tx_ptr as *mut Sender<CaptureFrame>);
            }

            let capture_frame_release_notify_tx_ptr: *mut c_void =
                msg_send![obj, captureFrameReleaseNotifyTx];
            if !capture_frame_release_notify_tx_ptr.is_null() {
                Box::from_raw(capture_frame_release_notify_tx_ptr as *mut Sender<()>);
            }

            let capture_frame_release_notify_rx_ptr: *mut c_void =
                msg_send![obj, captureFrameReleaseNotifyRx];
            if !capture_frame_release_notify_rx_ptr.is_null() {
                Box::from_raw(capture_frame_release_notify_rx_ptr as *mut Receiver<()>);
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

unsafe fn add_ivar_capture_frame_release_notify_tx(cls: &mut ClassDecl) {
    // ivar

    cls.add_ivar::<*mut c_void>("_capture_frame_release_notify_tx");

    // capture_frame_release_notify_tx setter

    extern "C" fn set_capture_frame_release_notify_tx(
        this: &mut Object,
        _cmd: Sel,
        tx_ptr: *mut c_void,
    ) {
        unsafe { this.set_ivar("_capture_frame_release_notify_tx", tx_ptr) }
    }

    let set_capture_frame_release_notify_tx_fn: extern "C" fn(&mut Object, Sel, *mut c_void) =
        set_capture_frame_release_notify_tx;

    cls.add_method(
        sel!(setCaptureFrameReleaseNotifyTx:),
        set_capture_frame_release_notify_tx_fn,
    );

    // capture_frame_release_notify_tx getter

    extern "C" fn get_capture_frame_release_notify_tx(this: &Object, _cmd: Sel) -> *mut c_void {
        unsafe { *this.get_ivar("_capture_frame_release_notify_tx") }
    }

    let get_capture_frame_release_notify_tx_fn: extern "C" fn(&Object, Sel) -> *mut c_void =
        get_capture_frame_release_notify_tx;

    cls.add_method(
        sel!(captureFrameReleaseNotifyTx),
        get_capture_frame_release_notify_tx_fn,
    );
}

unsafe fn add_ivar_capture_frame_release_notify_rx(cls: &mut ClassDecl) {
    // ivar

    cls.add_ivar::<*mut c_void>("_capture_frame_release_notify_rx");

    // capture_frame_release_notify_rx setter

    extern "C" fn set_capture_frame_release_notify_rx(
        this: &mut Object,
        _cmd: Sel,
        tx_ptr: *mut c_void,
    ) {
        unsafe { this.set_ivar("_capture_frame_release_notify_rx", tx_ptr) }
    }

    let set_capture_frame_release_notify_rx_fn: extern "C" fn(&mut Object, Sel, *mut c_void) =
        set_capture_frame_release_notify_rx;

    cls.add_method(
        sel!(setCaptureFrameReleaseNotifyRx:),
        set_capture_frame_release_notify_rx_fn,
    );

    // capture_frame_release_notify_rx getter

    extern "C" fn get_capture_frame_release_notify_rx(this: &Object, _cmd: Sel) -> *mut c_void {
        unsafe { *this.get_ivar("_capture_frame_release_notify_rx") }
    }

    let get_capture_frame_release_notify_rx_fn: extern "C" fn(&Object, Sel) -> *mut c_void =
        get_capture_frame_release_notify_rx;

    cls.add_method(
        sel!(captureFrameReleaseNotifyRx),
        get_capture_frame_release_notify_rx_fn,
    );
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

            let capture_frame_tx =
                std::mem::transmute::<*mut c_void, &mut Sender<CaptureFrame>>(capture_frame_tx_ptr);

            // capture_frame_release_notify_tx

            let capture_frame_release_notify_tx_ptr: *mut c_void =
                msg_send![this, captureFrameReleaseNotifyTx];

            if capture_frame_release_notify_tx_ptr.is_null() {
                error!("capture_frame_release_notify_tx_ptr is null");
                return;
            }

            let capture_frame_release_notify_tx = std::mem::transmute::<*mut c_void, &mut Sender<()>>(
                capture_frame_release_notify_tx_ptr,
            );

            // capture_frame_release_notify_rx

            let capture_frame_release_notify_rx_ptr: *mut c_void =
                msg_send![this, captureFrameReleaseNotifyRx];

            if capture_frame_release_notify_rx_ptr.is_null() {
                error!("capture_frame_release_notify_tx_ptr is null");
                return;
            }

            let capture_frame_release_notify_rx = std::mem::transmute::<
                *mut c_void,
                &mut Receiver<()>,
            >(capture_frame_release_notify_rx_ptr);

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
            );

            let chrominance_stride = CVPixelBufferGetBytesPerRowOfPlane(image_buffer, 1);
            let chrominance_buffer = std::slice::from_raw_parts(
                chrominance_buffer as *const u8,
                chrominance_stride * height / 2,
            );

            let capture_frame = CaptureFrame::new(
                width,
                height,
                luminance_buffer,
                luminance_stride,
                chrominance_buffer,
                chrominance_stride,
                capture_frame_release_notify_tx.clone(),
            );

            if let Err(err) = capture_frame_tx.send(capture_frame) {
                error!(?err, "send capture frame failed");
                return;
            }

            if let Err(_) = capture_frame_release_notify_rx.recv() {
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
