#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct wire_uint_8_list {
  uint8_t *ptr;
  int32_t len;
} wire_uint_8_list;

typedef struct MouseEvent_Up {
  int32_t field0;
} MouseEvent_Up;

typedef struct MouseEvent_Down {
  int32_t field0;
} MouseEvent_Down;

typedef struct MouseEvent_Move {
  int32_t field0;
} MouseEvent_Move;

typedef struct MouseEvent_ScrollWheel {
  float field0;
} MouseEvent_ScrollWheel;

typedef union MouseEventKind {
  struct MouseEvent_Up *Up;
  struct MouseEvent_Down *Down;
  struct MouseEvent_Move *Move;
  struct MouseEvent_ScrollWheel *ScrollWheel;
} MouseEventKind;

typedef struct wire_MouseEvent {
  int32_t tag;
  union MouseEventKind *kind;
} wire_MouseEvent;

typedef struct WireSyncReturnStruct {
  uint8_t *ptr;
  int32_t len;
  bool success;
} WireSyncReturnStruct;

typedef int64_t DartPort;

typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);

void wire_init(int64_t port_,
               struct wire_uint_8_list *os_type,
               struct wire_uint_8_list *os_version,
               struct wire_uint_8_list *config_dir);

void wire_config_read_device_id(int64_t port_);

void wire_config_save_device_id(int64_t port_, struct wire_uint_8_list *device_id);

void wire_config_read_device_id_expiration(int64_t port_);

void wire_config_save_device_id_expiration(int64_t port_, int32_t time_stamp);

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

void wire_endpoint_mouse_event(int64_t port_,
                               struct wire_uint_8_list *remote_device_id,
                               struct wire_MouseEvent *event,
                               float x,
                               float y);

struct wire_MouseEvent *new_box_autoadd_mouse_event_0(void);

struct wire_uint_8_list *new_uint_8_list_0(int32_t len);

union MouseEventKind *inflate_MouseEvent_Up(void);

union MouseEventKind *inflate_MouseEvent_Down(void);

union MouseEventKind *inflate_MouseEvent_Move(void);

union MouseEventKind *inflate_MouseEvent_ScrollWheel(void);

void free_WireSyncReturnStruct(struct WireSyncReturnStruct val);

void store_dart_post_cobject(DartPostCObjectFnType ptr);

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
    dummy_var ^= ((int64_t) (void*) wire_endpoint_mouse_event);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_mouse_event_0);
    dummy_var ^= ((int64_t) (void*) new_uint_8_list_0);
    dummy_var ^= ((int64_t) (void*) inflate_MouseEvent_Up);
    dummy_var ^= ((int64_t) (void*) inflate_MouseEvent_Down);
    dummy_var ^= ((int64_t) (void*) inflate_MouseEvent_Move);
    dummy_var ^= ((int64_t) (void*) inflate_MouseEvent_ScrollWheel);
    dummy_var ^= ((int64_t) (void*) free_WireSyncReturnStruct);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    return dummy_var;
}