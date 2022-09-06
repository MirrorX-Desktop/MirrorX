use crate::{
    component::{
        audio_decoder::audio_decoder::AudioDecoder, audio_encoder::audio_encoder::AudioEncoder,
    },
    core_error,
    error::{CoreError, CoreResult},
    service::endpoint::message::*,
};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    InputCallbackInfo, OutputCallbackInfo, SampleFormat, SampleRate, SupportedStreamConfigRange,
};
use rtrb::{Consumer, Producer};
use scopeguard::defer;
use tracing::{error, info};

pub async fn start_audio_capture_process(
    remote_device_id: String,
    pcm_tx: crossbeam::channel::Sender<(Vec<f32>, u128)>,
) -> CoreResult<crossbeam::channel::Sender<()>> {
    let (exit_tx, exit_rx) = crossbeam::channel::bounded(1);
    let (inner_error_tx, inner_error_rx) = tokio::sync::oneshot::channel();

    let _ = std::thread::Builder::new()
        .name(format!("audio_capture_process:{}", remote_device_id))
        .spawn(move || {
            let host = cpal::default_host();

            let device = match host.default_output_device() {
                Some(device) => device,
                None => {
                    let _ = inner_error_tx
                        .send(Some(core_error!("default audio output device is null")));
                    return;
                }
            };

            info!(name=?device.name(),"select default audio output device");

            let supported_configs = match device.supported_output_configs() {
                Ok(config) => config,
                Err(err) => {
                    let _ = inner_error_tx.send(Some(core_error!(
                        "get audio device supported config failed ({})",
                        err
                    )));
                    return;
                }
            };

            let supported_config_vec: Vec<SupportedStreamConfigRange> =
                supported_configs.into_iter().collect();

            if supported_config_vec.len() == 0 {
                let _ = inner_error_tx
                    .send(Some(core_error!("no supported audio device output config")));
                return;
            }

            let sample_format = supported_config_vec[0].sample_format();

            if sample_format != SampleFormat::F32 {
                let _ = inner_error_tx.send(Some(core_error!(
                    "unsupported audio sample format {}",
                    sample_format.sample_size()
                )));
                return;
            }

            let output_config = supported_config_vec[0]
                .clone()
                .with_sample_rate(SampleRate(48000))
                .config();

            let mut audio_epoch: Option<std::time::Instant> = None;

            let input_callback = move |data: &[f32], info: &InputCallbackInfo| unsafe {
                let elapsed = if let Some(instant) = audio_epoch {
                    instant.elapsed().as_millis()
                } else {
                    let instant = std::time::Instant::now();
                    audio_epoch = Some(instant);
                    0
                };

                let _ = pcm_tx.try_send((data.to_vec(), elapsed));
            };

            let err_callback = |err| error!(?err, "error occurred on the output input stream");

            let loopback_stream =
                match device.build_input_stream(&output_config, input_callback, err_callback) {
                    Ok(stream) => stream,
                    Err(err) => {
                        let _ = inner_error_tx
                            .send(Some(core_error!("build input stream failed ({})", err)));
                        return;
                    }
                };

            if let Err(err) = loopback_stream.play() {
                let _ = inner_error_tx
                    .send(Some(core_error!("loop back stream play failed ({})", err)));
                return;
            }

            defer! {
                let _ = loopback_stream.pause();
                info!(?remote_device_id,"audio capture process exit");
            }

            let _ = inner_error_tx.send(None);
            let _ = exit_rx.recv();
        });

    match inner_error_rx.await {
        Ok(inner_err) => match inner_err {
            Some(err) => Err(err),
            None => Ok(exit_tx),
        },
        Err(err) => Err(core_error!(
            "receive start_audio_capture_process result failed ({})",
            err
        )),
    }
}

