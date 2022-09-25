use crate::{
    api::endpoint::message::EndPointAudioFrame, component::audio::decoder::AudioDecoder,
    utility::runtime::TOKIO_RUNTIME,
};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    BufferSize, OutputCallbackInfo, StreamConfig,
};
use crossbeam::channel::{bounded, Sender};
use dashmap::DashMap;
use once_cell::{sync::Lazy, unsync::OnceCell};
use rtrb::{Consumer, Producer};
use scopeguard::defer;
use std::time::Duration;

static DECODERS: Lazy<DashMap<(i64, i64), crossbeam::channel::Sender<EndPointAudioFrame>>> =
    Lazy::new(DashMap::new);

pub async fn handle_audio_frame(
    active_device_id: i64,
    passive_device_id: i64,
    audio_frame: EndPointAudioFrame,
) {
    if let Some(tx) = DECODERS.get(&(active_device_id, passive_device_id)) {
        if tx.try_send(audio_frame).is_err() {
            tracing::error!(
                ?active_device_id,
                ?passive_device_id,
                "send audio frame failed"
            );
        }
    }
}

pub fn serve_audio_decode(active_device_id: i64, passive_device_id: i64) {
    if !DECODERS.contains_key(&(active_device_id, passive_device_id)) {
        let (audio_frame_tx, audio_frame_rx) = bounded(180);
        DECODERS.insert((active_device_id, passive_device_id), audio_frame_tx);

        TOKIO_RUNTIME.spawn_blocking(move || {
            defer! {
                tracing::info!(?active_device_id, ?passive_device_id, "decode audio frame process exit");
                DECODERS.remove(&(active_device_id, passive_device_id));
            }

            let mut current_decoder: OnceCell<AudioDecoder> = OnceCell::new();
            let mut current_render_exit_tx: OnceCell<Sender<()>> = OnceCell::new();
            let mut current_audio_sample_tx: OnceCell<Producer<f32>> = OnceCell::new();
            let (audio_render_process_exit_tx, audio_render_process_exit_rx) = bounded(0);

            loop {
                match audio_render_process_exit_rx.try_recv() {
                    Ok(_) => return,
                    Err(err) => {
                        if err.is_disconnected() {
                            return;
                        }
                    }
                };

                match audio_frame_rx.recv_timeout(Duration::from_secs(1)) {
                    Ok(audio_frame) => {
                        // until first audio frame arrived, we can know the actual frame size
                        // so we create audio decoder and audio render process when the first
                        // frame arrived or frame rate and frame size modified.

                        if let Some((sample_rate, sample_format, channels)) =
                            audio_frame.re_init_data
                        {
                            let (audio_sample_tx, audio_sample_rx) = rtrb::RingBuffer::new(180);

                            let _ = current_decoder.take();
                            let _ = current_render_exit_tx.take();
                            let _ = current_audio_sample_tx.take();

                            let new_decoder =
                                match AudioDecoder::new(sample_rate as i32, channels as isize) {
                                    Ok(new_decoder) => new_decoder,
                                    Err(err) => {
                                        tracing::error!(?err, "audio decoder initialize failed");
                                        return;
                                    }
                                };

                            let host = cpal::default_host();

                            let device = match host.default_output_device() {
                                Some(device) => device,
                                None => {
                                    tracing::error!("default audio output device not exist");
                                    return;
                                }
                            };

                            tracing::info!(name = ?device.name(), "select audio output device");

                            let mut stream_config = match device.default_output_config() {
                                Ok(output_config) => output_config.config(),
                                Err(err) => {
                                    tracing::error!(
                                        ?err,
                                        "get device default output config failed"
                                    );
                                    return;
                                }
                            };

                            stream_config.buffer_size = BufferSize::Fixed(
                                (audio_frame.buffer.len() / 4 / (channels as usize)) as u32,
                            );

                            let tx = serve_audio_render_process(
                                device,
                                stream_config,
                                audio_render_process_exit_tx.clone(),
                                audio_sample_rx,
                            );

                            if current_decoder.set(new_decoder).is_err() {
                                tracing::error!("current decoder should be empty!");
                                return;
                            }

                            if current_render_exit_tx.set(tx).is_err() {
                                tracing::error!("current render exit tx should be empty!");
                                return;
                            }

                            if current_audio_sample_tx.set(audio_sample_tx).is_err() {
                                tracing::error!("current audio sample tx should be empty!");
                                return;
                            }
                        }

                        if let (Some(decoder), Some(audio_sample_tx)) =
                            (current_decoder.get_mut(), current_audio_sample_tx.get_mut())
                        {
                            match decoder.decode(&audio_frame.buffer) {
                                Ok(mut buffer) => {
                                    while !(buffer).is_empty() {
                                        match audio_sample_tx.write_chunk_uninit(
                                            buffer.len().min(audio_sample_tx.slots()),
                                        ) {
                                            Ok(mut chunk) => {
                                                let (slice1, slice2) = chunk.as_mut_slices();

                                                let mut total_copy_length = 0;
                                                let mut copy_length =
                                                    slice1.len().min(buffer.len());
                                                unsafe {
                                                    std::ptr::copy_nonoverlapping(
                                                        buffer.as_ptr(),
                                                        slice1.as_mut_ptr() as *mut f32,
                                                        copy_length,
                                                    );
                                                }
                                                total_copy_length += copy_length;
                                                buffer = &buffer[copy_length..];

                                                if !buffer.is_empty() && !slice2.is_empty() {
                                                    copy_length = slice2.len().min(buffer.len());
                                                    unsafe {
                                                        std::ptr::copy_nonoverlapping(
                                                            buffer.as_ptr(),
                                                            slice2.as_mut_ptr() as *mut f32,
                                                            copy_length,
                                                        );
                                                    }
                                                    total_copy_length += copy_length;
                                                    buffer = &buffer[copy_length..];
                                                }

                                                unsafe { chunk.commit(total_copy_length) };
                                            }
                                            Err(err) => {
                                                tracing::error!(?err, "audio sample tx required invalid slots capacity");
                                                return;
                                            }
                                        }
                                    }
                                }
                                Err(err) => {
                                    tracing::error!(?err, "audio decoder decode failed");
                                    return;
                                }
                            }
                        } else {
                            tracing::error!("audio decoder not initialized");
                            return;
                        }
                    }
                    Err(err) => {
                        if err.is_disconnected() {
                            return;
                        }
                    }
                }
            }
        });
    }
}

