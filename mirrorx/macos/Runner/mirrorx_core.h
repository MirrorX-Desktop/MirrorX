#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef int64_t DartPort;

typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);

typedef struct wire_uint_8_list {
  uint8_t *ptr;
  int32_t len;
} wire_uint_8_list;

typedef struct wire_DomainConfig {
  struct wire_uint_8_list *uri;
  int64_t device_id;
  struct wire_uint_8_list *device_finger_print;
  struct wire_uint_8_list *device_password;
} wire_DomainConfig;

typedef struct wire_DialRequest {
  struct wire_uint_8_list *uri;
} wire_DialRequest;

typedef struct wire_RegisterRequest {
  int64_t *device_id;
  struct wire_uint_8_list *device_finger_print;
} wire_RegisterRequest;

typedef struct wire_SubscribeRequest {
  int64_t local_device_id;
  struct wire_uint_8_list *device_finger_print;
  struct wire_uint_8_list *config_path;
} wire_SubscribeRequest;

typedef struct wire_HeartbeatRequest {
  int64_t device_id;
  uint32_t timestamp;
} wire_HeartbeatRequest;

typedef struct wire_VisitRequest {
  struct wire_uint_8_list *domain;
  int64_t local_device_id;
  int64_t remote_device_id;
  int32_t resource_type;
} wire_VisitRequest;

typedef struct wire_VisitReplyRequest {
  struct wire_uint_8_list *domain;
  int64_t active_device_id;
  int64_t passive_device_id;
  bool allow;
} wire_VisitReplyRequest;

typedef struct wire_KeyExchangeRequest {
  struct wire_uint_8_list *domain;
  int64_t local_device_id;
  int64_t remote_device_id;
  struct wire_uint_8_list *password;
} wire_KeyExchangeRequest;

typedef struct wire_ConnectRequest {
  int64_t active_device_id;
  int64_t passive_device_id;
  struct wire_uint_8_list *addr;
} wire_ConnectRequest;

typedef struct wire_HandshakeRequest {
  int64_t active_device_id;
  int64_t passive_device_id;
  struct wire_uint_8_list *visit_credentials;
  struct wire_uint_8_list *opening_key_bytes;
  struct wire_uint_8_list *opening_nonce_bytes;
  struct wire_uint_8_list *sealing_key_bytes;
  struct wire_uint_8_list *sealing_nonce_bytes;
} wire_HandshakeRequest;

typedef struct wire_NegotiateVisitDesktopParamsRequest {
  int64_t active_device_id;
  int64_t passive_device_id;
} wire_NegotiateVisitDesktopParamsRequest;

typedef struct wire_NegotiateSelectMonitorRequest {
  int64_t active_device_id;
  int64_t passive_device_id;
} wire_NegotiateSelectMonitorRequest;

