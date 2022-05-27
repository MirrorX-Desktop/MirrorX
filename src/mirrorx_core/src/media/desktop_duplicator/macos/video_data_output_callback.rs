use crate::media::{
    desktop_duplicator::macos::bindings::CMSampleBufferRef, video_encoder::VideoEncoder,
};
use block::{ConcreteBlock, RcBlock};
use objc::{
    class,
    declare::ClassDecl,
    msg_send,
    runtime::{Class, Object, Protocol, Sel},
    sel, sel_impl,
};
use std::{ffi::c_void, sync::Once};

static VIDEO_DATA_OUTPUT_CALLBACK_CLASS_INIT_ONCE: Once = Once::new();

pub struct VideoDataOutputCallback {}

impl VideoDataOutputCallback {
    pub fn set_callback(
        &mut self,
        callback: impl Fn(&mut VideoEncoder, CMSampleBufferRef) -> () + 'static,
    ) {
        unsafe {
            let obj = &mut *(self as *mut _ as *mut Object);

            let block = ConcreteBlock::new(callback).copy();
            let block_ptr = Box::into_raw(Box::new(block));
            msg_send![obj, setCallback: block_ptr as *const c_void]
        }
    }

    pub fn set_video_encoder(&mut self, video_encoder: VideoEncoder) {
        unsafe {
            let obj = &mut *(self as *mut _ as *mut Object);

            let video_encoder_ptr = Box::into_raw(Box::new(video_encoder));
            msg_send![obj, setVideoEncoder: video_encoder_ptr as *mut c_void]
        }
    }
}

unsafe impl objc::Message for VideoDataOutputCallback {}

impl objc_foundation::INSObject for VideoDataOutputCallback {
    fn class() -> &'static Class {
        VIDEO_DATA_OUTPUT_CALLBACK_CLASS_INIT_ONCE.call_once(|| unsafe {
            let super_class = class!(NSObject);
            let mut cls = ClassDecl::new("VideoOutputCallback", super_class).unwrap();

            cls.add_ivar::<*const c_void>("_capture_callback");
            cls.add_ivar::<*mut c_void>("_video_encoder");

            extern "C" fn set_callback(this: &mut Object, _cmd: Sel, callback: *const c_void) {
                unsafe {
                    this.set_ivar("_capture_callback", callback);
                }
            }

            extern "C" fn get_callback(this: &Object, _cmd: Sel) -> *const c_void {
                unsafe { *this.get_ivar("_capture_callback") }
            }

            extern "C" fn set_video_encoder(this: &mut Object, _cmd: Sel, callback: *mut c_void) {
                unsafe {
                    this.set_ivar("_video_encoder", callback);
                }
            }

            extern "C" fn get_video_encoder(this: &Object, _cmd: Sel) -> *mut c_void {
                unsafe { *this.get_ivar("_video_encoder") }
            }

            extern "C" fn capture_out_callback(
                this: &mut Object,
                _: Sel,
                _: *mut Object,
                #[allow(non_snake_case)] didOutputSampleBuffer: CMSampleBufferRef,
                _: *mut Object,
            ) {
                unsafe {
                    let callback_block: *mut RcBlock<(&VideoEncoder, CMSampleBufferRef), ()> =
                        msg_send![this, callback];

                    let video_encoder: *mut VideoEncoder = msg_send![this, videoEncoder];

                    (&*callback_block).call((
                        &mut *video_encoder as &mut VideoEncoder,
                        didOutputSampleBuffer,
                    ));
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

            let set_callback_fn: extern "C" fn(&mut Object, Sel, *const c_void) = set_callback;
            cls.add_method(sel!(setCallback:), set_callback_fn);

            let get_callback_fn: extern "C" fn(&Object, Sel) -> *const c_void = get_callback;
            cls.add_method(sel!(callback), get_callback_fn);

            let set_video_encoder_fn: extern "C" fn(&mut Object, Sel, *mut c_void) =
                set_video_encoder;
            cls.add_method(sel!(setVideoEncoder:), set_video_encoder_fn);

            let get_video_encoder_fn: extern "C" fn(&Object, Sel) -> *mut c_void =
                get_video_encoder;
            cls.add_method(sel!(videoEncoder), get_video_encoder_fn);

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

            cls.register();
        });

        Class::get("VideoOutputCallback").unwrap()
    }
}

impl Drop for VideoDataOutputCallback {
    fn drop(&mut self) {
        unsafe {
            let obj = &mut *(self as *mut _ as *mut Object);

            let block_ptr: *const c_void = msg_send![obj, callback];
            let block =
                Box::from_raw(block_ptr as *mut RcBlock<(&mut VideoEncoder, *mut c_void), ()>);
            drop(block);

            let video_encoder_ptr: *mut c_void = msg_send![obj, videoEncoder];
            let video_encoder = Box::from_raw(video_encoder_ptr as *mut VideoEncoder);
            drop(video_encoder);

            msg_send![obj, release]
        }
    }
}