pub async fn start_audio_play_process(
    remote_device_id: String,
    mut samples_rx: Consumer<f32>,
) -> Result<crossbeam::channel::Sender<()>, CoreError> {
    let (exit_tx, exit_rx) = crossbeam::channel::bounded(1);
    let (inner_error_tx, inner_error_rx) = tokio::sync::oneshot::channel();

    let _ = std::thread::Builder::new()
        .name(format!("audio_play_process:{}", remote_device_id))
        .spawn(move || {
            let host = cpal::default_host();

            let device = match host.default_output_device() {
                Some(device) => device,
                None => {
                    let _ = inner_error_tx
                        .send(Some(core_error!("default audio output device is null")));
                    return;
                }
            };

            info!(name=?device.name(),"select default audio output device");

            let supported_configs = match device.supported_output_configs() {
                Ok(config) => config,
                Err(err) => {
                    let _ = inner_error_tx.send(Some(core_error!(
                        "get audio device supported config failed ({})",
                        err
                    )));
                    return;
                }
            };

            let supported_config_vec: Vec<SupportedStreamConfigRange> =
                supported_configs.into_iter().collect();

            if supported_config_vec.len() == 0 {
                let _ = inner_error_tx
                    .send(Some(core_error!("no supported audio device output config")));
                return;
            }

            let output_config = if let Some(config) = supported_config_vec
                .iter()
                .find(|config| config.max_sample_rate() == SampleRate(48000))
            {
                config.clone().with_sample_rate(SampleRate(48000)).config()
            } else {
                let _ = inner_error_tx.send(Some(core_error!(
                    "no supported audio device output config with sample rate 48000"
                )));
                return;
            };

            let sample_format = supported_config_vec[0].sample_format();

            if sample_format != SampleFormat::F32 {
                let _ = inner_error_tx.send(Some(core_error!(
                    "unsupported audio sample format {}",
                    sample_format.sample_size()
                )));
                return;
            }

            let input_callback = move |data: &mut [f32], info: &OutputCallbackInfo| {
                for b in data {
                    *b = match samples_rx.pop() {
                        Ok(v) => v,
                        Err(_) => Default::default(),
                    };
                }
            };

            let err_callback = |err| error!(?err, "error occurred on the output audio stream");

            let loopback_stream =
                match device.build_output_stream(&output_config, input_callback, err_callback) {
                    Ok(stream) => stream,
                    Err(err) => {
                        let _ = inner_error_tx
                            .send(Some(core_error!("build output stream failed ({})", err)));
                        return;
                    }
                };

            if let Err(err) = loopback_stream.play() {
                let _ = inner_error_tx
                    .send(Some(core_error!("loop back stream play failed ({})", err)));
                return;
            }

            defer! {
                let _ = loopback_stream.pause();
                info!(?remote_device_id,"audio frame play process exit");
            }

            let _ = inner_error_tx.send(None);
            let _ = exit_rx.recv();
        });

    match inner_error_rx.await {
        Ok(inner_error) => match inner_error {
            Some(err) => Err(err),
            None => Ok(exit_tx),
        },
        Err(err) => Err(core_error!(
            "receive start_audio_play_process result failed ({})",
            err
        )),
    }
}

pub fn start_audio_encode_process(
    remote_device_id: String,
    pcm_rx: crossbeam::channel::Receiver<(Vec<f32>, u128)>,
    packet_tx: tokio::sync::mpsc::Sender<EndPointMessagePacket>,
    sample_rate: i32,
    channels: isize,
) -> Result<(), CoreError> {
    let mut audio_encoder = AudioEncoder::new(sample_rate, channels)?;

    let _ = std::thread::Builder::new()
        .name(format!("audio_encode_process:{}", remote_device_id))
        .spawn(move || {
            loop {
                let (pcm_buffer, elpased) = match pcm_rx.recv() {
                    Ok(audio_frame) => audio_frame,
                    Err(_) => {
                        info!("pcm channel closed");
                        break;
                    }
                };

                let encoded_frame = match audio_encoder.encode(&pcm_buffer) {
                    Ok(buffer) => buffer,
                    Err(err) => {
                        error!(?err, "audio encode failed");
                        break;
                    }
                };

                let _ = packet_tx.try_send(EndPointMessagePacket {
                    typ: EndPointMessagePacketType::Push,
                    call_id: None,
                    message: EndPointMessage::AudioFrame(AudioFrame {
                        buffer: encoded_frame,
                        frame_size_per_channel: (pcm_buffer.len() as isize / channels) as u16,
                        elapsed: elpased,
                    }),
                });
            }

            info!(?remote_device_id, "audio encode process exit");
        });

    Ok(())
}

pub fn start_audio_decode_process(
    remote_device_id: String,
    sample_rate: i32,
    channels: isize,
    audio_frame_rx: crossbeam::channel::Receiver<AudioFrame>,
    mut pcm_producer: Producer<f32>,
) -> Result<(), CoreError> {
    let mut audio_decoder = AudioDecoder::new(sample_rate, channels)?;

    let _ = std::thread::Builder::new()
        .name(format!("audio_decode_process:{}", remote_device_id))
        .spawn(move || {
            loop {
                let audio_frame = match audio_frame_rx.recv() {
                    Ok(audio_frame) => audio_frame,
                    Err(_) => {
                        info!("audio frame receiver closed");
                        break;
                    }
                };

                let decoded_frame = match audio_decoder
                    .decode(&audio_frame.buffer, audio_frame.frame_size_per_channel)
                {
                    Ok(pcm) => pcm,
                    Err(err) => {
                        error!(?err, "audio decode failed");
                        break;
                    }
                };

                if let Ok(mut chunk) =
                    pcm_producer.write_chunk(decoded_frame.len().min(pcm_producer.slots()))
                {
                    unsafe {
                        let (first, second) = chunk.as_mut_slices();
                        let first_copy_length = decoded_frame.len().min(first.len());
                        std::ptr::copy_nonoverlapping(
                            decoded_frame.as_ptr(),
                            first.as_mut_ptr(),
                            first_copy_length,
                        );

                        let second_copy_length =
                            second.len().min(decoded_frame.len() - first_copy_length);

                        if second_copy_length > 0 {
                            std::ptr::copy_nonoverlapping(
                                decoded_frame.as_ptr().add(first_copy_length),
                                second.as_mut_ptr(),
                                second_copy_length,
                            );
                        }

                        chunk.commit(first_copy_length + second_copy_length);
                    }
                }
            }

            info!(?remote_device_id, "audio decode process exit");
        });

    Ok(())
}
