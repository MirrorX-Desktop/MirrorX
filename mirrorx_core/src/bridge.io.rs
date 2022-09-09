use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_init(
    port_: i64,
    os_version: *mut wire_uint_8_list,
    config_dir: *mut wire_uint_8_list,
) {
    wire_init_impl(port_, os_version, config_dir)
}

#[no_mangle]
pub extern "C" fn wire_config_read_device_id(port_: i64) {
    wire_config_read_device_id_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_config_save_device_id(port_: i64, device_id: *mut wire_uint_8_list) {
    wire_config_save_device_id_impl(port_, device_id)
}

#[no_mangle]
pub extern "C" fn wire_config_read_device_id_expiration(port_: i64) {
    wire_config_read_device_id_expiration_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_config_save_device_id_expiration(port_: i64, time_stamp: u32) {
    wire_config_save_device_id_expiration_impl(port_, time_stamp)
}

#[no_mangle]
pub extern "C" fn wire_config_read_device_password(port_: i64) {
    wire_config_read_device_password_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_config_save_device_password(
    port_: i64,
    device_password: *mut wire_uint_8_list,
) {
    wire_config_save_device_password_impl(port_, device_password)
}

#[no_mangle]
pub extern "C" fn wire_signaling_connect(port_: i64, remote_device_id: *mut wire_uint_8_list) {
    wire_signaling_connect_impl(port_, remote_device_id)
}

#[no_mangle]
pub extern "C" fn wire_signaling_connection_key_exchange(
    port_: i64,
    remote_device_id: *mut wire_uint_8_list,
    password: *mut wire_uint_8_list,
) {
    wire_signaling_connection_key_exchange_impl(port_, remote_device_id, password)
}

#[no_mangle]
pub extern "C" fn wire_endpoint_get_display_info(
    port_: i64,
    remote_device_id: *mut wire_uint_8_list,
) {
    wire_endpoint_get_display_info_impl(port_, remote_device_id)
}

#[no_mangle]
pub extern "C" fn wire_endpoint_start_media_transmission(
    port_: i64,
    remote_device_id: *mut wire_uint_8_list,
    expect_fps: u8,
    expect_display_id: *mut wire_uint_8_list,
    texture_id: i64,
    video_texture_ptr: i64,
    update_frame_callback_ptr: i64,
) {
    wire_endpoint_start_media_transmission_impl(
        port_,
        remote_device_id,
        expect_fps,
        expect_display_id,
        texture_id,
        video_texture_ptr,
        update_frame_callback_ptr,
    )
}

#[no_mangle]
pub extern "C" fn wire_endpoint_input(
    port_: i64,
    remote_device_id: *mut wire_uint_8_list,
    event: *mut wire_InputEvent,
) {
    wire_endpoint_input_impl(port_, remote_device_id, event)
}

#[no_mangle]
pub extern "C" fn wire_endpoint_manually_close(
    port_: i64,
    remote_device_id: *mut wire_uint_8_list,
) {
    wire_endpoint_manually_close_impl(port_, remote_device_id)
}

#[no_mangle]
pub extern "C" fn wire_endpoint_close_notify(port_: i64, remote_device_id: *mut wire_uint_8_list) {
    wire_endpoint_close_notify_impl(port_, remote_device_id)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_box_autoadd_input_event_0() -> *mut wire_InputEvent {
    support::new_leak_box_ptr(wire_InputEvent::new_with_null_ptr())
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
impl Wire2Api<InputEvent> for *mut wire_InputEvent {
    fn wire2api(self) -> InputEvent {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<InputEvent>::wire2api(*wrap).into()
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

impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
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

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturnStruct(val: support::WireSyncReturnStruct) {
    unsafe {
        let _ = support::vec_from_leak_ptr(val.ptr, val.len);
    }
}
