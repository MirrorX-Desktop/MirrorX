use crossbeam::channel::Sender;
use libc::size_t;

pub struct CaptureFrame {
    width: size_t,
    height: size_t,
    luminance_buffer: &'static [u8],
    luminance_stride: size_t,
    chrominance_buffer: &'static [u8],
    chrominance_stride: size_t,
    notify_release_tx: Sender<()>,
}

impl CaptureFrame {
    pub fn new(
        width: size_t,
        height: size_t,
        luminance_buffer: &'static [u8],
        luminance_stride: size_t,
        chrominance_buffer: &'static [u8],
        chrominance_stride: size_t,
        notify_release_tx: Sender<()>,
    ) -> Self {
        CaptureFrame {
            width,
            height,
            luminance_buffer,
            luminance_stride,
            chrominance_buffer,
            chrominance_stride,
            notify_release_tx,
        }
    }

    pub fn width(&self) -> size_t {
        self.width
    }

    pub fn height(&self) -> size_t {
        self.height
    }

    pub fn luminance_buffer(&self) -> &[u8] {
        self.luminance_buffer
    }

    pub fn luminance_stride(&self) -> size_t {
        self.luminance_stride
    }

    pub fn chrominance_buffer(&self) -> &[u8] {
        self.chrominance_buffer
    }

    pub fn chrominance_stride(&self) -> size_t {
        self.chrominance_stride
    }

    pub fn notify_frame_release(&self) {
        let _ = self.notify_release_tx.send(());
    }
}
