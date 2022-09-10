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

typedef struct WireSyncReturnStruct {
  uint8_t *ptr;
  int32_t len;
  bool success;
} WireSyncReturnStruct;

void store_dart_post_cobject(DartPostCObjectFnType ptr);

void wire_logger_init(int64_t port_);

void wire_config_read(int64_t port_, struct wire_uint_8_list *path, struct wire_uint_8_list *key);

void wire_config_save(int64_t port_,
                      struct wire_uint_8_list *path,
                      struct wire_uint_8_list *key,
                      struct wire_ConfigProperties *properties);

void wire_signaling_dial(int64_t port_, struct wire_DialRequest *req);

void wire_signaling_register(int64_t port_, struct wire_RegisterRequest *req);

void wire_signaling_subscribe(int64_t port_, struct wire_SubscribeRequest *req);

void wire_signaling_heartbeat(int64_t port_, struct wire_HeartbeatRequest *req);

void wire_signaling_visit(int64_t port_, struct wire_VisitRequest *req);

void wire_signaling_key_exchange(int64_t port_, struct wire_KeyExchangeRequest *req);

struct wire_ConfigProperties *new_box_autoadd_config_properties_0(void);

struct wire_DialRequest *new_box_autoadd_dial_request_0(void);

struct wire_HeartbeatRequest *new_box_autoadd_heartbeat_request_0(void);

struct wire_KeyExchangeRequest *new_box_autoadd_key_exchange_request_0(void);

struct wire_RegisterRequest *new_box_autoadd_register_request_0(void);

struct wire_SubscribeRequest *new_box_autoadd_subscribe_request_0(void);

struct wire_VisitRequest *new_box_autoadd_visit_request_0(void);

struct wire_uint_8_list *new_uint_8_list_0(int32_t len);

void free_WireSyncReturnStruct(struct WireSyncReturnStruct val);

static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) wire_logger_init);
    dummy_var ^= ((int64_t) (void*) wire_config_read);
    dummy_var ^= ((int64_t) (void*) wire_config_save);
    dummy_var ^= ((int64_t) (void*) wire_signaling_dial);
    dummy_var ^= ((int64_t) (void*) wire_signaling_register);
    dummy_var ^= ((int64_t) (void*) wire_signaling_subscribe);
    dummy_var ^= ((int64_t) (void*) wire_signaling_heartbeat);
    dummy_var ^= ((int64_t) (void*) wire_signaling_visit);
    dummy_var ^= ((int64_t) (void*) wire_signaling_key_exchange);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_config_properties_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_dial_request_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_heartbeat_request_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_key_exchange_request_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_register_request_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_subscribe_request_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_visit_request_0);
    dummy_var ^= ((int64_t) (void*) new_uint_8_list_0);
    dummy_var ^= ((int64_t) (void*) free_WireSyncReturnStruct);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    return dummy_var;
}