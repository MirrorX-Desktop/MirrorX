use crate::error::MirrorXError;
use scopeguard::defer;
use std::{mem::ManuallyDrop, str::FromStr};
use windows::{
    core::Interface,
    Win32::{
        Media::MediaFoundation::*,
        System::{
            Com::*,
            Ole::{VT_BOOL, VT_UI4},
        },
    },
};

pub unsafe fn startup(width: u16, height: u16, fps: u8) -> Result<(), MirrorXError> {
    // CoInitializeEx(std::ptr::null(), COINIT_APARTMENTTHREADED).map_err(|err| {
    //     MirrorXError::Native {
    //         name: "CoInitializeEx",
    //         code: err.code().0,
    //     }
    // })?;

    let version = MF_SDK_VERSION << 16 | MF_API_VERSION;
    if let Err(err) = MFStartup(version, MFSTARTUP_NOSOCKET) {
        panic!("{}", err);
    } else {
        print!("MFStartup Ok");
    }

    defer! {
        MFShutdown();
    }

    let input_media_type = MFCreateMediaType()?;
    input_media_type.SetGUID(&MF_MT_MAJOR_TYPE, &MFMediaType_Video)?;
    input_media_type.SetGUID(&MF_MT_SUBTYPE, &MFVideoFormat_NV12)?;
    input_media_type.SetUINT32(&MF_MT_INTERLACE_MODE, MFVideoInterlace_Progressive.0 as u32)?;
    input_media_type.SetUINT64(&MF_MT_FRAME_SIZE, ((width as u64) << 32) | height as u64)?;
    input_media_type.SetUINT64(&MF_MT_FRAME_RATE_RANGE_MAX, ((fps as u64) << 32) | 1u64)?;
    input_media_type.SetUINT64(&MF_MT_PIXEL_ASPECT_RATIO, (1u64 << 32) | 1u64)?;

    let output_media_type = MFCreateMediaType()?;
    output_media_type.SetGUID(&MF_MT_MAJOR_TYPE, &MFMediaType_Video)?;
    output_media_type.SetGUID(&MF_MT_SUBTYPE, &MFVideoFormat_H264)?;
    output_media_type.SetUINT32(&MF_MT_INTERLACE_MODE, MFVideoInterlace_Progressive.0 as u32)?;
    output_media_type.SetUINT64(&MF_MT_FRAME_SIZE, ((width as u64) << 32) | height as u64)?;
    output_media_type.SetUINT64(&MF_MT_FRAME_RATE_RANGE_MAX, ((fps as u64) << 32) | 1u64)?;
    output_media_type.SetUINT64(&MF_MT_PIXEL_ASPECT_RATIO, (1u64 << 32) | 1u64)?;
    output_media_type.SetUINT32(&MF_LOW_LATENCY, 1)?;

    let mut active = std::ptr::null_mut();
    let mut count = 0;

    let info = MFT_REGISTER_TYPE_INFO {
        guidMajorType: MFMediaType_Video,
        guidSubtype: MFVideoFormat_H264,
    };

    let mut flags = MFT_ENUM_FLAG::default();
    flags |= MFT_ENUM_FLAG_HARDWARE;
    flags |= MFT_ENUM_FLAG_ASYNCMFT;
    flags |= MFT_ENUM_FLAG_SORTANDFILTER;

    MFTEnumEx(
        MFT_CATEGORY_VIDEO_ENCODER,
        flags,
        std::ptr::null(),
        &info,
        &mut active,
        &mut count,
    )?;

    if count == 0 {
        tracing::info!("no enumed mft");
        return Ok(());
    }

    // let it: IMFTransform =
    //     CoCreateInstance(clsids, None, CLSCTX_ALL).map_err(|err| MirrorXError::Native {
    //         name: "CoCreateInstance",
    //         code: err.code().0,
    //         additional: None,
    //     })?;

    // let codec_api: ICodecAPI = it.cast().map_err(|err| MirrorXError::Native {
    //     name: "IMFTransform.cast",
    //     code: err.code().0,
    //     additional: None,
    // })?;

    // let val: VARIANT = VARIANT {
    //     Anonymous: VARIANT_0 {
    //         Anonymous: ManuallyDrop::new(VARIANT_0_0 {
    //             vt: VT_UI4.0 as u16,
    //             Anonymous: VARIANT_0_0_0 {
    //                 ulVal: eAVEncCommonRateControlMode_GlobalLowDelayVBR.0 as u32,
    //             },
    //             ..Default::default()
    //         }),
    //     },
    // };

    // defer! {
    //     drop(&val.Anonymous.Anonymous);
    // }

    // codec_api
    //     .SetValue(&CODECAPI_AVEncCommonMaxBitRate, &val)
    //     .map_err(|err| MirrorXError::Native {
    //         name: "ICodecAPI->SetValue",
    //         code: err.code().0,
    //         additional:Some(String::from("key: CODECAPI_AVEncCommonMaxBitRate, value: eAVEncCommonRateControlMode_GlobalLowDelayVBR"))
    //     })?;

    // let val: VARIANT = VARIANT {
    //     Anonymous: VARIANT_0 {
    //         Anonymous: ManuallyDrop::new(VARIANT_0_0 {
    //             vt: VT_BOOL.0 as u16,
    //             Anonymous: VARIANT_0_0_0 { boolVal: -1 }, // true in variant is -1(0xFFFF) and false is 0
    //             ..Default::default()
    //         }),
    //     },
    // };

    // defer! {
    //     drop(&val.Anonymous.Anonymous);
    // }

    // codec_api
    //     .SetValue(&CODECAPI_AVLowLatencyMode, &val)
    //     .map_err(|err| MirrorXError::Native {
    //         name: "ICodecAPI->SetValue",
    //         code: err.code().0,
    //         additional: Some(String::from(
    //             "key: CODECAPI_AVLowLatencyMode, value: VARIANT_TRUE",
    //         )),
    //     })?;

    Ok(())
}

