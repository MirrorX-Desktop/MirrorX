use super::bindings;
use crate::media::video_frame::VideoFrame;
use anyhow::bail;
use crossbeam_channel::{bounded, Receiver, Sender};
use std::ffi::c_void;

struct InnerDesktopDuplicatorPointer(*mut c_void);

unsafe impl Send for InnerDesktopDuplicatorPointer {}
unsafe impl Sync for InnerDesktopDuplicatorPointer {}

impl Drop for InnerDesktopDuplicatorPointer {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                bindings::desktop_duplicator_destroy(self.0);
            }
        }
    }
}

pub struct DesktopDuplicator {
    duplicator_ptr: InnerDesktopDuplicatorPointer,
    boxed_tx: Box<Sender<VideoFrame>>,
}

impl DesktopDuplicator {
    pub fn new(fps: i32) -> anyhow::Result<(DesktopDuplicator, Receiver<VideoFrame>)> {
        let (tx, rx) = bounded::<VideoFrame>(600);
        let mut boxed_tx = Box::new(tx);

        unsafe {
            let duplicator_ptr = bindings::desktop_duplicator_create(
                0,
                fps,
                boxed_tx.as_mut() as *mut _ as *mut c_void,
                bindings::callback,
            );

            if duplicator_ptr.is_null() {
                bail!("create duplicator failed");
            }

            Ok((
                DesktopDuplicator {
                    duplicator_ptr: InnerDesktopDuplicatorPointer(duplicator_ptr),
                    boxed_tx,
                },
                rx,
            ))
        }
    }

    pub fn start_capture(&self) {
        unsafe {
            bindings::desktop_duplicator_start(self.duplicator_ptr.0);
        }
    }

    pub fn stop_capture(&self) {
        unsafe {
            bindings::desktop_duplicator_stop(self.duplicator_ptr.0);
        }
    }
}
