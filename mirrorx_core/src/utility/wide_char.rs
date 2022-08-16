use std::{
    ffi::{OsStr, OsString},
    os::windows::prelude::{OsStrExt, OsStringExt},
};

pub trait ToWide {
    fn to_wide(&self) -> Vec<u16>;
    fn to_wide_null(&self) -> Vec<u16>;
}
impl<T> ToWide for T
where
    T: AsRef<OsStr>,
{
    fn to_wide(&self) -> Vec<u16> {
        self.as_ref().encode_wide().collect()
    }
    fn to_wide_null(&self) -> Vec<u16> {
        self.as_ref().encode_wide().chain(Some(0)).collect()
    }
}

pub trait FromWide {
    fn from_wide_null(wide: &[u16]) -> Self;
    unsafe fn from_wide_ptr(wide: *mut u16) -> Self;
}

impl FromWide for OsString {
    fn from_wide_null(wide: &[u16]) -> OsString {
        let len = wide.iter().take_while(|&&c| c != 0).count();
        OsString::from_wide(&wide[..len])
    }

    unsafe fn from_wide_ptr(wide_ptr: *mut u16) -> Self {
        let mut pw_str_end = wide_ptr;
        while *pw_str_end != 0 {
            pw_str_end = pw_str_end.add(1);
        }

        let s = std::slice::from_raw_parts(wide_ptr, pw_str_end.offset_from(wide_ptr) as _);
        OsString::from_wide(s)
    }
}
