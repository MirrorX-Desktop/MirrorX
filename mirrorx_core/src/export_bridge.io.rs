use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_init_logger(port_: i64) {
    wire_init_logger_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_read_primary_domain(port_: i64, path: *mut wire_uint_8_list) {
    wire_read_primary_domain_impl(port_, path)
}

#[no_mangle]
pub extern "C" fn wire_save_primary_domain(
    port_: i64,
    path: *mut wire_uint_8_list,
    value: *mut wire_uint_8_list,
) {
    wire_save_primary_domain_impl(port_, path, value)
}

#[no_mangle]
pub extern "C" fn wire_read_domain_config(
    port_: i64,
    path: *mut wire_uint_8_list,
    domain: *mut wire_uint_8_list,
) {
    wire_read_domain_config_impl(port_, path, domain)
}

#[no_mangle]
pub extern "C" fn wire_save_domain_config(
    port_: i64,
    path: *mut wire_uint_8_list,
    domain: *mut wire_uint_8_list,
    value: *mut wire_DomainConfig,
) {
    wire_save_domain_config_impl(port_, path, domain, value)
}

#[no_mangle]
pub extern "C" fn wire_signaling_dial(port_: i64, req: *mut wire_DialRequest) {
    wire_signaling_dial_impl(port_, req)
}

#[no_mangle]
pub extern "C" fn wire_signaling_disconnect(port_: i64) {
    wire_signaling_disconnect_impl(port_)
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
pub extern "C" fn wire_signaling_visit_reply(port_: i64, req: *mut wire_VisitReplyRequest) {
    wire_signaling_visit_reply_impl(port_, req)
}

#[no_mangle]
pub extern "C" fn wire_signaling_key_exchange(port_: i64, req: *mut wire_KeyExchangeRequest) {
    wire_signaling_key_exchange_impl(port_, req)
}

#[no_mangle]
pub extern "C" fn wire_endpoint_connect(port_: i64, req: *mut wire_ConnectRequest) {
    wire_endpoint_connect_impl(port_, req)
}

#[no_mangle]
pub extern "C" fn wire_endpoint_handshake(port_: i64, req: *mut wire_HandshakeRequest) {
    wire_endpoint_handshake_impl(port_, req)
}

#[no_mangle]
pub extern "C" fn wire_endpoint_negotiate_visit_desktop_params(
    port_: i64,
    req: *mut wire_NegotiateVisitDesktopParamsRequest,
) {
    wire_endpoint_negotiate_visit_desktop_params_impl(port_, req)
}

#[no_mangle]
pub extern "C" fn wire_endpoint_negotiate_select_monitor(
    port_: i64,
    req: *mut wire_NegotiateSelectMonitorRequest,
) {
    wire_endpoint_negotiate_select_monitor_impl(port_, req)
}

#[no_mangle]
pub extern "C" fn wire_endpoint_negotiate_finished(
    port_: i64,
    req: *mut wire_NegotiateFinishedRequest,
) {
    wire_endpoint_negotiate_finished_impl(port_, req)
}

#[no_mangle]
pub extern "C" fn wire_endpoint_input(port_: i64, req: *mut wire_InputRequest) {
    wire_endpoint_input_impl(port_, req)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_box_autoadd_connect_request_0() -> *mut wire_ConnectRequest {
    support::new_leak_box_ptr(wire_ConnectRequest::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_dial_request_0() -> *mut wire_DialRequest {
    support::new_leak_box_ptr(wire_DialRequest::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_domain_config_0() -> *mut wire_DomainConfig {
    support::new_leak_box_ptr(wire_DomainConfig::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_handshake_request_0() -> *mut wire_HandshakeRequest {
    support::new_leak_box_ptr(wire_HandshakeRequest::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_heartbeat_request_0() -> *mut wire_HeartbeatRequest {
    support::new_leak_box_ptr(wire_HeartbeatRequest::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_i64_0(value: i64) -> *mut i64 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_input_request_0() -> *mut wire_InputRequest {
    support::new_leak_box_ptr(wire_InputRequest::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_key_exchange_request_0() -> *mut wire_KeyExchangeRequest {
    support::new_leak_box_ptr(wire_KeyExchangeRequest::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_keyboard_event_0() -> *mut wire_KeyboardEvent {
    support::new_leak_box_ptr(wire_KeyboardEvent::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_mouse_event_0() -> *mut wire_MouseEvent {
    support::new_leak_box_ptr(wire_MouseEvent::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_negotiate_finished_request_0(
) -> *mut wire_NegotiateFinishedRequest {
    support::new_leak_box_ptr(wire_NegotiateFinishedRequest::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_negotiate_select_monitor_request_0(
) -> *mut wire_NegotiateSelectMonitorRequest {
    support::new_leak_box_ptr(wire_NegotiateSelectMonitorRequest::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_negotiate_visit_desktop_params_request_0(
) -> *mut wire_NegotiateVisitDesktopParamsRequest {
    support::new_leak_box_ptr(wire_NegotiateVisitDesktopParamsRequest::new_with_null_ptr())
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
pub extern "C" fn new_box_autoadd_visit_reply_request_0() -> *mut wire_VisitReplyRequest {
    support::new_leak_box_ptr(wire_VisitReplyRequest::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_visit_request_0() -> *mut wire_VisitRequest {
    support::new_leak_box_ptr(wire_VisitRequest::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_input_event_0() -> *mut wire_InputEvent {
    support::new_leak_box_ptr(wire_InputEvent::new_with_null_ptr())
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

impl Wire2Api<ConnectRequest> for *mut wire_ConnectRequest {
    fn wire2api(self) -> ConnectRequest {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<ConnectRequest>::wire2api(*wrap).into()
    }
}
impl Wire2Api<DialRequest> for *mut wire_DialRequest {
    fn wire2api(self) -> DialRequest {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<DialRequest>::wire2api(*wrap).into()
    }
}
impl Wire2Api<DomainConfig> for *mut wire_DomainConfig {
    fn wire2api(self) -> DomainConfig {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<DomainConfig>::wire2api(*wrap).into()
    }
}
impl Wire2Api<HandshakeRequest> for *mut wire_HandshakeRequest {
    fn wire2api(self) -> HandshakeRequest {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<HandshakeRequest>::wire2api(*wrap).into()
    }
}
impl Wire2Api<HeartbeatRequest> for *mut wire_HeartbeatRequest {
    fn wire2api(self) -> HeartbeatRequest {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<HeartbeatRequest>::wire2api(*wrap).into()
    }
}

impl Wire2Api<InputRequest> for *mut wire_InputRequest {
    fn wire2api(self) -> InputRequest {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<InputRequest>::wire2api(*wrap).into()
    }
}
impl Wire2Api<KeyExchangeRequest> for *mut wire_KeyExchangeRequest {
    fn wire2api(self) -> KeyExchangeRequest {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<KeyExchangeRequest>::wire2api(*wrap).into()
    }
}
impl Wire2Api<KeyboardEvent> for *mut wire_KeyboardEvent {
    fn wire2api(self) -> KeyboardEvent {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<KeyboardEvent>::wire2api(*wrap).into()
    }
}
impl Wire2Api<MouseEvent> for *mut wire_MouseEvent {
    fn wire2api(self) -> MouseEvent {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<MouseEvent>::wire2api(*wrap).into()
    }
}
impl Wire2Api<NegotiateFinishedRequest> for *mut wire_NegotiateFinishedRequest {
    fn wire2api(self) -> NegotiateFinishedRequest {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<NegotiateFinishedRequest>::wire2api(*wrap).into()
    }
}
impl Wire2Api<NegotiateSelectMonitorRequest> for *mut wire_NegotiateSelectMonitorRequest {
    fn wire2api(self) -> NegotiateSelectMonitorRequest {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<NegotiateSelectMonitorRequest>::wire2api(*wrap).into()
    }
}
impl Wire2Api<NegotiateVisitDesktopParamsRequest> for *mut wire_NegotiateVisitDesktopParamsRequest {
    fn wire2api(self) -> NegotiateVisitDesktopParamsRequest {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<NegotiateVisitDesktopParamsRequest>::wire2api(*wrap).into()
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
impl Wire2Api<VisitReplyRequest> for *mut wire_VisitReplyRequest {
    fn wire2api(self) -> VisitReplyRequest {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<VisitReplyRequest>::wire2api(*wrap).into()
    }
}
impl Wire2Api<VisitRequest> for *mut wire_VisitRequest {
    fn wire2api(self) -> VisitRequest {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<VisitRequest>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Box<InputEvent>> for *mut wire_InputEvent {
    fn wire2api(self) -> Box<InputEvent> {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<InputEvent>::wire2api(*wrap).into()
    }
}
impl Wire2Api<ConnectRequest> for wire_ConnectRequest {
    fn wire2api(self) -> ConnectRequest {
        ConnectRequest {
            local_device_id: self.local_device_id.wire2api(),
            remote_device_id: self.remote_device_id.wire2api(),
            addr: self.addr.wire2api(),
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
impl Wire2Api<DomainConfig> for wire_DomainConfig {
    fn wire2api(self) -> DomainConfig {
        DomainConfig {
            uri: self.uri.wire2api(),
            device_id: self.device_id.wire2api(),
            device_finger_print: self.device_finger_print.wire2api(),
            device_password: self.device_password.wire2api(),
        }
    }
}

impl Wire2Api<HandshakeRequest> for wire_HandshakeRequest {
    fn wire2api(self) -> HandshakeRequest {
        HandshakeRequest {
            active_device_id: self.active_device_id.wire2api(),
            passive_device_id: self.passive_device_id.wire2api(),
            visit_credentials: self.visit_credentials.wire2api(),
            opening_key_bytes: self.opening_key_bytes.wire2api(),
            opening_nonce_bytes: self.opening_nonce_bytes.wire2api(),
            sealing_key_bytes: self.sealing_key_bytes.wire2api(),
            sealing_nonce_bytes: self.sealing_nonce_bytes.wire2api(),
        }
    }
}
impl Wire2Api<HeartbeatRequest> for wire_HeartbeatRequest {
    fn wire2api(self) -> HeartbeatRequest {
        HeartbeatRequest {
            device_id: self.device_id.wire2api(),
            timestamp: self.timestamp.wire2api(),
        }
    }
}

impl Wire2Api<InputEvent> for wire_InputEvent {
    fn wire2api(self) -> InputEvent {
        match self.tag {
            0 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Mouse);
                InputEvent::Mouse(ans.field0.wire2api())
            },
            1 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Keyboard);
                InputEvent::Keyboard(ans.field0.wire2api())
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<InputRequest> for wire_InputRequest {
    fn wire2api(self) -> InputRequest {
        InputRequest {
            active_device_id: self.active_device_id.wire2api(),
            passive_device_id: self.passive_device_id.wire2api(),
            event: self.event.wire2api(),
        }
    }
}
impl Wire2Api<KeyExchangeRequest> for wire_KeyExchangeRequest {
    fn wire2api(self) -> KeyExchangeRequest {
        KeyExchangeRequest {
            domain: self.domain.wire2api(),
            local_device_id: self.local_device_id.wire2api(),
            remote_device_id: self.remote_device_id.wire2api(),
            password: self.password.wire2api(),
        }
    }
}
impl Wire2Api<KeyboardEvent> for wire_KeyboardEvent {
    fn wire2api(self) -> KeyboardEvent {
        match self.tag {
            0 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.KeyUp);
                KeyboardEvent::KeyUp(ans.field0.wire2api())
            },
            1 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.KeyDown);
                KeyboardEvent::KeyDown(ans.field0.wire2api())
            },
            _ => unreachable!(),
        }
    }
}

impl Wire2Api<MouseEvent> for wire_MouseEvent {
    fn wire2api(self) -> MouseEvent {
        match self.tag {
            0 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.MouseUp);
                MouseEvent::MouseUp(
                    ans.field0.wire2api(),
                    ans.field1.wire2api(),
                    ans.field2.wire2api(),
                )
            },
            1 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.MouseDown);
                MouseEvent::MouseDown(
                    ans.field0.wire2api(),
                    ans.field1.wire2api(),
                    ans.field2.wire2api(),
                )
            },
            2 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.MouseMove);
                MouseEvent::MouseMove(
                    ans.field0.wire2api(),
                    ans.field1.wire2api(),
                    ans.field2.wire2api(),
                )
            },
            3 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.MouseScrollWheel);
                MouseEvent::MouseScrollWheel(ans.field0.wire2api())
            },
            _ => unreachable!(),
        }
    }
}

impl Wire2Api<NegotiateFinishedRequest> for wire_NegotiateFinishedRequest {
    fn wire2api(self) -> NegotiateFinishedRequest {
        NegotiateFinishedRequest {
            active_device_id: self.active_device_id.wire2api(),
            passive_device_id: self.passive_device_id.wire2api(),
            selected_monitor_id: self.selected_monitor_id.wire2api(),
            expect_frame_rate: self.expect_frame_rate.wire2api(),
        }
    }
}
impl Wire2Api<NegotiateSelectMonitorRequest> for wire_NegotiateSelectMonitorRequest {
    fn wire2api(self) -> NegotiateSelectMonitorRequest {
        NegotiateSelectMonitorRequest {
            active_device_id: self.active_device_id.wire2api(),
            passive_device_id: self.passive_device_id.wire2api(),
        }
    }
}
impl Wire2Api<NegotiateVisitDesktopParamsRequest> for wire_NegotiateVisitDesktopParamsRequest {
    fn wire2api(self) -> NegotiateVisitDesktopParamsRequest {
        NegotiateVisitDesktopParamsRequest {
            active_device_id: self.active_device_id.wire2api(),
            passive_device_id: self.passive_device_id.wire2api(),
        }
    }
}

impl Wire2Api<RegisterRequest> for wire_RegisterRequest {
    fn wire2api(self) -> RegisterRequest {
        RegisterRequest {
            device_id: self.device_id.wire2api(),
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
impl Wire2Api<VisitReplyRequest> for wire_VisitReplyRequest {
    fn wire2api(self) -> VisitReplyRequest {
        VisitReplyRequest {
            domain: self.domain.wire2api(),
            active_device_id: self.active_device_id.wire2api(),
            passive_device_id: self.passive_device_id.wire2api(),
            allow: self.allow.wire2api(),
        }
    }
}
impl Wire2Api<VisitRequest> for wire_VisitRequest {
    fn wire2api(self) -> VisitRequest {
        VisitRequest {
            domain: self.domain.wire2api(),
            local_device_id: self.local_device_id.wire2api(),
            remote_device_id: self.remote_device_id.wire2api(),
            resource_type: self.resource_type.wire2api(),
        }
    }
}
// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_ConnectRequest {
    local_device_id: i64,
    remote_device_id: i64,
    addr: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_DialRequest {
    uri: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_DomainConfig {
    uri: *mut wire_uint_8_list,
    device_id: i64,
    device_finger_print: *mut wire_uint_8_list,
    device_password: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_HandshakeRequest {
    active_device_id: i64,
    passive_device_id: i64,
    visit_credentials: *mut wire_uint_8_list,
    opening_key_bytes: *mut wire_uint_8_list,
    opening_nonce_bytes: *mut wire_uint_8_list,
    sealing_key_bytes: *mut wire_uint_8_list,
    sealing_nonce_bytes: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_HeartbeatRequest {
    device_id: i64,
    timestamp: u32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_InputRequest {
    active_device_id: i64,
    passive_device_id: i64,
    event: *mut wire_InputEvent,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_KeyExchangeRequest {
    domain: *mut wire_uint_8_list,
    local_device_id: i64,
    remote_device_id: i64,
    password: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_NegotiateFinishedRequest {
    active_device_id: i64,
    passive_device_id: i64,
    selected_monitor_id: *mut wire_uint_8_list,
    expect_frame_rate: u8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_NegotiateSelectMonitorRequest {
    active_device_id: i64,
    passive_device_id: i64,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_NegotiateVisitDesktopParamsRequest {
    active_device_id: i64,
    passive_device_id: i64,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RegisterRequest {
    device_id: *mut i64,
    device_finger_print: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_SubscribeRequest {
    local_device_id: i64,
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
pub struct wire_VisitReplyRequest {
    domain: *mut wire_uint_8_list,
    active_device_id: i64,
    passive_device_id: i64,
    allow: bool,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_VisitRequest {
    domain: *mut wire_uint_8_list,
    local_device_id: i64,
    remote_device_id: i64,
    resource_type: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_InputEvent {
    tag: i32,
    kind: *mut InputEventKind,
}

#[repr(C)]
pub union InputEventKind {
    Mouse: *mut wire_InputEvent_Mouse,
    Keyboard: *mut wire_InputEvent_Keyboard,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_InputEvent_Mouse {
    field0: *mut wire_MouseEvent,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_InputEvent_Keyboard {
    field0: *mut wire_KeyboardEvent,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_KeyboardEvent {
    tag: i32,
    kind: *mut KeyboardEventKind,
}

#[repr(C)]
pub union KeyboardEventKind {
    KeyUp: *mut wire_KeyboardEvent_KeyUp,
    KeyDown: *mut wire_KeyboardEvent_KeyDown,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_KeyboardEvent_KeyUp {
    field0: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_KeyboardEvent_KeyDown {
    field0: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_MouseEvent {
    tag: i32,
    kind: *mut MouseEventKind,
}

#[repr(C)]
pub union MouseEventKind {
    MouseUp: *mut wire_MouseEvent_MouseUp,
    MouseDown: *mut wire_MouseEvent_MouseDown,
    MouseMove: *mut wire_MouseEvent_MouseMove,
    MouseScrollWheel: *mut wire_MouseEvent_MouseScrollWheel,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_MouseEvent_MouseUp {
    field0: i32,
    field1: f32,
    field2: f32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_MouseEvent_MouseDown {
    field0: i32,
    field1: f32,
    field2: f32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_MouseEvent_MouseMove {
    field0: i32,
    field1: f32,
    field2: f32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_MouseEvent_MouseScrollWheel {
    field0: f32,
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

impl NewWithNullPtr for wire_ConnectRequest {
    fn new_with_null_ptr() -> Self {
        Self {
            local_device_id: Default::default(),
            remote_device_id: Default::default(),
            addr: core::ptr::null_mut(),
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

impl NewWithNullPtr for wire_DomainConfig {
    fn new_with_null_ptr() -> Self {
        Self {
            uri: core::ptr::null_mut(),
            device_id: Default::default(),
            device_finger_print: core::ptr::null_mut(),
            device_password: core::ptr::null_mut(),
        }
    }
}

impl NewWithNullPtr for wire_HandshakeRequest {
    fn new_with_null_ptr() -> Self {
        Self {
            active_device_id: Default::default(),
            passive_device_id: Default::default(),
            visit_credentials: core::ptr::null_mut(),
            opening_key_bytes: core::ptr::null_mut(),
            opening_nonce_bytes: core::ptr::null_mut(),
            sealing_key_bytes: core::ptr::null_mut(),
            sealing_nonce_bytes: core::ptr::null_mut(),
        }
    }
}

impl NewWithNullPtr for wire_HeartbeatRequest {
    fn new_with_null_ptr() -> Self {
        Self {
            device_id: Default::default(),
            timestamp: Default::default(),
        }
    }
}

impl NewWithNullPtr for wire_InputEvent {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn inflate_InputEvent_Mouse() -> *mut InputEventKind {
    support::new_leak_box_ptr(InputEventKind {
        Mouse: support::new_leak_box_ptr(wire_InputEvent_Mouse {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_InputEvent_Keyboard() -> *mut InputEventKind {
    support::new_leak_box_ptr(InputEventKind {
        Keyboard: support::new_leak_box_ptr(wire_InputEvent_Keyboard {
            field0: core::ptr::null_mut(),
        }),
    })
}

impl NewWithNullPtr for wire_InputRequest {
    fn new_with_null_ptr() -> Self {
        Self {
            active_device_id: Default::default(),
            passive_device_id: Default::default(),
            event: core::ptr::null_mut(),
        }
    }
}

impl NewWithNullPtr for wire_KeyExchangeRequest {
    fn new_with_null_ptr() -> Self {
        Self {
            domain: core::ptr::null_mut(),
            local_device_id: Default::default(),
            remote_device_id: Default::default(),
            password: core::ptr::null_mut(),
        }
    }
}

impl NewWithNullPtr for wire_KeyboardEvent {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn inflate_KeyboardEvent_KeyUp() -> *mut KeyboardEventKind {
    support::new_leak_box_ptr(KeyboardEventKind {
        KeyUp: support::new_leak_box_ptr(wire_KeyboardEvent_KeyUp {
            field0: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_KeyboardEvent_KeyDown() -> *mut KeyboardEventKind {
    support::new_leak_box_ptr(KeyboardEventKind {
        KeyDown: support::new_leak_box_ptr(wire_KeyboardEvent_KeyDown {
            field0: Default::default(),
        }),
    })
}

impl NewWithNullPtr for wire_MouseEvent {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn inflate_MouseEvent_MouseUp() -> *mut MouseEventKind {
    support::new_leak_box_ptr(MouseEventKind {
        MouseUp: support::new_leak_box_ptr(wire_MouseEvent_MouseUp {
            field0: Default::default(),
            field1: Default::default(),
            field2: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_MouseEvent_MouseDown() -> *mut MouseEventKind {
    support::new_leak_box_ptr(MouseEventKind {
        MouseDown: support::new_leak_box_ptr(wire_MouseEvent_MouseDown {
            field0: Default::default(),
            field1: Default::default(),
            field2: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_MouseEvent_MouseMove() -> *mut MouseEventKind {
    support::new_leak_box_ptr(MouseEventKind {
        MouseMove: support::new_leak_box_ptr(wire_MouseEvent_MouseMove {
            field0: Default::default(),
            field1: Default::default(),
            field2: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_MouseEvent_MouseScrollWheel() -> *mut MouseEventKind {
    support::new_leak_box_ptr(MouseEventKind {
        MouseScrollWheel: support::new_leak_box_ptr(wire_MouseEvent_MouseScrollWheel {
            field0: Default::default(),
        }),
    })
}

impl NewWithNullPtr for wire_NegotiateFinishedRequest {
    fn new_with_null_ptr() -> Self {
        Self {
            active_device_id: Default::default(),
            passive_device_id: Default::default(),
            selected_monitor_id: core::ptr::null_mut(),
            expect_frame_rate: Default::default(),
        }
    }
}

impl NewWithNullPtr for wire_NegotiateSelectMonitorRequest {
    fn new_with_null_ptr() -> Self {
        Self {
            active_device_id: Default::default(),
            passive_device_id: Default::default(),
        }
    }
}

impl NewWithNullPtr for wire_NegotiateVisitDesktopParamsRequest {
    fn new_with_null_ptr() -> Self {
        Self {
            active_device_id: Default::default(),
            passive_device_id: Default::default(),
        }
    }
}

impl NewWithNullPtr for wire_RegisterRequest {
    fn new_with_null_ptr() -> Self {
        Self {
            device_id: core::ptr::null_mut(),
            device_finger_print: core::ptr::null_mut(),
        }
    }
}

impl NewWithNullPtr for wire_SubscribeRequest {
    fn new_with_null_ptr() -> Self {
        Self {
            local_device_id: Default::default(),
            device_finger_print: core::ptr::null_mut(),
            config_path: core::ptr::null_mut(),
        }
    }
}

impl NewWithNullPtr for wire_VisitReplyRequest {
    fn new_with_null_ptr() -> Self {
        Self {
            domain: core::ptr::null_mut(),
            active_device_id: Default::default(),
            passive_device_id: Default::default(),
            allow: Default::default(),
        }
    }
}

impl NewWithNullPtr for wire_VisitRequest {
    fn new_with_null_ptr() -> Self {
        Self {
            domain: core::ptr::null_mut(),
            local_device_id: Default::default(),
            remote_device_id: Default::default(),
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
