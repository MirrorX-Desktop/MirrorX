#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef int64_t DartPort;

typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);

typedef struct wire_uint_8_list {
  uint8_t *ptr;
  int32_t len;
} wire_uint_8_list;

typedef struct wire_ConfigProperties {
  struct wire_uint_8_list *domain;
  struct wire_uint_8_list *device_id;
  struct wire_uint_8_list *device_finger_print;
  struct wire_uint_8_list *device_password;
} wire_ConfigProperties;

typedef struct wire_DialRequest {
  struct wire_uint_8_list *uri;
} wire_DialRequest;

typedef struct wire_RegisterRequest {
  struct wire_uint_8_list *local_device_id;
  struct wire_uint_8_list *device_finger_print;
} wire_RegisterRequest;

typedef struct wire_SubscribeRequest {
  struct wire_uint_8_list *local_device_id;
  struct wire_uint_8_list *device_finger_print;
  struct wire_uint_8_list *config_path;
} wire_SubscribeRequest;

typedef struct wire_HeartbeatRequest {
  struct wire_uint_8_list *local_device_id;
  uint32_t timestamp;
} wire_HeartbeatRequest;

typedef struct wire_VisitRequest {
  struct wire_uint_8_list *local_device_id;
  struct wire_uint_8_list *remote_device_id;
  int32_t resource_type;
} wire_VisitRequest;

typedef struct wire_KeyExchangeRequest {
  struct wire_uint_8_list *local_device_id;
  struct wire_uint_8_list *remote_device_id;
  struct wire_uint_8_list *password;
} wire_KeyExchangeRequest;

typedef struct wire_ConnectRequest {
  struct wire_uint_8_list *active_device_id;
  struct wire_uint_8_list *passive_device_id;
  struct wire_uint_8_list *addr;
} wire_ConnectRequest;

typedef struct wire_HandshakeRequest {
  struct wire_uint_8_list *active_device_id;
  struct wire_uint_8_list *passive_device_id;
  struct wire_uint_8_list *visit_credentials;
  struct wire_uint_8_list *opening_key_bytes;
  struct wire_uint_8_list *opening_nonce_bytes;
  struct wire_uint_8_list *sealing_key_bytes;
  struct wire_uint_8_list *sealing_nonce_bytes;
} wire_HandshakeRequest;

typedef struct wire_NegotiateVisitDesktopParamsRequest {
  struct wire_uint_8_list *active_device_id;
  struct wire_uint_8_list *passive_device_id;
} wire_NegotiateVisitDesktopParamsRequest;

typedef struct wire_NegotiateSelectMonitorRequest {
  struct wire_uint_8_list *active_device_id;
  struct wire_uint_8_list *passive_device_id;
} wire_NegotiateSelectMonitorRequest;

typedef struct wire_NegotiateFinishedRequest {
  struct wire_uint_8_list *active_device_id;
  struct wire_uint_8_list *passive_device_id;
  struct wire_uint_8_list *selected_monitor_id;
  uint8_t expect_frame_rate;
} wire_NegotiateFinishedRequest;

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

typedef struct wire_InputReqeust {
  struct wire_uint_8_list *active_device_id;
  struct wire_uint_8_list *passive_device_id;
  struct wire_InputEvent *event;
} wire_InputReqeust;

typedef struct WireSyncReturnStruct {
  uint8_t *ptr;
  int32_t len;
  bool success;
} WireSyncReturnStruct;

void store_dart_post_cobject(DartPostCObjectFnType ptr);

void wire_init_logger(int64_t port_);

void wire_config_read(int64_t port_,
                      struct wire_uint_8_list *path,
                      struct wire_uint_8_list *domain);

void wire_config_save(int64_t port_,
                      struct wire_uint_8_list *path,
                      struct wire_uint_8_list *domain,
                      struct wire_ConfigProperties *properties);

void wire_config_read_all(int64_t port_, struct wire_uint_8_list *path);

void wire_signaling_dial(int64_t port_, struct wire_DialRequest *req);

void wire_signaling_register(int64_t port_, struct wire_RegisterRequest *req);

void wire_signaling_subscribe(int64_t port_, struct wire_SubscribeRequest *req);

