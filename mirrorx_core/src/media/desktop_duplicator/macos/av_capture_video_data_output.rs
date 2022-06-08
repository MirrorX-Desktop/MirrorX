use crate::media::bindings::macos::{
    kCVPixelBufferHeightKey, kCVPixelBufferPixelFormatTypeKey, kCVPixelBufferWidthKey,
    kCVPixelFormatType_420YpCbCr8BiPlanarFullRange, CMSampleBufferRef,
};
use crate::media::{
    desktop_duplicator::macos::video_data_output_callback::VideoDataOutputCallback,
    video_encoder::VideoEncoder,
};
use core_foundation::{base::ToVoid, dictionary::CFMutableDictionary, number::CFNumber};
use dispatch::ffi::{dispatch_queue_create, dispatch_release, DISPATCH_QUEUE_SERIAL};
use objc::{
    class, msg_send,
    runtime::{Object, YES},
    sel, sel_impl,
};
use objc_foundation::INSObject;
use objc_id::Id;
use std::ffi::{c_void, CString};

pub struct AVCaptureVideoDataOutput {
    obj: *mut Object,
    _delegate: Id<VideoDataOutputCallback>,
}

impl AVCaptureVideoDataOutput {
    pub fn new(
        video_encoder: VideoEncoder,
        callback: impl Fn(&mut VideoEncoder, CMSampleBufferRef) -> () + 'static,
    ) -> Self {
        unsafe {
            let cls = class!(AVCaptureVideoDataOutput);
            let obj: *mut Object = msg_send![cls, new];

            let mut video_settings = CFMutableDictionary::new();
            video_settings.add(
                &kCVPixelBufferPixelFormatTypeKey.to_void(),
                &CFNumber::from(kCVPixelFormatType_420YpCbCr8BiPlanarFullRange),
            );
            video_settings.add(&kCVPixelBufferWidthKey.to_void(), &CFNumber::from(1920i32));
            video_settings.add(&kCVPixelBufferHeightKey.to_void(), &CFNumber::from(1080i32));

            let mut delegate = VideoDataOutputCallback::new();
            delegate.set_callback(callback);
            delegate.set_video_encoder(video_encoder);

            let queue_label =
                CString::new("cloud.mirrorx.desktop_duplicator.video_data_output_queue").unwrap();
            let queue = dispatch_queue_create(queue_label.as_ptr(), DISPATCH_QUEUE_SERIAL);

            let _: () = msg_send![obj, setAlwaysDiscardsLateVideoFrames: YES];
            let _: () = msg_send![obj, setVideoSettings: video_settings.to_untyped().to_void()];
            let _: () = msg_send![obj, setSampleBufferDelegate: &*delegate
                                                 queue: queue as *const _ as *const c_void];

            dispatch_release(queue);

            AVCaptureVideoDataOutput {
                obj,
                _delegate: delegate,
            }
        }
    }

    pub fn obj_class(&self) -> *const Object {
        self.obj
    }
}