unsafe fn create_descriptor(activate: IMFActivate) -> Result<(), MirrorXError> {
    let flags = activate.GetUINT32(&MF_TRANSFORM_FLAGS_Attribute)?;

    if flags & MFT_ENUM_FLAG_SYNCMFT.0 != 0 {
        tracing::info!("is sync");
    } else if flags & MFT_ENUM_FLAG_ASYNCMFT.0 != 0 {
        tracing::info!("is async");
    } else {
        tracing::info!("sync unknown");
    }

    if flags & MFT_ENUM_FLAG_HARDWARE.0 != 0 {
        tracing::info!("is hardware");
    } else {
        tracing::info!("is not hardware");
    }

    let guid = activate.GetGUID(&MFT_TRANSFORM_CLSID_Attribute)?;
    tracing::info!("guid: {:?}", guid);

    Ok(())
}

#[test]
fn test_startup() {
    tracing_subscriber::fmt::init();

    unsafe {
        // if let Err(err) = startup() {
        //     panic!("{}", err)
        // }
        let video_color_converter: IMFTransform =
            CoCreateInstance(&CLSID_VideoProcessorMFT, None, CLSCTX_INPROC_SERVER).unwrap();

        video_color_converter.SetInputType(0, IMFMediaType, dwflags);
    }
}

#[allow(non_snake_case)]
unsafe fn MFSetAttributeSize(
    attributes: &IMFAttributes,
    key: &windows::core::GUID,
    width: u32,
    height: u32,
) -> windows::core::Result<()> {
    attributes.SetUINT64(key, ((width as u64) << 32) | height as u64)
}

#[allow(non_snake_case)]
unsafe fn MFSetAttributeRatio(
    attributes: &IMFAttributes,
    key: &windows::core::GUID,
    numerator: u32,
    denominator: u32,
) -> windows::core::Result<()> {
    attributes.SetUINT64(key, ((numerator as u64) << 32) | denominator as u64)
}