void wire_signaling_heartbeat(int64_t port_, struct wire_HeartbeatRequest *req);

void wire_signaling_visit(int64_t port_, struct wire_VisitRequest *req);

void wire_signaling_key_exchange(int64_t port_, struct wire_KeyExchangeRequest *req);

void wire_endpoint_connect(int64_t port_, struct wire_ConnectRequest *req);

void wire_endpoint_handshake(int64_t port_, struct wire_HandshakeRequest *req);

void wire_endpoint_negotiate_visit_desktop_params(int64_t port_,
                                                  struct wire_NegotiateVisitDesktopParamsRequest *req);

void wire_endpoint_negotiate_select_monitor(int64_t port_,
                                            struct wire_NegotiateSelectMonitorRequest *req);

void wire_endpoint_negotiate_finished(int64_t port_, struct wire_NegotiateFinishedRequest *req);

void wire_endpoint_input(int64_t port_, struct wire_InputReqeust *req);

struct wire_ConfigProperties *new_box_autoadd_config_properties_0(void);

struct wire_ConnectRequest *new_box_autoadd_connect_request_0(void);

struct wire_DialRequest *new_box_autoadd_dial_request_0(void);

struct wire_HandshakeRequest *new_box_autoadd_handshake_request_0(void);

struct wire_HeartbeatRequest *new_box_autoadd_heartbeat_request_0(void);

struct wire_InputReqeust *new_box_autoadd_input_reqeust_0(void);

struct wire_KeyExchangeRequest *new_box_autoadd_key_exchange_request_0(void);

struct wire_KeyboardEvent *new_box_autoadd_keyboard_event_0(void);

struct wire_MouseEvent *new_box_autoadd_mouse_event_0(void);

struct wire_NegotiateFinishedRequest *new_box_autoadd_negotiate_finished_request_0(void);

struct wire_NegotiateSelectMonitorRequest *new_box_autoadd_negotiate_select_monitor_request_0(void);

struct wire_NegotiateVisitDesktopParamsRequest *new_box_autoadd_negotiate_visit_desktop_params_request_0(void);

struct wire_RegisterRequest *new_box_autoadd_register_request_0(void);

struct wire_SubscribeRequest *new_box_autoadd_subscribe_request_0(void);

struct wire_VisitRequest *new_box_autoadd_visit_request_0(void);

struct wire_InputEvent *new_box_input_event_0(void);

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
    dummy_var ^= ((int64_t) (void*) wire_init_logger);
    dummy_var ^= ((int64_t) (void*) wire_config_read);
    dummy_var ^= ((int64_t) (void*) wire_config_save);
    dummy_var ^= ((int64_t) (void*) wire_config_read_all);
    dummy_var ^= ((int64_t) (void*) wire_signaling_dial);
    dummy_var ^= ((int64_t) (void*) wire_signaling_register);
    dummy_var ^= ((int64_t) (void*) wire_signaling_subscribe);
    dummy_var ^= ((int64_t) (void*) wire_signaling_heartbeat);
    dummy_var ^= ((int64_t) (void*) wire_signaling_visit);
    dummy_var ^= ((int64_t) (void*) wire_signaling_key_exchange);
    dummy_var ^= ((int64_t) (void*) wire_endpoint_connect);
    dummy_var ^= ((int64_t) (void*) wire_endpoint_handshake);
    dummy_var ^= ((int64_t) (void*) wire_endpoint_negotiate_visit_desktop_params);
    dummy_var ^= ((int64_t) (void*) wire_endpoint_negotiate_select_monitor);
    dummy_var ^= ((int64_t) (void*) wire_endpoint_negotiate_finished);
    dummy_var ^= ((int64_t) (void*) wire_endpoint_input);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_config_properties_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_connect_request_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_dial_request_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_handshake_request_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_heartbeat_request_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_input_reqeust_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_key_exchange_request_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_keyboard_event_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_mouse_event_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_negotiate_finished_request_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_negotiate_select_monitor_request_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_negotiate_visit_desktop_params_request_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_register_request_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_subscribe_request_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_visit_request_0);
    dummy_var ^= ((int64_t) (void*) new_box_input_event_0);
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