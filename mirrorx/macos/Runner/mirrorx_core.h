#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef int64_t DartPort;

typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);

typedef struct wire_uint_8_list {
  uint8_t *ptr;
  int32_t len;
} wire_uint_8_list;

typedef struct wire_MouseEvent_MouseUp {
  int32_t field0;
  float field1;
  float field2;
} wire_MouseEvent_MouseUp;

typedef struct wire_MouseEvent_MouseDown {
  int32_t field0;
  float field1;
  float field2;
} wire_MouseEvent_MouseDown;

typedef struct wire_MouseEvent_MouseMove {
  int32_t field0;
  float field1;
  float field2;
} wire_MouseEvent_MouseMove;

typedef struct wire_MouseEvent_MouseScrollWheel {
  float field0;
} wire_MouseEvent_MouseScrollWheel;

typedef union MouseEventKind {
  struct wire_MouseEvent_MouseUp *MouseUp;
  struct wire_MouseEvent_MouseDown *MouseDown;
  struct wire_MouseEvent_MouseMove *MouseMove;
  struct wire_MouseEvent_MouseScrollWheel *MouseScrollWheel;
} MouseEventKind;

typedef struct wire_MouseEvent {
  int32_t tag;
  union MouseEventKind *kind;
} wire_MouseEvent;

typedef struct wire_InputEvent_Mouse {
  struct wire_MouseEvent *field0;
} wire_InputEvent_Mouse;

typedef struct wire_KeyboardEvent_KeyUp {
  int32_t field0;
} wire_KeyboardEvent_KeyUp;

typedef struct wire_KeyboardEvent_KeyDown {
  int32_t field0;
} wire_KeyboardEvent_KeyDown;

typedef union KeyboardEventKind {
  struct wire_KeyboardEvent_KeyUp *KeyUp;
  struct wire_KeyboardEvent_KeyDown *KeyDown;
} KeyboardEventKind;

typedef struct wire_KeyboardEvent {
  int32_t tag;
  union KeyboardEventKind *kind;
} wire_KeyboardEvent;

typedef struct wire_InputEvent_Keyboard {
  struct wire_KeyboardEvent *field0;
} wire_InputEvent_Keyboard;

typedef union InputEventKind {
  struct wire_InputEvent_Mouse *Mouse;
  struct wire_InputEvent_Keyboard *Keyboard;
} InputEventKind;

typedef struct wire_InputEvent {
  int32_t tag;
  union InputEventKind *kind;
} wire_InputEvent;

typedef struct WireSyncReturnStruct {
  uint8_t *ptr;
  int32_t len;
  bool success;
} WireSyncReturnStruct;

void store_dart_post_cobject(DartPostCObjectFnType ptr);

void wire_init(int64_t port_,
               struct wire_uint_8_list *os_version,
               struct wire_uint_8_list *config_dir);

void wire_config_read_device_id(int64_t port_);

void wire_config_save_device_id(int64_t port_, struct wire_uint_8_list *device_id);

void wire_config_read_device_id_expiration(int64_t port_);

void wire_config_save_device_id_expiration(int64_t port_, uint32_t time_stamp);

void wire_config_read_device_password(int64_t port_);

void wire_config_save_device_password(int64_t port_, struct wire_uint_8_list *device_password);

void wire_signaling_connect(int64_t port_, struct wire_uint_8_list *remote_device_id);

void wire_signaling_connection_key_exchange(int64_t port_,
                                            struct wire_uint_8_list *remote_device_id,
                                            struct wire_uint_8_list *password);

void wire_endpoint_get_display_info(int64_t port_, struct wire_uint_8_list *remote_device_id);

void wire_endpoint_start_media_transmission(int64_t port_,
                                            struct wire_uint_8_list *remote_device_id,
                                            uint8_t expect_fps,
                                            struct wire_uint_8_list *expect_display_id,
                                            int64_t texture_id,
                                            int64_t video_texture_ptr,
                                            int64_t update_frame_callback_ptr);

void wire_endpoint_input(int64_t port_,
                         struct wire_uint_8_list *remote_device_id,
                         struct wire_InputEvent *event);

void wire_endpoint_manually_close(int64_t port_, struct wire_uint_8_list *remote_device_id);

void wire_endpoint_close_notify(int64_t port_, struct wire_uint_8_list *remote_device_id);

struct wire_InputEvent *new_box_autoadd_input_event_0(void);

struct wire_KeyboardEvent *new_box_autoadd_keyboard_event_0(void);

struct wire_MouseEvent *new_box_autoadd_mouse_event_0(void);

struct wire_uint_8_list *new_uint_8_list_0(int32_t len);

union InputEventKind *inflate_InputEvent_Mouse(void);

union InputEventKind *inflate_InputEvent_Keyboard(void);

union KeyboardEventKind *inflate_KeyboardEvent_KeyUp(void);

union KeyboardEventKind *inflate_KeyboardEvent_KeyDown(void);

union MouseEventKind *inflate_MouseEvent_MouseUp(void);

union MouseEventKind *inflate_MouseEvent_MouseDown(void);

union MouseEventKind *inflate_MouseEvent_MouseMove(void);

union MouseEventKind *inflate_MouseEvent_MouseScrollWheel(void);

void free_WireSyncReturnStruct(struct WireSyncReturnStruct val);

static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) wire_init);
    dummy_var ^= ((int64_t) (void*) wire_config_read_device_id);
    dummy_var ^= ((int64_t) (void*) wire_config_save_device_id);
    dummy_var ^= ((int64_t) (void*) wire_config_read_device_id_expiration);
    dummy_var ^= ((int64_t) (void*) wire_config_save_device_id_expiration);
    dummy_var ^= ((int64_t) (void*) wire_config_read_device_password);
    dummy_var ^= ((int64_t) (void*) wire_config_save_device_password);
    dummy_var ^= ((int64_t) (void*) wire_signaling_connect);
    dummy_var ^= ((int64_t) (void*) wire_signaling_connection_key_exchange);
    dummy_var ^= ((int64_t) (void*) wire_endpoint_get_display_info);
    dummy_var ^= ((int64_t) (void*) wire_endpoint_start_media_transmission);
    dummy_var ^= ((int64_t) (void*) wire_endpoint_input);
    dummy_var ^= ((int64_t) (void*) wire_endpoint_manually_close);
    dummy_var ^= ((int64_t) (void*) wire_endpoint_close_notify);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_input_event_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_keyboard_event_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_mouse_event_0);
    dummy_var ^= ((int64_t) (void*) new_uint_8_list_0);
    dummy_var ^= ((int64_t) (void*) inflate_InputEvent_Mouse);
    dummy_var ^= ((int64_t) (void*) inflate_InputEvent_Keyboard);
    dummy_var ^= ((int64_t) (void*) inflate_KeyboardEvent_KeyUp);
    dummy_var ^= ((int64_t) (void*) inflate_KeyboardEvent_KeyDown);
    dummy_var ^= ((int64_t) (void*) inflate_MouseEvent_MouseUp);
    dummy_var ^= ((int64_t) (void*) inflate_MouseEvent_MouseDown);
    dummy_var ^= ((int64_t) (void*) inflate_MouseEvent_MouseMove);
    dummy_var ^= ((int64_t) (void*) inflate_MouseEvent_MouseScrollWheel);
    dummy_var ^= ((int64_t) (void*) free_WireSyncReturnStruct);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    return dummy_var;
}