use super::bindings::CMTime;
use objc::{
    class, msg_send,
    runtime::{Object, NO, YES},
    sel, sel_impl,
};

pub struct AVCaptureScreenInput {
    obj: *mut Object,
}

impl AVCaptureScreenInput {
    pub fn new(display_idx: u32) -> Self {
        let obj: *mut Object = unsafe {
            let cls = class!(AVCaptureScreenInput);
            let obj: *mut Object = msg_send![cls, alloc];
            msg_send![obj, initWithDisplayID: display_idx]
        };

        AVCaptureScreenInput { obj }
    }

    pub fn set_captures_cursor(&self, captures_cursor: bool) {
        unsafe {
            let value = match captures_cursor {
                true => YES,
                false => NO,
            };

            msg_send![self.obj, setCapturesCursor: value]
        }
    }

    pub fn set_captures_mouse_clicks(&self, captures_mouse_clicks: bool) {
        unsafe {
            let value = match captures_mouse_clicks {
                true => YES,
                false => NO,
            };

            msg_send![self.obj, setCapturesMouseClicks: value]
        }
    }

    pub fn set_min_frame_duration(&self, min_frame_duration: CMTime) {
        unsafe { msg_send![self.obj, setMinFrameDuration: min_frame_duration] }
    }

    pub fn obj_class(&self) -> *const Object {
        self.obj
    }
}

impl Drop for AVCaptureScreenInput {
    fn drop(&mut self) {
        unsafe { msg_send![self.obj, release] }
    }
}
