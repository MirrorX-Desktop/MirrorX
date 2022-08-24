use crate::{
    check_if_failed,
    component::media_foundation::{
        enumerator::{enum_descriptors, Descriptor},
        log::log_media_type,
    },
    error::{CoreResult, MirrorXError},
};
use once_cell::sync::OnceCell;
use scopeguard::defer;
use std::{ffi::OsString, mem::ManuallyDrop, os::windows::prelude::OsStringExt, str::FromStr};
use windows::{
    core::{Interface, GUID},
    Win32::{
        Foundation::MAX_PATH,
        Graphics::Direct3D11::{ID3D11Device, ID3D11Texture2D},
        Media::MediaFoundation::*,
        System::{
            Com::*,
            Ole::{VT_BOOL, VT_UI4},
        },
    },
};

pub struct VideoEncoder {
    frame_width: u16,
    frame_height: u16,
    descriptor: Descriptor,
    event_generator: Option<IMFMediaEventGenerator>,
    needs_create_output_sample: bool,
    transform: IMFTransform,
    async_need_input: bool,
    async_have_output: bool,
    draining: bool,
    draining_done: bool,
    async_marker: bool,
    sample_sent: bool,
    output_stream_info: MFT_OUTPUT_STREAM_INFO,
    device_manager: Option<IMFDXGIDeviceManager>,
}

impl VideoEncoder {
    pub fn new(
        frame_width: u16,
        frame_height: u16,
        fps: u8,
        descriptor: &Descriptor,
        device: &ID3D11Device,
    ) -> CoreResult<VideoEncoder> {
        unsafe {
            let (input_media_type, output_media_type) =
                create_input_and_output_media_type(frame_width, frame_height, fps)?;

            let guid = GUID::from(descriptor.guid());

            let transform: IMFTransform =
                check_if_failed!(CoCreateInstance(&guid, None, CLSCTX_ALL));

            let attributes = check_if_failed!(transform.GetAttributes());

            let support_d3d11 = match attributes.GetUINT32(&MF_SA_D3D11_AWARE) {
                Ok(res) => res > 0,
                Err(_) => false,
            };

            let device_manager = if support_d3d11 {
                let mut reset_token = 0;
                let mut device_manager = None;

                check_if_failed!(MFCreateDXGIDeviceManager(
                    &mut reset_token,
                    &mut device_manager
                ));

                if let Some(device_manager) = &device_manager {
                    check_if_failed!(device_manager.ResetDevice(device, reset_token));
                }

                device_manager
            } else {
                None
            };

            if descriptor.is_async() {
                check_if_failed!(attributes.SetUINT32(&MF_TRANSFORM_ASYNC_UNLOCK, 1));
            }

            let codec_api: ICodecAPI = check_if_failed!(transform.cast());

            set_codec_api_rate_control_mode(&codec_api)?;
            set_codec_api_max_bit_rate(&codec_api, 40000000)?;
            set_codec_api_low_latency_mode(&codec_api)?;
            set_codec_api_gop(&codec_api, 120)?;
            set_codec_api_b_frames_count(&codec_api, 0)?;
            set_codec_api_entropy_encoding(&codec_api)?;

            tracing::info!(
                name = descriptor.name(),
                is_async = descriptor.is_async(),
                is_hardware = descriptor.is_hardware(),
                "activating media foundation encoder"
            );

            tracing::info!("input media type attributes");
            log_media_type(&input_media_type)?;

            tracing::info!("output media type attributes");
            log_media_type(&output_media_type)?;

            if let Some(device_manager) = &device_manager {
                check_if_failed!(transform.ProcessMessage(
                    MFT_MESSAGE_SET_D3D_MANAGER,
                    std::mem::transmute(device_manager.as_raw())
                ));
            }
            check_if_failed!(transform.ProcessMessage(MFT_MESSAGE_NOTIFY_BEGIN_STREAMING, 0));
            check_if_failed!(transform.ProcessMessage(MFT_MESSAGE_NOTIFY_START_OF_STREAM, 0));

            let event_generator = if descriptor.is_async() {
                let e: IMFMediaEventGenerator = check_if_failed!(transform.cast());
                Some(e)
            } else {
                None
            };

            let stream_info = check_if_failed!(transform.GetOutputStreamInfo(0));
            let needs_create_output_sample = stream_info.dwFlags
                & (MFT_OUTPUT_STREAM_PROVIDES_SAMPLES.0 as u32
                    | MFT_OUTPUT_STREAM_CAN_PROVIDE_SAMPLES.0 as u32)
                == 0;

            Ok(VideoEncoder {
                frame_width,
                frame_height,
                descriptor: descriptor.clone(),
                event_generator,
                needs_create_output_sample,
                transform,
                async_need_input: false,
                async_have_output: false,
                draining: false,
                draining_done: false,
                async_marker: false,
                sample_sent: false,
                output_stream_info: stream_info,
                device_manager,
            })
        }
    }

