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
    pub fn set_callback(&mut self, callback: impl Fn(*mut Object) -> () + 'static) {
        unsafe {
            let block = ConcreteBlock::new(callback).copy();
            let block_ptr = Box::into_raw(Box::new(block));
            let obj = &mut *(self as *mut _ as *mut Object);
            msg_send![obj, setCallback: block_ptr as *const c_void]
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

            extern "C" fn set_callback(this: &mut Object, _cmd: Sel, callback: *const c_void) {
                unsafe {
                    this.set_ivar("_capture_callback", callback);
                }
            }

            extern "C" fn get_callback(this: &Object, _cmd: Sel) -> *const c_void {
                unsafe { *this.get_ivar("_capture_callback") }
            }

            extern "C" fn capture_out_callback(
                this: &mut Object,
                _: Sel,
                _: *mut Object,
                didOutputSampleBuffer: *mut Object,
                _: *mut Object,
            ) {
                unsafe {
                    let callback_block: *mut RcBlock<(), ()> = msg_send![this, callback];
                    (&*callback_block).call(());
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

            let capture_out_callback_fn: extern "C" fn(
                &mut Object,
                Sel,
                *mut Object,
                *mut Object,
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
            let block = Box::from_raw(block_ptr as *mut RcBlock<(), ()>);
            drop(block);

            msg_send![obj, release]
        }
    }
}
