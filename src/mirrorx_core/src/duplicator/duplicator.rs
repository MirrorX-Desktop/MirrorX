use crate::duplicator::bindings;
use crate::frame::frame::Frame;
use log::error;
use std::{
    ffi::c_void,
    os::raw::c_int,
    ptr,
    sync::mpsc::{channel, Receiver, Sender},
};

pub struct Duplicator {
    inner_duplicator_context: *const c_void,
    tx_ptr: *const Sender<Frame>,
    rx: Receiver<Frame>,
}

impl Duplicator {
    pub fn new() -> Result<Self, String> {
        let (tx, rx) = channel::<Frame>();
        let tx_box = Box::new(tx);
        let tx_ptr: *const Sender<Frame> = Box::leak(tx_box);

        unsafe {
            let inner_duplicator_context = bindings::create_duplication_context(
                0,
                tx_ptr as *const c_void,
                Duplicator::callback,
            );

            if inner_duplicator_context.is_null() {
                return Err("Failed to create duplicator".to_string());
            }

            Ok(Duplicator {
                inner_duplicator_context,
                tx_ptr,
                rx,
            })
        }
    }

    pub fn start_capture(&self) {
        unsafe {
            bindings::start_capture(self.inner_duplicator_context);
        }
    }

    pub fn stop_capture(&self) {
        unsafe {
            bindings::stop_capture(self.inner_duplicator_context);
        }
    }

    pub fn get_frame(&self) -> Option<Frame> {
        match self.rx.recv() {
            Ok(frame) => Some(frame),
            Err(e) => {
                error!("{}", e);
                None
            }
        }
    }

    extern "C" fn callback(
        tx: *const c_void,
        width: c_int,
        height: c_int,
        y_line_size: c_int,
        y_buffer_address: *const u8,
        uv_line_size: c_int,
        uv_buffer_address: *const u8,
    ) {
        unsafe {
            if tx.is_null() {
                error!("capture callback: tx is null");
                return;
            }

            let y_length = y_line_size * height;
            let mut y_buffer = Vec::with_capacity(y_length as usize);
            ptr::copy_nonoverlapping(y_buffer_address, y_buffer.as_mut_ptr(), y_length as usize);
            y_buffer.set_len(y_length as usize);

            let uv_length = uv_line_size * height / 2;
            let mut uv_buffer = Vec::with_capacity(uv_length as usize);
            ptr::copy_nonoverlapping(
                uv_buffer_address,
                uv_buffer.as_mut_ptr(),
                uv_length as usize,
            );
            uv_buffer.set_len(uv_length as usize);

            let tx = &mut *(tx as *mut Sender<Frame>);
            tx.send(Frame::new(
                width,
                height,
                y_line_size,
                y_buffer,
                uv_line_size,
                uv_buffer,
            ))
            .ok();
        }
    }
}

impl Drop for Duplicator {
    fn drop(&mut self) {
        self.stop_capture();
        unsafe {
            Box::from_raw(self.tx_ptr as *mut Sender<Frame>);
            bindings::release_duplication_context(self.inner_duplicator_context);
        }
    }
}