typedef struct wire_NegotiateFinishedRequest {
  int64_t active_device_id;
  int64_t passive_device_id;
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

typedef struct wire_InputRequest {
  int64_t active_device_id;
  int64_t passive_device_id;
  struct wire_InputEvent *event;
} wire_InputRequest;

typedef struct WireSyncReturnStruct {
  uint8_t *ptr;
  int32_t len;
  bool success;
} WireSyncReturnStruct;

void store_dart_post_cobject(DartPostCObjectFnType ptr);

void wire_init_logger(int64_t port_);

void wire_read_primary_domain(int64_t port_, struct wire_uint_8_list *path);

void wire_save_primary_domain(int64_t port_,
                              struct wire_uint_8_list *path,
                              struct wire_uint_8_list *value);

void wire_read_domain_config(int64_t port_,
                             struct wire_uint_8_list *path,
                             struct wire_uint_8_list *domain);

void wire_save_domain_config(int64_t port_,
                             struct wire_uint_8_list *path,
                             struct wire_uint_8_list *domain,
                             struct wire_DomainConfig *value);

void wire_signaling_dial(int64_t port_, struct wire_DialRequest *req);

void wire_signaling_disconnect(int64_t port_);

void wire_signaling_register(int64_t port_, struct wire_RegisterRequest *req);

void wire_signaling_subscribe(int64_t port_, struct wire_SubscribeRequest *req);

void wire_signaling_heartbeat(int64_t port_, struct wire_HeartbeatRequest *req);

void wire_signaling_visit(int64_t port_, struct wire_VisitRequest *req);

void wire_signaling_visit_reply(int64_t port_, struct wire_VisitReplyRequest *req);

void wire_signaling_key_exchange(int64_t port_, struct wire_KeyExchangeRequest *req);

void wire_endpoint_connect(int64_t port_, struct wire_ConnectRequest *req);

void wire_endpoint_handshake(int64_t port_, struct wire_HandshakeRequest *req);

void wire_endpoint_negotiate_visit_desktop_params(int64_t port_,
                                                  struct wire_NegotiateVisitDesktopParamsRequest *req);

void wire_endpoint_negotiate_select_monitor(int64_t port_,
                                            struct wire_NegotiateSelectMonitorRequest *req);

void wire_endpoint_negotiate_finished(int64_t port_, struct wire_NegotiateFinishedRequest *req);

void wire_endpoint_input(int64_t port_, struct wire_InputRequest *req);

struct wire_ConnectRequest *new_box_autoadd_connect_request_0(void);

struct wire_DialRequest *new_box_autoadd_dial_request_0(void);

struct wire_DomainConfig *new_box_autoadd_domain_config_0(void);

struct wire_HandshakeRequest *new_box_autoadd_handshake_request_0(void);

struct wire_HeartbeatRequest *new_box_autoadd_heartbeat_request_0(void);

int64_t *new_box_autoadd_i64_0(int64_t value);

struct wire_InputRequest *new_box_autoadd_input_request_0(void);

struct wire_KeyExchangeRequest *new_box_autoadd_key_exchange_request_0(void);

struct wire_KeyboardEvent *new_box_autoadd_keyboard_event_0(void);

struct wire_MouseEvent *new_box_autoadd_mouse_event_0(void);

struct wire_NegotiateFinishedRequest *new_box_autoadd_negotiate_finished_request_0(void);

struct wire_NegotiateSelectMonitorRequest *new_box_autoadd_negotiate_select_monitor_request_0(void);

struct wire_NegotiateVisitDesktopParamsRequest *new_box_autoadd_negotiate_visit_desktop_params_request_0(void);

struct wire_RegisterRequest *new_box_autoadd_register_request_0(void);

struct wire_SubscribeRequest *new_box_autoadd_subscribe_request_0(void);

struct wire_VisitReplyRequest *new_box_autoadd_visit_reply_request_0(void);

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
    dummy_var ^= ((int64_t) (void*) wire_read_primary_domain);
    dummy_var ^= ((int64_t) (void*) wire_save_primary_domain);
    dummy_var ^= ((int64_t) (void*) wire_read_domain_config);
    dummy_var ^= ((int64_t) (void*) wire_save_domain_config);
    dummy_var ^= ((int64_t) (void*) wire_signaling_dial);
    dummy_var ^= ((int64_t) (void*) wire_signaling_disconnect);
    dummy_var ^= ((int64_t) (void*) wire_signaling_register);
    dummy_var ^= ((int64_t) (void*) wire_signaling_subscribe);
    dummy_var ^= ((int64_t) (void*) wire_signaling_heartbeat);
    dummy_var ^= ((int64_t) (void*) wire_signaling_visit);
    dummy_var ^= ((int64_t) (void*) wire_signaling_visit_reply);
    dummy_var ^= ((int64_t) (void*) wire_signaling_key_exchange);
    dummy_var ^= ((int64_t) (void*) wire_endpoint_connect);
    dummy_var ^= ((int64_t) (void*) wire_endpoint_handshake);
    dummy_var ^= ((int64_t) (void*) wire_endpoint_negotiate_visit_desktop_params);
    dummy_var ^= ((int64_t) (void*) wire_endpoint_negotiate_select_monitor);
    dummy_var ^= ((int64_t) (void*) wire_endpoint_negotiate_finished);
    dummy_var ^= ((int64_t) (void*) wire_endpoint_input);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_connect_request_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_dial_request_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_domain_config_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_handshake_request_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_heartbeat_request_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_i64_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_input_request_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_key_exchange_request_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_keyboard_event_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_mouse_event_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_negotiate_finished_request_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_negotiate_select_monitor_request_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_negotiate_visit_desktop_params_request_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_register_request_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_subscribe_request_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_visit_reply_request_0);
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