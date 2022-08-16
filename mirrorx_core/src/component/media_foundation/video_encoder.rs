use crate::{
    component::media_foundation::{
        enumerator::{enum_descriptors, Descriptor},
        log::log_media_type,
    },
    error::MirrorXError,
};
use once_cell::sync::OnceCell;
use scopeguard::defer;
use std::{ffi::OsString, mem::ManuallyDrop, os::windows::prelude::OsStringExt, str::FromStr};
use windows::{
    core::{Interface, GUID},
    Win32::{
        Foundation::MAX_PATH,
        Media::MediaFoundation::*,
        System::{
            Com::*,
            Ole::{VT_BOOL, VT_UI4},
        },
    },
};

pub struct VideoEncoder {
    event_generator: Option<IMFMediaEventGenerator>,
    needs_create_output_sample: bool,
}

impl VideoEncoder {
    pub fn new(
        frame_width: u16,
        frame_height: u16,
        fps: u8,
        descriptor: &Descriptor,
    ) -> Result<VideoEncoder, MirrorXError> {
        unsafe {
            let (input_media_type, output_media_type) =
                create_input_and_output_media_type(frame_width, frame_height, fps)?;

            let guid = GUID::from(descriptor.guid());

            let transform: IMFTransform = syscall_check!(CoCreateInstance(&guid, None, CLSCTX_ALL));
            if descriptor.is_async() {
                let attributes = syscall_check!(transform.GetAttributes());
                syscall_check!(attributes.SetUINT32(&MF_TRANSFORM_ASYNC_UNLOCK, 1));
            }

            let codec_api: ICodecAPI = syscall_check!(transform.cast());

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

            syscall_check!(transform.ProcessMessage(MFT_MESSAGE_NOTIFY_BEGIN_STREAMING, 0));
            syscall_check!(transform.ProcessMessage(MFT_MESSAGE_NOTIFY_START_OF_STREAM, 0));

            let event_generator = if descriptor.is_async() {
                let e: IMFMediaEventGenerator = syscall_check!(transform.cast());
                Some(e)
            } else {
                None
            };

            let stream_info = syscall_check!(transform.GetOutputStreamInfo(0));
            let needs_create_output_sample = stream_info.dwFlags
                & (MFT_OUTPUT_STREAM_PROVIDES_SAMPLES.0 as u32
                    | MFT_OUTPUT_STREAM_CAN_PROVIDE_SAMPLES.0 as u32)
                == 0;

            Ok(VideoEncoder {
                event_generator,
                needs_create_output_sample,
            })
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

        syscall_check!(codec_api.SetValue(&CODECAPI_AVEncCommonRateControlMode, &val));

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

        syscall_check!(codec_api.SetValue(&CODECAPI_AVEncCommonMaxBitRate, &val));

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

        syscall_check!(codec_api.SetValue(&CODECAPI_AVLowLatencyMode, &val));

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

        syscall_check!(codec_api.SetValue(&CODECAPI_AVEncMPVGOPSize, &val));

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

        syscall_check!(codec_api.SetValue(&CODECAPI_AVEncMPVDefaultBPictureCount, &val));

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

        syscall_check!(codec_api.SetValue(&CODECAPI_AVEncH264CABACEnable, &val));

        Ok(())
    }
}

unsafe fn create_input_and_output_media_type(
    frame_width: u16,
    frame_height: u16,
    fps: u8,
) -> Result<(IMFMediaType, IMFMediaType), MirrorXError> {
    let input_media_type = syscall_check!(MFCreateMediaType());

    syscall_check!(input_media_type.SetGUID(&MF_MT_MAJOR_TYPE, &MFMediaType_Video));

    syscall_check!(input_media_type.SetGUID(&MF_MT_SUBTYPE, &MFVideoFormat_NV12));

    syscall_check!(
        input_media_type.SetUINT32(&MF_MT_INTERLACE_MODE, MFVideoInterlace_Progressive.0 as u32)
    );

    syscall_check!(input_media_type.SetUINT64(
        &MF_MT_FRAME_SIZE,
        ((frame_width as u64) << 32) | frame_height as u64,
    ));

    syscall_check!(
        input_media_type.SetUINT64(&MF_MT_FRAME_RATE_RANGE_MAX, ((fps as u64) << 32) | 1u64)
    );

    syscall_check!(input_media_type.SetUINT64(&MF_MT_PIXEL_ASPECT_RATIO, (1u64 << 32) | 1u64));

    let output_media_type = syscall_check!(MFCreateMediaType());

    syscall_check!(output_media_type.SetGUID(&MF_MT_MAJOR_TYPE, &MFMediaType_Video));

    syscall_check!(output_media_type.SetGUID(&MF_MT_SUBTYPE, &MFVideoFormat_H264));

    syscall_check!(
        output_media_type.SetUINT32(&MF_MT_INTERLACE_MODE, MFVideoInterlace_Progressive.0 as u32)
    );

    syscall_check!(output_media_type.SetUINT64(
        &MF_MT_FRAME_SIZE,
        ((frame_width as u64) << 32) | frame_height as u64,
    ));

    syscall_check!(
        output_media_type.SetUINT64(&MF_MT_FRAME_RATE_RANGE_MAX, ((fps as u64) << 32) | 1u64)
    );

    syscall_check!(output_media_type.SetUINT64(&MF_MT_PIXEL_ASPECT_RATIO, (1u64 << 32) | 1u64));

    Ok((input_media_type, output_media_type))
}

#[test]
fn test_media_foundation_video_encoder() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    unsafe {
        let version = MF_SDK_VERSION << 16 | MF_API_VERSION;
        if let Err(err) = MFStartup(version, MFSTARTUP_NOSOCKET) {
            panic!("{}", err);
        } else {
            tracing::info!("MFStartup Ok");
        }

        defer! {
            let _ = MFShutdown();
        }

        let descriptors = enum_descriptors()?;
        if descriptors.len() == 0 {
            return Err(anyhow::anyhow!("descriptors is empty"));
        }

        VideoEncoder::new(1920, 1080, 60, &descriptors[0])?;

        Ok(())
    }
}