fn serve_audio_render_process(
    device: cpal::Device,
    stream_config: StreamConfig,
    audio_render_process_exit_tx: Sender<()>,
    mut audio_sample_rx: Consumer<f32>,
) -> Sender<()> {
    let (callback_exit_tx, callback_exit_rx) = crossbeam::channel::bounded(1);
    let err_callback_exit_tx = callback_exit_tx.clone();
    let process_exit_tx = callback_exit_tx.clone();

    TOKIO_RUNTIME.spawn_blocking(move || {
        defer! {
            let _ = audio_render_process_exit_tx.send(());
        }

        let input_callback =
            move |data: &mut [f32], info: &OutputCallbackInfo| match audio_sample_rx
                .read_chunk(data.len().min(audio_sample_rx.slots()))
            {
                Ok(chunk) => {
                    for (i, v) in chunk.into_iter().enumerate() {
                        data[i] = v;
                    }
                }
                Err(err) => {
                    tracing::error!(?err, "audio sample rx required invalid slots capacity");
                    let _ = callback_exit_tx.send(());
                }
            };

        let err_callback = move |err| {
            tracing::error!(?err, "error occurred on the output audio stream");
            let _ = err_callback_exit_tx.send(());
        };

        let loopback_stream =
            match device.build_output_stream(&stream_config, input_callback, err_callback) {
                Ok(stream) => stream,
                Err(err) => {
                    tracing::error!(?err, "build audio output stream failed");
                    return;
                }
            };

        if let Err(err) = loopback_stream.play() {
            tracing::error!(?err, "audio loop back stream play failed");
            return;
        }

        defer! {
            let _ = loopback_stream.pause();
        }

        let _ = callback_exit_rx.recv();
    });

    process_exit_tx
}
