use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_logger_init(port_: i64) {
    wire_logger_init_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_config_read(
    port_: i64,
    path: *mut wire_uint_8_list,
    key: *mut wire_uint_8_list,
) {
    wire_config_read_impl(port_, path, key)
}

#[no_mangle]
pub extern "C" fn wire_config_save(
    port_: i64,
    path: *mut wire_uint_8_list,
    key: *mut wire_uint_8_list,
    properties: *mut wire_ConfigProperties,
) {
    wire_config_save_impl(port_, path, key, properties)
}

#[no_mangle]
pub extern "C" fn wire_signaling_dial(port_: i64, req: *mut wire_DialRequest) {
    wire_signaling_dial_impl(port_, req)
}

#[no_mangle]
pub extern "C" fn wire_signaling_register(port_: i64, req: *mut wire_RegisterRequest) {
    wire_signaling_register_impl(port_, req)
}

#[no_mangle]
pub extern "C" fn wire_signaling_subscribe(port_: i64, req: *mut wire_SubscribeRequest) {
    wire_signaling_subscribe_impl(port_, req)
}

#[no_mangle]
pub extern "C" fn wire_signaling_heartbeat(port_: i64, req: *mut wire_HeartbeatRequest) {
    wire_signaling_heartbeat_impl(port_, req)
}

#[no_mangle]
pub extern "C" fn wire_signaling_visit(port_: i64, req: *mut wire_VisitRequest) {
    wire_signaling_visit_impl(port_, req)
}

#[no_mangle]
pub extern "C" fn wire_signaling_key_exchange(port_: i64, req: *mut wire_KeyExchangeRequest) {
    wire_signaling_key_exchange_impl(port_, req)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_box_autoadd_config_properties_0() -> *mut wire_ConfigProperties {
    support::new_leak_box_ptr(wire_ConfigProperties::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_dial_request_0() -> *mut wire_DialRequest {
    support::new_leak_box_ptr(wire_DialRequest::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_heartbeat_request_0() -> *mut wire_HeartbeatRequest {
    support::new_leak_box_ptr(wire_HeartbeatRequest::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_key_exchange_request_0() -> *mut wire_KeyExchangeRequest {
    support::new_leak_box_ptr(wire_KeyExchangeRequest::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_register_request_0() -> *mut wire_RegisterRequest {
    support::new_leak_box_ptr(wire_RegisterRequest::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_subscribe_request_0() -> *mut wire_SubscribeRequest {
    support::new_leak_box_ptr(wire_SubscribeRequest::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_visit_request_0() -> *mut wire_VisitRequest {
    support::new_leak_box_ptr(wire_VisitRequest::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_uint_8_list_0(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: impl Wire2Api

impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}
impl Wire2Api<ConfigProperties> for *mut wire_ConfigProperties {
    fn wire2api(self) -> ConfigProperties {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<ConfigProperties>::wire2api(*wrap).into()
    }
}
impl Wire2Api<DialRequest> for *mut wire_DialRequest {
    fn wire2api(self) -> DialRequest {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<DialRequest>::wire2api(*wrap).into()
    }
}
impl Wire2Api<HeartbeatRequest> for *mut wire_HeartbeatRequest {
    fn wire2api(self) -> HeartbeatRequest {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<HeartbeatRequest>::wire2api(*wrap).into()
    }
}
impl Wire2Api<KeyExchangeRequest> for *mut wire_KeyExchangeRequest {
    fn wire2api(self) -> KeyExchangeRequest {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<KeyExchangeRequest>::wire2api(*wrap).into()
    }
}
impl Wire2Api<RegisterRequest> for *mut wire_RegisterRequest {
    fn wire2api(self) -> RegisterRequest {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<RegisterRequest>::wire2api(*wrap).into()
    }
}
impl Wire2Api<SubscribeRequest> for *mut wire_SubscribeRequest {
    fn wire2api(self) -> SubscribeRequest {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<SubscribeRequest>::wire2api(*wrap).into()
    }
}
impl Wire2Api<VisitRequest> for *mut wire_VisitRequest {
    fn wire2api(self) -> VisitRequest {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<VisitRequest>::wire2api(*wrap).into()
    }
}
impl Wire2Api<ConfigProperties> for wire_ConfigProperties {
    fn wire2api(self) -> ConfigProperties {
        ConfigProperties {
            device_id: self.device_id.wire2api(),
            device_finger_print: self.device_finger_print.wire2api(),
            device_password: self.device_password.wire2api(),
        }
    }
}
impl Wire2Api<DialRequest> for wire_DialRequest {
    fn wire2api(self) -> DialRequest {
        DialRequest {
            uri: self.uri.wire2api(),
        }
    }
}
impl Wire2Api<HeartbeatRequest> for wire_HeartbeatRequest {
    fn wire2api(self) -> HeartbeatRequest {
        HeartbeatRequest {
            local_device_id: self.local_device_id.wire2api(),
            timestamp: self.timestamp.wire2api(),
        }
    }
}

impl Wire2Api<KeyExchangeRequest> for wire_KeyExchangeRequest {
    fn wire2api(self) -> KeyExchangeRequest {
        KeyExchangeRequest {
            local_device_id: self.local_device_id.wire2api(),
            remote_device_id: self.remote_device_id.wire2api(),
            password: self.password.wire2api(),
        }
    }
}

impl Wire2Api<RegisterRequest> for wire_RegisterRequest {
    fn wire2api(self) -> RegisterRequest {
        RegisterRequest {
            local_device_id: self.local_device_id.wire2api(),
            device_finger_print: self.device_finger_print.wire2api(),
        }
    }
}

impl Wire2Api<SubscribeRequest> for wire_SubscribeRequest {
    fn wire2api(self) -> SubscribeRequest {
        SubscribeRequest {
            local_device_id: self.local_device_id.wire2api(),
            device_finger_print: self.device_finger_print.wire2api(),
            config_path: self.config_path.wire2api(),
        }
    }
}

impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<VisitRequest> for wire_VisitRequest {
    fn wire2api(self) -> VisitRequest {
        VisitRequest {
            local_device_id: self.local_device_id.wire2api(),
            remote_device_id: self.remote_device_id.wire2api(),
            resource_type: self.resource_type.wire2api(),
        }
    }
}
// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_ConfigProperties {
    device_id: *mut wire_uint_8_list,
    device_finger_print: *mut wire_uint_8_list,
    device_password: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_DialRequest {
    uri: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_HeartbeatRequest {
    local_device_id: *mut wire_uint_8_list,
    timestamp: u32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_KeyExchangeRequest {
    local_device_id: *mut wire_uint_8_list,
    remote_device_id: *mut wire_uint_8_list,
    password: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RegisterRequest {
    local_device_id: *mut wire_uint_8_list,
    device_finger_print: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_SubscribeRequest {
    local_device_id: *mut wire_uint_8_list,
    device_finger_print: *mut wire_uint_8_list,
    config_path: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_VisitRequest {
    local_device_id: *mut wire_uint_8_list,
    remote_device_id: *mut wire_uint_8_list,
    resource_type: i32,
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

impl NewWithNullPtr for wire_ConfigProperties {
    fn new_with_null_ptr() -> Self {
        Self {
            device_id: core::ptr::null_mut(),
            device_finger_print: core::ptr::null_mut(),
            device_password: core::ptr::null_mut(),
        }
    }
}

impl NewWithNullPtr for wire_DialRequest {
    fn new_with_null_ptr() -> Self {
        Self {
            uri: core::ptr::null_mut(),
        }
    }
}

impl NewWithNullPtr for wire_HeartbeatRequest {
    fn new_with_null_ptr() -> Self {
        Self {
            local_device_id: core::ptr::null_mut(),
            timestamp: Default::default(),
        }
    }
}

impl NewWithNullPtr for wire_KeyExchangeRequest {
    fn new_with_null_ptr() -> Self {
        Self {
            local_device_id: core::ptr::null_mut(),
            remote_device_id: core::ptr::null_mut(),
            password: core::ptr::null_mut(),
        }
    }
}

impl NewWithNullPtr for wire_RegisterRequest {
    fn new_with_null_ptr() -> Self {
        Self {
            local_device_id: core::ptr::null_mut(),
            device_finger_print: core::ptr::null_mut(),
        }
    }
}

impl NewWithNullPtr for wire_SubscribeRequest {
    fn new_with_null_ptr() -> Self {
        Self {
            local_device_id: core::ptr::null_mut(),
            device_finger_print: core::ptr::null_mut(),
            config_path: core::ptr::null_mut(),
        }
    }
}

impl NewWithNullPtr for wire_VisitRequest {
    fn new_with_null_ptr() -> Self {
        Self {
            local_device_id: core::ptr::null_mut(),
            remote_device_id: core::ptr::null_mut(),
            resource_type: Default::default(),
        }
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturnStruct(val: support::WireSyncReturnStruct) {
    unsafe {
        let _ = support::vec_from_leak_ptr(val.ptr, val.len);
    }
}