    pub fn encode(&mut self, texture: &ID3D11Texture2D) -> CoreResult<()> {
        unsafe {
            let image_size = check_if_failed!(MFCalculateImageSize(
                &MFVideoFormat_NV12,
                self.frame_width as u32,
                self.frame_height as u32
            ));

            let in_sample = create_sample(texture)?;
            check_if_failed!(in_sample.SetSampleTime(0));
            check_if_failed!(in_sample.SetSampleDuration(0));

            self.send_sample(in_sample)?;

            let out_sample = self.receive_sample()?;

            Ok(())
        }
    }

    // unsafe fn drain_event(&self, block: bool) -> Result<bool, MirrorXError> {
    //     if let Some(event_generator) = self.event_generator {
    //         let event_flags = if block {
    //             MF_EVENT_FLAG_NONE
    //         } else {
    //             MF_EVENT_FLAG_NO_WAIT
    //         };

    //         match event_generator.GetEvent(event_flags) {
    //             Ok(event) => {
    //                 let typ = syscall_check!(event.GetType());
    //                 let status = syscall_check!(event.GetStatus());
    //                 if status.is_ok() {
    //                     if typ == METransformNeedInput.0 {}
    //                 }
    //             }
    //             Err(hr) => hr.code() == MF_E_NO_EVENTS_AVAILABLE,
    //         }
    //     }

    //     false
    // }

    unsafe fn wait_events(&mut self) -> Result<(), MirrorXError> {
        if !self.descriptor.is_async() {
            return Ok(());
        }

        while !(self.async_need_input
            || self.async_have_output
            || self.draining_done
            || self.async_marker)
        {
            if let Some(event_generator) = &self.event_generator {
                let event = check_if_failed!(event_generator.GetEvent(MF_EVENT_FLAG_NONE));
                let event_type = check_if_failed!(event.GetType());
                match MF_EVENT_TYPE(event_type as i32) {
                    METransformNeedInput => {
                        if !self.draining {
                            self.async_need_input = true
                        }
                    }
                    METransformHaveOutput => self.async_have_output = true,
                    METransformDrainComplete => self.draining_done = true,
                    METransformMarker => self.async_marker = true,
                    _ => {}
                }
            }
        }

        return Ok(());
    }

    unsafe fn send_sample(&mut self, sample: IMFSample) -> CoreResult<()> {
        if true {
            if self.descriptor.is_async() {
                self.wait_events()?;
                if !self.async_need_input {
                    return Err(MirrorXError::TryAgain);
                }
            }

            if !self.sample_sent {
                check_if_failed!(sample.SetUINT32(&MFSampleExtension_Discontinuity, 1));
            }
            self.sample_sent = true;

            if let Err(hr) = self.transform.ProcessInput(0, &sample, 0) {
                if hr.code() == MF_E_NOTACCEPTING {
                    return Err(MirrorXError::TryAgain);
                } else {
                    return Err(MirrorXError::Syscall {
                        code: hr.code(),
                        message: hr.message().to_string(),
                        file: String::from(file!()),
                        line: line!().to_string(),
                    });
                }
            }

            self.async_need_input = false;
        } else if !self.draining {
            if let Err(err) = self.transform.ProcessMessage(MFT_MESSAGE_COMMAND_DRAIN, 0) {
                tracing::error!(
                    "process message 'MFT_MESSAGE_COMMAND_DRAIN' failed ({})",
                    err
                );
            }

            self.draining = true;
            self.async_need_input = false;
        } else {
            return Err(MirrorXError::EOF);
        }

        return Ok(());
    }

