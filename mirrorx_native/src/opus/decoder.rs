pub enum OpusDecoder {}

extern "C" {
    pub fn opus_decoder_get_size(channels: isize) -> isize;
    pub fn opus_decoder_create(fs: i32, channels: isize, error: *mut isize) -> *mut OpusDecoder;
    pub fn opus_decoder_init(st: *mut OpusDecoder, fs: i32, channels: isize) -> isize;
    pub fn opus_decode(
        st: *mut OpusDecoder,
        data: *const u8,
        len: i32,
        pcm: *mut i16,
        frame_size: isize,
        decodec_fec: isize,
    ) -> isize;
    pub fn opus_decode_float(
        st: *mut OpusDecoder,
        data: *const u8,
        len: i32,
        pcm: *mut f32,
        frame_size: isize,
        decodec_fec: isize,
    ) -> isize;
    pub fn opus_decoder_destroy(st: *mut OpusDecoder);
    pub fn opus_packet_parse(
        data: *const u8,
        len: i32,
        out_toc: *mut u8,
        frames: &[*mut u8; 48],
        size: &[i16; 48],
        payload_offset: *mut isize,
    ) -> isize;
    pub fn opus_packet_get_bandwidth(data: *const u8) -> isize;
    pub fn opus_packet_get_samples_per_frame(data: *const u8, fs: i32) -> isize;
    pub fn opus_packet_get_nb_channels(data: *const u8) -> isize;
    pub fn opus_packet_get_nb_frames(packet: *const u8, len: i32) -> isize;
    pub fn opus_packet_get_nb_samples(packet: *const u8, len: i32, fs: i32) -> isize;
    pub fn opus_decoder_get_nb_samples(
        dec: *const OpusDecoder,
        packet: *const u8,
        len: i32,
    ) -> isize;
    pub fn opus_pcm_soft_clip(
        pcm: *mut f32,
        frame_size: isize,
        channels: isize,
        softclip_mem: *mut f32,
    );
}
