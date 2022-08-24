use crate::{check_if_failed, error::MirrorXError, utility::wide_char::FromWide};
use std::ffi::OsString;
use windows::Win32::{Foundation::MAX_PATH, Media::MediaFoundation::*};

#[derive(Debug, Clone)]
pub struct Descriptor {
    name: String,
    guid: String,
    is_async: bool,
    is_hardware: bool,
}

impl Descriptor {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn guid(&self) -> &str {
        self.guid.as_ref()
    }

    pub fn is_async(&self) -> bool {
        self.is_async
    }

    pub fn is_hardware(&self) -> bool {
        self.is_hardware
    }
}

pub fn enum_descriptors() -> Result<Vec<Descriptor>, MirrorXError> {
    unsafe {
        let mut activates = std::ptr::null_mut();
        let mut count = 0;

        let info = MFT_REGISTER_TYPE_INFO {
            guidMajorType: MFMediaType_Video,
            guidSubtype: MFVideoFormat_H264,
        };

        let mut flags = MFT_ENUM_FLAG::default();
        flags |= MFT_ENUM_FLAG_LOCALMFT;
        flags |= MFT_ENUM_FLAG_HARDWARE;
        flags |= MFT_ENUM_FLAG_TRANSCODE_ONLY;
        flags |= MFT_ENUM_FLAG_SYNCMFT;
        flags |= MFT_ENUM_FLAG_ASYNCMFT;
        flags |= MFT_ENUM_FLAG_SORTANDFILTER;

        check_if_failed!(MFTEnumEx(
            MFT_CATEGORY_VIDEO_ENCODER,
            flags,
            std::ptr::null(),
            &info,
            &mut activates,
            &mut count,
        ));

        if count == 0 {
            return Ok(Vec::new());
        }

        let mut descriptors = Vec::with_capacity(count as usize);
        for i in 0..count {
            if let Some(activate) = &*activates.add(i as usize) {
                let descriptor = create_descriptor(activate)?;
                descriptors.push(descriptor);
            }
        }

        Ok(descriptors)
    }
}

unsafe fn create_descriptor(activate: &IMFActivate) -> Result<Descriptor, MirrorXError> {
    let flags = check_if_failed!(activate.GetUINT32(&MF_TRANSFORM_FLAGS_Attribute));

    let mut is_async = !((flags & MFT_ENUM_FLAG_SYNCMFT.0) != 0);
    is_async |= !!((flags & MFT_ENUM_FLAG_ASYNCMFT.0) != 0);

    let is_hardware = flags & MFT_ENUM_FLAG_HARDWARE.0 != 0;

    let guid = check_if_failed!(activate.GetGUID(&MFT_TRANSFORM_CLSID_Attribute));

    let mut name = [0u16; MAX_PATH as usize];

    check_if_failed!(activate.GetString(
        &MFT_FRIENDLY_NAME_Attribute,
        &mut name,
        std::ptr::null_mut(),
    ));

    let name = OsString::from_wide_null(&name)
        .into_string()
        .map_err(|err| {
            MirrorXError::Other(anyhow::anyhow!(
                "convert OsString into String failed ({:?})",
                err
            ))
        })?;

    Ok(Descriptor {
        name,
        guid: format!("{:?}", guid),
        is_async,
        is_hardware,
    })
}
