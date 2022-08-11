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

pub unsafe fn startup() -> Result<(), MirrorXError> {
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

    let input_info = MFT_REGISTER_TYPE_INFO {
        guidMajorType: MFMediaType_Video,
        guidSubtype: MFVideoFormat_NV12,
    };
    let output_info = MFT_REGISTER_TYPE_INFO {
        guidMajorType: MFMediaType_Video,
        guidSubtype: MFVideoFormat_H264,
    };

    let mut clsids = std::ptr::null_mut();
    let mut count = 0;

    MFTEnum(
        MFT_CATEGORY_VIDEO_ENCODER,
        MFT_ENUM_FLAG_HARDWARE.0 | MFT_ENUM_FLAG_SORTANDFILTER.0,
        &input_info,
        &output_info,
        None,
        &mut clsids,
        &mut count,
    )
    .map_err(|err| MirrorXError::Native {
        name: "MFTEnum",
        code: err.code().0,
        additional: None,
    })?;

    let it: IMFTransform =
        CoCreateInstance(clsids, None, CLSCTX_ALL).map_err(|err| MirrorXError::Native {
            name: "CoCreateInstance",
            code: err.code().0,
            additional: None,
        })?;

    let codec_api: ICodecAPI = it.cast().map_err(|err| MirrorXError::Native {
        name: "IMFTransform.cast",
        code: err.code().0,
        additional: None,
    })?;

    let val: VARIANT = VARIANT {
        Anonymous: VARIANT_0 {
            Anonymous: ManuallyDrop::new(VARIANT_0_0 {
                vt: VT_UI4.0 as u16,
                Anonymous: VARIANT_0_0_0 {
                    ulVal: eAVEncCommonRateControlMode_GlobalLowDelayVBR.0 as u32,
                },
                ..Default::default()
            }),
        },
    };

    defer! {
        drop(&val.Anonymous.Anonymous);
    }

    codec_api
        .SetValue(&CODECAPI_AVEncCommonMaxBitRate, &val)
        .map_err(|err| MirrorXError::Native {
            name: "ICodecAPI->SetValue",
            code: err.code().0,
            additional:Some(String::from("key: CODECAPI_AVEncCommonMaxBitRate, value: eAVEncCommonRateControlMode_GlobalLowDelayVBR"))
        })?;

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

    codec_api
        .SetValue(&CODECAPI_AVLowLatencyMode, &val)
        .map_err(|err| MirrorXError::Native {
            name: "ICodecAPI->SetValue",
            code: err.code().0,
            additional: Some(String::from(
                "key: CODECAPI_AVLowLatencyMode, value: VARIANT_TRUE",
            )),
        })?;

    Ok(())
}

#[test]
fn test_startup() {
    tracing_subscriber::fmt::init();

    unsafe {
        if let Err(err) = startup() {
            panic!("{}", err)
        }
    }
}