    unsafe fn receive_sample(&mut self) -> CoreResult<IMFSample> {
        let mut out_sample = None;

        loop {
            let mut sample = None;

            if self.descriptor.is_async() {
                self.wait_events()?;
                if !self.async_have_output || self.draining_done {
                    break;
                }
            }

            if self.needs_create_output_sample {
                sample = Some(create_memory_sample(
                    self.output_stream_info.cbSize,
                    self.output_stream_info.cbAlignment,
                )?);
            }

            let mut out_buffers: [MFT_OUTPUT_DATA_BUFFER; 1] = std::mem::zeroed();
            out_buffers[0].dwStreamID = 0;
            out_buffers[0].pSample = sample;

            let mut status = 0;
            match self
                .transform
                .ProcessOutput(0, &mut out_buffers, &mut status)
            {
                Ok(_) => {
                    out_sample = out_buffers[0].pSample.clone();
                    break;
                }
                Err(err) => match err.code() {
                    MF_E_TRANSFORM_NEED_MORE_INPUT => {
                        if self.draining {
                            self.draining_done = true;
                        }
                    }
                    MF_E_TRANSFORM_STREAM_CHANGE => {
                        tracing::info!("stream format changed");
                        // ret = mf_choose_output_type(avctx);
                        // if (ret == 0) // we don't expect renegotiating the input type
                        //     ret = AVERROR_EXTERNAL;
                        // if (ret > 0) {
                        //     ret = mf_setup_context(avctx);
                        //     if (ret >= 0) {
                        //         c->async_have_output = 0;
                        //         continue;
                        //     }
                        // }
                    }
                    _ => {
                        tracing::error!("processing output failed ({})", err);
                        return Err(MirrorXError::Syscall {
                            code: err.code(),
                            message: err.message().to_string(),
                            file: String::from(file!()),
                            line: line!().to_string(),
                        });
                    }
                },
            };

            break;
        }

        self.async_have_output = false;

        match out_sample {
            Some(sample) => Ok(sample),
            None => {
                if self.draining_done {
                    Err(MirrorXError::EOF)
                } else {
                    Err(MirrorXError::TryAgain)
                }
            }
        }
    }
}

fn set_codec_api_rate_control_mode(codec_api: &ICodecAPI) -> Result<(), MirrorXError> {
    unsafe {
        let val: VARIANT = VARIANT {
            Anonymous: VARIANT_0 {
                Anonymous: ManuallyDrop::new(VARIANT_0_0 {
                    vt: VT_UI4.0 as u16,
                    Anonymous: VARIANT_0_0_0 {
                        ulVal: eAVEncCommonRateControlMode_PeakConstrainedVBR.0 as u32,
                    },
                    ..Default::default()
                }),
            },
        };

        defer! {
            drop(&val.Anonymous.Anonymous);
        }

        check_if_failed!(codec_api.SetValue(&CODECAPI_AVEncCommonRateControlMode, &val));

        Ok(())
    }
}

fn set_codec_api_max_bit_rate(codec_api: &ICodecAPI, bit_rate: u32) -> Result<(), MirrorXError> {
    unsafe {
        let val: VARIANT = VARIANT {
            Anonymous: VARIANT_0 {
                Anonymous: ManuallyDrop::new(VARIANT_0_0 {
                    vt: VT_UI4.0 as u16,
                    Anonymous: VARIANT_0_0_0 { ulVal: bit_rate },
                    ..Default::default()
                }),
            },
        };

        defer! {
            drop(&val.Anonymous.Anonymous);
        }

        check_if_failed!(codec_api.SetValue(&CODECAPI_AVEncCommonMaxBitRate, &val));

        Ok(())
    }
}

fn set_codec_api_low_latency_mode(codec_api: &ICodecAPI) -> Result<(), MirrorXError> {
    unsafe {
        let val: VARIANT = VARIANT {
            Anonymous: VARIANT_0 {
                Anonymous: ManuallyDrop::new(VARIANT_0_0 {
                    vt: VT_BOOL.0 as u16,
                    Anonymous: VARIANT_0_0_0 { boolVal: -1 }, // true in variant is -1(0xFFFF) and false is 0
                    ..Default::default()
                }),
            },
        };

        defer! {
            drop(&val.Anonymous.Anonymous);
        }

        check_if_failed!(codec_api.SetValue(&CODECAPI_AVLowLatencyMode, &val));

        Ok(())
    }
}

fn set_codec_api_gop(codec_api: &ICodecAPI, gop_size: u32) -> Result<(), MirrorXError> {
    unsafe {
        let val: VARIANT = VARIANT {
            Anonymous: VARIANT_0 {
                Anonymous: ManuallyDrop::new(VARIANT_0_0 {
                    vt: VT_UI4.0 as u16,
                    Anonymous: VARIANT_0_0_0 { ulVal: gop_size },
                    ..Default::default()
                }),
            },
        };

        defer! {
            drop(&val.Anonymous.Anonymous);
        }

        check_if_failed!(codec_api.SetValue(&CODECAPI_AVEncMPVGOPSize, &val));

        Ok(())
    }
}

