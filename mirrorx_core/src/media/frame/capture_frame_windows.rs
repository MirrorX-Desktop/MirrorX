use std::sync::Arc;

use crossbeam_channel::{Receiver, Sender};

pub struct CaptureFrame {
    width: i32,
    height: i32,
    luminance_buffer: &'static [u8],
    luminance_stride: u32,
    chrominance_buffer: &'static [u8],
    chrominance_stride: u32,
    notify_tx: Sender<()>,
    notify_rx: Receiver<()>,
}

impl CaptureFrame {
    pub fn new(
        width: i32,
        height: i32,
        luminance_buffer: &'static [u8],
        luminance_stride: u32,
        chrominance_buffer: &'static [u8],
        chrominance_stride: u32,
    ) -> Arc<Self> {
        let (notify_tx, notify_rx) = crossbeam_channel::bounded(0);

        Arc::new(CaptureFrame {
            width,
            height,
            luminance_buffer,
            luminance_stride,
            chrominance_buffer,
            chrominance_stride,
            notify_tx,
            notify_rx,
        })
    }

    pub fn notify(&self) {
        self.notify_tx.send(());
    }

    pub fn wait(&self) {
        self.notify_rx.recv();
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn luminance_buffer(&self) -> &[u8] {
        self.luminance_buffer
    }

    pub fn luminance_stride(&self) -> u32 {
        self.luminance_stride
    }

    pub fn chrominance_buffer(&self) -> &[u8] {
        self.chrominance_buffer
    }

    pub fn chrominance_stride(&self) -> u32 {
        self.chrominance_stride
    }
}
