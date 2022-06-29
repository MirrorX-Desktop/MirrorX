use objc::{
    class, msg_send,
    rc::autoreleasepool,
    runtime::{Object, BOOL, YES},
    sel, sel_impl,
};
use objc_foundation::{INSString, NSString};

use super::{
    av_capture_screen_input::AVCaptureScreenInput,
    av_capture_video_data_output::AVCaptureVideoDataOutput,
};

pub enum AVCaptureSessionPreset {
    AVCaptureSessionPresetHigh,
}

pub struct AVCaptureSession {
    obj: *mut Object,
    _input: Option<AVCaptureScreenInput>,
    _output: Option<AVCaptureVideoDataOutput>,
}

impl AVCaptureSession {
    pub fn new() -> Self {
        let obj: *mut Object = unsafe {
            let obj: *mut Object = msg_send![class!(AVCaptureSession), alloc];
            msg_send![obj, init]
        };

        AVCaptureSession {
            obj,
            _input: None,
            _output: None,
        }
    }

    pub fn begin_configuration(&self) {
        unsafe { msg_send![self.obj, beginConfiguration] }
    }

    pub fn commit_configuration(&self) {
        unsafe { msg_send![self.obj, commitConfiguration] }
    }

    pub fn set_session_preset(&self, session_preset: AVCaptureSessionPreset) {
        autoreleasepool(|| unsafe {
            let session_preset_str = match session_preset {
                AVCaptureSessionPreset::AVCaptureSessionPresetHigh => "AVCaptureSessionPresetHigh",
            };

            msg_send![
                self.obj,
                setSessionPreset: NSString::from_str(session_preset_str)
            ]
        })
    }

    pub fn can_add_input(&self, input: &AVCaptureScreenInput) -> bool {
        unsafe {
            let b: BOOL = msg_send![self.obj, canAddInput: input.obj_class()];
            b == YES
        }
    }

    pub fn add_input(&mut self, input: AVCaptureScreenInput) {
        unsafe {
            let _: () = msg_send![self.obj, addInput: input.obj_class()];
        }
        self._input = Some(input);
    }

    pub fn can_add_output(&self, output: &AVCaptureVideoDataOutput) -> bool {
        unsafe {
            let b: BOOL = msg_send![self.obj, canAddOutput: output.obj_class()];
            b == YES
        }
    }

    pub fn add_output(&mut self, input: AVCaptureVideoDataOutput) {
        unsafe {
            let _: () = msg_send![self.obj, addOutput: input.obj_class()];
        }
        self._output = Some(input);
    }

    pub fn start_running(&self) {
        unsafe { msg_send![self.obj, startRunning] }
    }

    pub fn stop_running(&self) {
        unsafe { msg_send![self.obj, stopRunning] }
    }
}

impl Drop for AVCaptureSession {
    fn drop(&mut self) {
        unsafe { msg_send![self.obj, release] }
    }
}
