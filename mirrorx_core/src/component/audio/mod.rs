use self::{decoder::AudioDecoder, encoder::AudioEncoder};
use cpal::{traits::StreamTrait, SampleRate};

pub mod decoder;
pub mod duplicator;
pub mod encoder;
pub mod player;
pub mod resampler;

#[test]
fn sound() {
    tracing_subscriber::fmt::init();

    let mut encoder = AudioEncoder::default();
    let mut decoder = AudioDecoder::new(2, cpal::SampleFormat::F32, SampleRate(48000));

    let (stream, mut rx) = duplicator::new_record_stream_and_rx().unwrap();
    stream.play().unwrap();

    let (ps, tx) =
        player::new_play_stream_and_tx(2, cpal::SampleFormat::F32, SampleRate(44100), 960).unwrap();

    ps.play().unwrap();

    loop {
        let frame = rx.blocking_recv().unwrap();
        tracing::info!("{:?}", frame.buffer.len());

        match encoder.encode(frame) {
            Ok(encoded_frame) => {
                let d = decoder.decode(encoded_frame).unwrap();
                let _ = tx.blocking_send(d);
            }
            Err(err) => tracing::error!(?err, "failed"),
        }
    }
}
