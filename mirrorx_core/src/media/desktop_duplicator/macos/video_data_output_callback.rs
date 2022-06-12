use crate::media::{
    bindings::macos::{
        CMSampleBufferGetImageBuffer, CMSampleBufferGetSampleTimingInfo, CMSampleBufferIsValid,
        CMSampleBufferRef, CMSampleTimingInfo, CVPixelBufferRetain,
    },
    frame::CaptureFrame,
};
use crossbeam_channel::Sender;
use objc::{
    class,
    declare::ClassDecl,
    msg_send,
    runtime::{Class, Object, Protocol, Sel},
    sel, sel_impl,
};
use std::{os::raw::c_void, sync::Once};
use tracing::error;

static VIDEO_DATA_OUTPUT_CALLBACK_CLASS_INIT_ONCE: Once = Once::new();

pub struct VideoDataOutputCallback {}

impl VideoDataOutputCallback {
    pub fn set_tx(&mut self, tx: Sender<CaptureFrame>) {
        unsafe {
            let obj = &mut *(self as *mut _ as *mut Object);
            let tx_ptr = Box::into_raw(Box::new(tx));
            msg_send![obj, setTx: tx_ptr as *mut c_void]
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

            cls.add_ivar::<*mut c_void>("_tx");

            // methods

            extern "C" fn set_tx(this: &mut Object, _cmd: Sel, tx_ptr: *mut c_void) {
                unsafe { this.set_ivar("_tx", tx_ptr) }
            }

            let set_tx_fn: extern "C" fn(&mut Object, Sel, *mut c_void) = set_tx;
            cls.add_method(sel!(setTx:), set_tx_fn);

            extern "C" fn get_tx(this: &Object, _cmd: Sel) -> *mut c_void {
                unsafe { *this.get_ivar("_tx") }
            }

            let get_tx_fn: extern "C" fn(&Object, Sel) -> *mut c_void = get_tx;
            cls.add_method(sel!(tx), get_tx_fn);

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

            cls.add_protocol(
                Protocol::get("AVCaptureVideoDataOutputSampleBufferDelegate").unwrap(),
            );

            // protocol implementations

            extern "C" fn capture_out_callback(
                this: &mut Object,
                _: Sel,
                _: *mut Object,
                #[allow(non_snake_case)] didOutputSampleBuffer: CMSampleBufferRef,
                _: *mut Object,
            ) {
                unsafe {
                    if !CMSampleBufferIsValid(didOutputSampleBuffer) {
                        tracing::error!("CMSampleBufferRef is invalid");
                        return;
                    }

                    let mut timing_info: CMSampleTimingInfo = std::mem::zeroed();
                    let ret = CMSampleBufferGetSampleTimingInfo(
                        didOutputSampleBuffer,
                        0,
                        &mut timing_info,
                    );
                    if ret != 0 {
                        tracing::error!(
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

                    let tx: *mut c_void = msg_send![this, tx];
                    if tx.is_null() {
                        return;
                    }

                    let tx = std::mem::transmute::<*mut c_void, &mut Sender<CaptureFrame>>(tx);

                    CVPixelBufferRetain(image_buffer);

                    let capture_frame = CaptureFrame {
                        cv_pixel_buffer: image_buffer,
                    };

                    if let Err(err) = tx.send(capture_frame) {
                        error!("send capture frame failed");
                    }
                }
            }

            extern "C" fn capture_drop_callback(
                _: &mut Object,
                _: Sel,
                _: *mut Object,
                _: *mut Object,
                _: *mut Object,
            ) {
            }

            cls.register();
        });

        Class::get("VideoOutputCallback").unwrap()
    }
}

impl Drop for VideoDataOutputCallback {
    fn drop(&mut self) {
        unsafe {
            let obj = &mut *(self as *mut _ as *mut Object);

            let tx_ptr: *mut c_void = msg_send![obj, tx];
            if !tx_ptr.is_null() {
                Box::from_raw(tx_ptr as *mut Sender<CaptureFrame>);
            }

            msg_send![obj, release]
        }
    }
}