fn set_codec_api_b_frames_count(
    codec_api: &ICodecAPI,
    b_frames_count: u32,
) -> Result<(), MirrorXError> {
    unsafe {
        let val: VARIANT = VARIANT {
            Anonymous: VARIANT_0 {
                Anonymous: ManuallyDrop::new(VARIANT_0_0 {
                    vt: VT_UI4.0 as u16,
                    Anonymous: VARIANT_0_0_0 {
                        ulVal: b_frames_count,
                    },
                    ..Default::default()
                }),
            },
        };

        defer! {
            drop(&val.Anonymous.Anonymous);
        }

        check_if_failed!(codec_api.SetValue(&CODECAPI_AVEncMPVDefaultBPictureCount, &val));

        Ok(())
    }
}

fn set_codec_api_entropy_encoding(codec_api: &ICodecAPI) -> Result<(), MirrorXError> {
    unsafe {
        let val: VARIANT = VARIANT {
            Anonymous: VARIANT_0 {
                Anonymous: ManuallyDrop::new(VARIANT_0_0 {
                    vt: VT_BOOL.0 as u16,
                    Anonymous: VARIANT_0_0_0 { boolVal: -1 }, // true in variant is -1(0xFFFF) and false is 0
                    ..Default::default()
                }),
            },
        };

        defer! {
            drop(&val.Anonymous.Anonymous);
        }

        check_if_failed!(codec_api.SetValue(&CODECAPI_AVEncH264CABACEnable, &val));

        Ok(())
    }
}

unsafe fn create_input_and_output_media_type(
    frame_width: u16,
    frame_height: u16,
    fps: u8,
) -> Result<(IMFMediaType, IMFMediaType), MirrorXError> {
    let input_media_type = check_if_failed!(MFCreateMediaType());

    check_if_failed!(input_media_type.SetGUID(&MF_MT_MAJOR_TYPE, &MFMediaType_Video));

    check_if_failed!(input_media_type.SetGUID(&MF_MT_SUBTYPE, &MFVideoFormat_NV12));

    check_if_failed!(
        input_media_type.SetUINT32(&MF_MT_INTERLACE_MODE, MFVideoInterlace_Progressive.0 as u32)
    );

    check_if_failed!(input_media_type.SetUINT64(
        &MF_MT_FRAME_SIZE,
        ((frame_width as u64) << 32) | frame_height as u64,
    ));

    check_if_failed!(
        input_media_type.SetUINT64(&MF_MT_FRAME_RATE_RANGE_MAX, ((fps as u64) << 32) | 1u64)
    );

    check_if_failed!(input_media_type.SetUINT64(&MF_MT_PIXEL_ASPECT_RATIO, (1u64 << 32) | 1u64));

    let output_media_type = check_if_failed!(MFCreateMediaType());

    check_if_failed!(output_media_type.SetGUID(&MF_MT_MAJOR_TYPE, &MFMediaType_Video));

    check_if_failed!(output_media_type.SetGUID(&MF_MT_SUBTYPE, &MFVideoFormat_H264));

    check_if_failed!(
        output_media_type.SetUINT32(&MF_MT_INTERLACE_MODE, MFVideoInterlace_Progressive.0 as u32)
    );

    check_if_failed!(output_media_type.SetUINT64(
        &MF_MT_FRAME_SIZE,
        ((frame_width as u64) << 32) | frame_height as u64,
    ));

    check_if_failed!(
        output_media_type.SetUINT64(&MF_MT_FRAME_RATE_RANGE_MAX, ((fps as u64) << 32) | 1u64)
    );

    check_if_failed!(output_media_type.SetUINT64(&MF_MT_PIXEL_ASPECT_RATIO, (1u64 << 32) | 1u64));

    Ok((input_media_type, output_media_type))
}

unsafe fn create_sample(texture: &ID3D11Texture2D) -> CoreResult<IMFSample> {
    let media_buffer = check_if_failed!(MFCreateDXGISurfaceBuffer(
        &ID3D11Texture2D::IID,
        texture,
        0,
        false
    ));

    let media_2d_buffer: IMF2DBuffer = check_if_failed!(media_buffer.cast());

    let length = check_if_failed!(media_2d_buffer.GetContiguousLength());
    check_if_failed!(media_buffer.SetCurrentLength(length));

    let sample = check_if_failed!(MFCreateVideoSampleFromSurface(None));
    check_if_failed!(sample.AddBuffer(media_buffer));

    Ok(sample)
}

unsafe fn create_memory_sample(size: u32, align: u32) -> CoreResult<IMFSample> {
    let sample = check_if_failed!(MFCreateSample());
    let media_buffer = check_if_failed!(MFCreateAlignedMemoryBuffer(size, align.max(16) - 1));
    check_if_failed!(sample.AddBuffer(media_buffer));
    Ok(sample)
}
