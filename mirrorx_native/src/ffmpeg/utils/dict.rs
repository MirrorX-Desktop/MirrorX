use std::os::raw::c_char;

#[repr(C)]
pub struct AVDictionary {
    pub count: isize,
    pub elems: *mut AVDictionaryEntry,
}

#[repr(C)]
pub struct AVDictionaryEntry {
    pub key: *mut c_char,
    pub value: *mut c_char,
}

extern "C" {
    pub fn av_dict_set(
        pm: *mut *mut AVDictionary,
        key: *const c_char,
        value: *const c_char,
        flags: isize,
    ) -> isize;

    pub fn av_dict_free(pm: *mut *mut AVDictionary);
}
