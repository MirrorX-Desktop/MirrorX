use crate::{error::MirrorXError, utility::wide_char::FromWide};
use scopeguard::defer;
use std::{ffi::OsString, os::raw::c_void};
use windows::{
    core::GUID,
    Win32::{
        Media::MediaFoundation::*,
        System::{
            Com::{StructuredStorage::*, *},
            Ole::*,
        },
    },
};

pub fn log_media_type(media_type: &IMFMediaType) -> Result<(), MirrorXError> {
    unsafe {
        let count = syscall_check!(media_type.GetCount());

        if count == 0 {
            tracing::info!("empty media type");
            return Ok(());
        }

        for i in 0..count {
            log_media_type_attribute(media_type, i)?;
        }

        Ok(())
    }
}

unsafe fn log_media_type_attribute(
    media_type: &IMFMediaType,
    index: u32,
) -> Result<(), MirrorXError> {
    let mut guid: GUID = std::mem::zeroed();
    let mut var: PROPVARIANT = std::mem::zeroed();

    syscall_check!(media_type.GetItemByIndex(index, &mut guid, &mut var));
    defer! {
        let _ = PropVariantClear(&var as *const _ as  *mut _);
    }

    let attribute_name = get_guid_name(&guid)?;

    match guid {
        MF_MT_FRAME_RATE
        | MF_MT_FRAME_RATE_RANGE_MAX
        | MF_MT_FRAME_RATE_RANGE_MIN
        | MF_MT_FRAME_SIZE
        | MF_MT_PIXEL_ASPECT_RATIO => {
            let high = (var.Anonymous.Anonymous.Anonymous.uhVal >> 32) as u32;
            let low = (var.Anonymous.Anonymous.Anonymous.uhVal & 0xFFFFFFFF) as u32;
            log_attribute(&attribute_name, &format!("{} x {}", high, low));
        }
        MF_MT_GEOMETRIC_APERTURE | MF_MT_MINIMUM_DISPLAY_APERTURE | MF_MT_PAN_SCAN_APERTURE => {
            if (var.Anonymous.Anonymous.Anonymous.caub.cElems as usize)
                < std::mem::size_of::<MFVideoArea>()
            {
                log_attribute(&attribute_name, "unknown");
            } else {
                let area: *mut MFVideoArea =
                    std::mem::transmute(var.Anonymous.Anonymous.Anonymous.caub.pElems);

                log_attribute(
                    &attribute_name,
                    &format!(
                        "({}, {}), ({}, {})",
                        offset_to_float(&(*area).OffsetX),
                        offset_to_float(&(*area).OffsetY),
                        (*area).Area.cx,
                        (*area).Area.cy
                    ),
                );
            }
        }
        _ => match VARENUM(var.Anonymous.Anonymous.vt.into()) {
            VT_UI4 => log_attribute(
                &attribute_name,
                var.Anonymous.Anonymous.Anonymous.ulVal.to_string().as_str(),
            ),
            VT_UI8 => log_attribute(
                &attribute_name,
                var.Anonymous.Anonymous.Anonymous.uhVal.to_string().as_str(),
            ),
            VT_R8 => log_attribute(
                &attribute_name,
                var.Anonymous
                    .Anonymous
                    .Anonymous
                    .dblVal
                    .to_string()
                    .as_str(),
            ),
            VT_CLSID => {
                let attribute_value = get_guid_name(&*var.Anonymous.Anonymous.Anonymous.puuid)?;
                log_attribute(&attribute_name, &attribute_value);
            }
            VT_LPWSTR => {
                let attribute_value =
                    OsString::from_wide_ptr(var.Anonymous.Anonymous.Anonymous.pwszVal.0)
                        .into_string()
                        .map_err(|err| {
                            MirrorXError::Other(anyhow::anyhow!(
                                "convert OsString into String failed ({:?})",
                                err
                            ))
                        })?;

                log_attribute(&attribute_name, &attribute_value);
            }
            VT_VECTOR | VT_UI1 => log_attribute(&attribute_name, "<<bytes array>>"),
            VT_UNKNOWN => log_attribute(&attribute_name, "IUnknown"),
            _ => log_attribute(
                &attribute_name,
                &format!("unexpected vt={}", var.Anonymous.Anonymous.vt),
            ),
        },
    }

    Ok(())
}

fn log_attribute(name: &str, value: &str) {
    tracing::info!(name, value, "media type attribute");
}

fn offset_to_float(offset: &MFOffset) -> f32 {
    offset.value as f32 + offset.fract as f32 / 65536.0f32
}

fn get_guid_name(guid: &GUID) -> Result<String, MirrorXError> {
    let mut name = String::from(match *guid {
        MF_MT_MAJOR_TYPE => "MF_MT_MAJOR_TYPE",
        MF_MT_SUBTYPE => "MF_MT_SUBTYPE",
        MF_MT_ALL_SAMPLES_INDEPENDENT => "MF_MT_ALL_SAMPLES_INDEPENDENT",
        MF_MT_FIXED_SIZE_SAMPLES => "MF_MT_FIXED_SIZE_SAMPLES",
        MF_MT_COMPRESSED => "MF_MT_COMPRESSED",
        MF_MT_SAMPLE_SIZE => "MF_MT_SAMPLE_SIZE",
        MF_MT_WRAPPED_TYPE => "MF_MT_WRAPPED_TYPE",
        MF_MT_AUDIO_NUM_CHANNELS => "MF_MT_AUDIO_NUM_CHANNELS",
        MF_MT_AUDIO_SAMPLES_PER_SECOND => "MF_MT_AUDIO_SAMPLES_PER_SECOND",
        MF_MT_AUDIO_FLOAT_SAMPLES_PER_SECOND => "MF_MT_AUDIO_FLOAT_SAMPLES_PER_SECOND",
        MF_MT_AUDIO_AVG_BYTES_PER_SECOND => "MF_MT_AUDIO_AVG_BYTES_PER_SECOND",
        MF_MT_AUDIO_BLOCK_ALIGNMENT => "MF_MT_AUDIO_BLOCK_ALIGNMENT",
        MF_MT_AUDIO_BITS_PER_SAMPLE => "MF_MT_AUDIO_BITS_PER_SAMPLE",
        MF_MT_AUDIO_VALID_BITS_PER_SAMPLE => "MF_MT_AUDIO_VALID_BITS_PER_SAMPLE",
        MF_MT_AUDIO_SAMPLES_PER_BLOCK => "MF_MT_AUDIO_SAMPLES_PER_BLOCK",
        MF_MT_AUDIO_CHANNEL_MASK => "MF_MT_AUDIO_CHANNEL_MASK",
        MF_MT_AUDIO_FOLDDOWN_MATRIX => "MF_MT_AUDIO_FOLDDOWN_MATRIX",
        MF_MT_AUDIO_WMADRC_PEAKREF => "MF_MT_AUDIO_WMADRC_PEAKREF",
        MF_MT_AUDIO_WMADRC_PEAKTARGET => "MF_MT_AUDIO_WMADRC_PEAKTARGET",
        MF_MT_AUDIO_WMADRC_AVGREF => "MF_MT_AUDIO_WMADRC_AVGREF",
        MF_MT_AUDIO_WMADRC_AVGTARGET => "MF_MT_AUDIO_WMADRC_AVGTARGET",
        MF_MT_AUDIO_PREFER_WAVEFORMATEX => "MF_MT_AUDIO_PREFER_WAVEFORMATEX",
        MF_MT_AAC_PAYLOAD_TYPE => "MF_MT_AAC_PAYLOAD_TYPE",
        MF_MT_AAC_AUDIO_PROFILE_LEVEL_INDICATION => "MF_MT_AAC_AUDIO_PROFILE_LEVEL_INDICATION",
        MF_MT_FRAME_SIZE => "MF_MT_FRAME_SIZE",
        MF_MT_FRAME_RATE => "MF_MT_FRAME_RATE",
        MF_MT_FRAME_RATE_RANGE_MAX => "MF_MT_FRAME_RATE_RANGE_MAX",
        MF_MT_FRAME_RATE_RANGE_MIN => "MF_MT_FRAME_RATE_RANGE_MIN",
        MF_MT_PIXEL_ASPECT_RATIO => "MF_MT_PIXEL_ASPECT_RATIO",
        MF_MT_DRM_FLAGS => "MF_MT_DRM_FLAGS",
        MF_MT_PAD_CONTROL_FLAGS => "MF_MT_PAD_CONTROL_FLAGS",
        MF_MT_SOURCE_CONTENT_HINT => "MF_MT_SOURCE_CONTENT_HINT",
        MF_MT_VIDEO_CHROMA_SITING => "MF_MT_VIDEO_CHROMA_SITING",
        MF_MT_INTERLACE_MODE => "MF_MT_INTERLACE_MODE",
        MF_MT_TRANSFER_FUNCTION => "MF_MT_TRANSFER_FUNCTION",
        MF_MT_VIDEO_PRIMARIES => "MF_MT_VIDEO_PRIMARIES",
        MF_MT_CUSTOM_VIDEO_PRIMARIES => "MF_MT_CUSTOM_VIDEO_PRIMARIES",
        MF_MT_YUV_MATRIX => "MF_MT_YUV_MATRIX",
        MF_MT_VIDEO_LIGHTING => "MF_MT_VIDEO_LIGHTING",
        MF_MT_VIDEO_NOMINAL_RANGE => "MF_MT_VIDEO_NOMINAL_RANGE",
        MF_MT_GEOMETRIC_APERTURE => "MF_MT_GEOMETRIC_APERTURE",
        MF_MT_MINIMUM_DISPLAY_APERTURE => "MF_MT_MINIMUM_DISPLAY_APERTURE",
        MF_MT_PAN_SCAN_APERTURE => "MF_MT_PAN_SCAN_APERTURE",
        MF_MT_PAN_SCAN_ENABLED => "MF_MT_PAN_SCAN_ENABLED",
        MF_MT_AVG_BITRATE => "MF_MT_AVG_BITRATE",
        MF_MT_AVG_BIT_ERROR_RATE => "MF_MT_AVG_BIT_ERROR_RATE",
        MF_MT_MAX_KEYFRAME_SPACING => "MF_MT_MAX_KEYFRAME_SPACING",
        MF_MT_DEFAULT_STRIDE => "MF_MT_DEFAULT_STRIDE",
        MF_MT_PALETTE => "MF_MT_PALETTE",
        MF_MT_USER_DATA => "MF_MT_USER_DATA",
        MF_MT_AM_FORMAT_TYPE => "MF_MT_AM_FORMAT_TYPE",
        MF_MT_MPEG_START_TIME_CODE => "MF_MT_MPEG_START_TIME_CODE",
        MF_MT_MPEG2_PROFILE => "MF_MT_MPEG2_PROFILE",
        MF_MT_MPEG2_LEVEL => "MF_MT_MPEG2_LEVEL",
        MF_MT_MPEG2_FLAGS => "MF_MT_MPEG2_FLAGS",
        MF_MT_MPEG_SEQUENCE_HEADER => "MF_MT_MPEG_SEQUENCE_HEADER",
        MF_MT_DV_AAUX_SRC_PACK_0 => "MF_MT_DV_AAUX_SRC_PACK_0",
        MF_MT_DV_AAUX_CTRL_PACK_0 => "MF_MT_DV_AAUX_CTRL_PACK_0",
        MF_MT_DV_AAUX_SRC_PACK_1 => "MF_MT_DV_AAUX_SRC_PACK_1",
        MF_MT_DV_AAUX_CTRL_PACK_1 => "MF_MT_DV_AAUX_CTRL_PACK_1",
        MF_MT_DV_VAUX_SRC_PACK => "MF_MT_DV_VAUX_SRC_PACK",
        MF_MT_DV_VAUX_CTRL_PACK => "MF_MT_DV_VAUX_CTRL_PACK",
        MF_MT_ARBITRARY_HEADER => "MF_MT_ARBITRARY_HEADER",
        MF_MT_ARBITRARY_FORMAT => "MF_MT_ARBITRARY_FORMAT",
        MF_MT_IMAGE_LOSS_TOLERANT => "MF_MT_IMAGE_LOSS_TOLERANT",
        MF_MT_MPEG4_SAMPLE_DESCRIPTION => "MF_MT_MPEG4_SAMPLE_DESCRIPTION",
        MF_MT_MPEG4_CURRENT_SAMPLE_ENTRY => "MF_MT_MPEG4_CURRENT_SAMPLE_ENTRY",
        MF_MT_ORIGINAL_4CC => "MF_MT_ORIGINAL_4CC",
        MF_MT_ORIGINAL_WAVE_FORMAT_TAG => "MF_MT_ORIGINAL_WAVE_FORMAT_TAG",

        MFMediaType_Audio => "MFMediaType_Audio",
        MFMediaType_Video => "MFMediaType_Video",
        MFMediaType_Protected => "MFMediaType_Protected",
        MFMediaType_SAMI => "MFMediaType_SAMI",
        MFMediaType_Script => "MFMediaType_Script",
        MFMediaType_Image => "MFMediaType_Image",
        MFMediaType_HTML => "MFMediaType_HTML",
        MFMediaType_Binary => "MFMediaType_Binary",
        MFMediaType_FileTransfer => "MFMediaType_FileTransfer",

        MFVideoFormat_AI44 => "MFVideoFormat_AI44",
        MFVideoFormat_ARGB32 => "MFVideoFormat_ARGB32",
        MFVideoFormat_AYUV => "MFVideoFormat_AYUV",
        MFVideoFormat_DV25 => "MFVideoFormat_DV25",
        MFVideoFormat_DV50 => "MFVideoFormat_DV50",
        MFVideoFormat_DVH1 => "MFVideoFormat_DVH1",
        MFVideoFormat_DVSD => "MFVideoFormat_DVSD",
        MFVideoFormat_DVSL => "MFVideoFormat_DVSL",
        MFVideoFormat_H264 => "MFVideoFormat_H264",
        MFVideoFormat_I420 => "MFVideoFormat_I420",
        MFVideoFormat_IYUV => "MFVideoFormat_IYUV",
        MFVideoFormat_M4S2 => "MFVideoFormat_M4S2",
        MFVideoFormat_MJPG => "MFVideoFormat_MJPG",
        MFVideoFormat_MP43 => "MFVideoFormat_MP43",
        MFVideoFormat_MP4S => "MFVideoFormat_MP4S",
        MFVideoFormat_MP4V => "MFVideoFormat_MP4V",
        MFVideoFormat_MPG1 => "MFVideoFormat_MPG1",
        MFVideoFormat_MSS1 => "MFVideoFormat_MSS1",
        MFVideoFormat_MSS2 => "MFVideoFormat_MSS2",
        MFVideoFormat_NV11 => "MFVideoFormat_NV11",
        MFVideoFormat_NV12 => "MFVideoFormat_NV12",
        MFVideoFormat_P010 => "MFVideoFormat_P010",
        MFVideoFormat_P016 => "MFVideoFormat_P016",
        MFVideoFormat_P210 => "MFVideoFormat_P210",
        MFVideoFormat_P216 => "MFVideoFormat_P216",
        MFVideoFormat_RGB24 => "MFVideoFormat_RGB24",
        MFVideoFormat_RGB32 => "MFVideoFormat_RGB32",
        MFVideoFormat_RGB555 => "MFVideoFormat_RGB555",
        MFVideoFormat_RGB565 => "MFVideoFormat_RGB565",
        MFVideoFormat_RGB8 => "MFVideoFormat_RGB8",
        MFVideoFormat_UYVY => "MFVideoFormat_UYVY",
        MFVideoFormat_v210 => "MFVideoFormat_v210",
        MFVideoFormat_v410 => "MFVideoFormat_v410",
        MFVideoFormat_WMV1 => "MFVideoFormat_WMV1",
        MFVideoFormat_WMV2 => "MFVideoFormat_WMV2",
        MFVideoFormat_WMV3 => "MFVideoFormat_WMV3",
        MFVideoFormat_WVC1 => "MFVideoFormat_WVC1",
        MFVideoFormat_Y210 => "MFVideoFormat_Y210",
        MFVideoFormat_Y216 => "MFVideoFormat_Y216",
        MFVideoFormat_Y410 => "MFVideoFormat_Y410",
        MFVideoFormat_Y416 => "MFVideoFormat_Y416",
        MFVideoFormat_Y41P => "MFVideoFormat_Y41P",
        MFVideoFormat_Y41T => "MFVideoFormat_Y41T",
        MFVideoFormat_YUY2 => "MFVideoFormat_YUY2",
        MFVideoFormat_YV12 => "MFVideoFormat_YV12",
        MFVideoFormat_YVYU => "MFVideoFormat_YVYU",

        MFAudioFormat_PCM => "MFAudioFormat_PCM",
        MFAudioFormat_Float => "MFAudioFormat_Float",
        MFAudioFormat_DTS => "MFAudioFormat_DTS",
        MFAudioFormat_Dolby_AC3_SPDIF => "MFAudioFormat_Dolby_AC3_SPDIF",
        MFAudioFormat_DRM => "MFAudioFormat_DRM",
        MFAudioFormat_WMAudioV8 => "MFAudioFormat_WMAudioV8",
        MFAudioFormat_WMAudioV9 => "MFAudioFormat_WMAudioV9",
        MFAudioFormat_WMAudio_Lossless => "MFAudioFormat_WMAudio_Lossless",
        MFAudioFormat_WMASPDIF => "MFAudioFormat_WMASPDIF",
        MFAudioFormat_MSP1 => "MFAudioFormat_MSP1",
        MFAudioFormat_MP3 => "MFAudioFormat_MP3",
        MFAudioFormat_MPEG => "MFAudioFormat_MPEG",
        MFAudioFormat_AAC => "MFAudioFormat_AAC",
        MFAudioFormat_ADTS => "MFAudioFormat_ADTS",
        _ => "",
    });

    if name == "" {
        unsafe {
            let pw_str = syscall_check!(StringFromCLSID(guid));
            defer! {
                CoTaskMemFree(pw_str.0 as *const c_void);
            }

            let value = OsString::from_wide_ptr(pw_str.0)
                .into_string()
                .map_err(|err| {
                    MirrorXError::Other(anyhow::anyhow!(
                        "convert OsString into String failed ({:?})",
                        err
                    ))
                })?;

            value.clone_into(&mut name);
        }
    }

    Ok(name)
}
