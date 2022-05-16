#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct wire_uint_8_list {
  uint8_t *ptr;
  int32_t len;
} wire_uint_8_list;

typedef struct WireSyncReturnStruct {
  uint8_t *ptr;
  int32_t len;
  bool success;
} WireSyncReturnStruct;

typedef int64_t DartPort;

typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);

void wire_init(int64_t port_,
               struct wire_uint_8_list *os_name,
               struct wire_uint_8_list *os_version,
               struct wire_uint_8_list *config_dir);

void wire_config_read_device_id(int64_t port_);

void wire_config_save_device_id(int64_t port_, struct wire_uint_8_list *device_id);

void wire_config_read_device_id_expiration(int64_t port_);

void wire_config_save_device_id_expiration(int64_t port_, uint32_t time_stamp);

void wire_config_read_device_password(int64_t port_);

void wire_config_save_device_password(int64_t port_, struct wire_uint_8_list *device_password);

void wire_desktop_connect(int64_t port_, struct wire_uint_8_list *remote_device_id);

void wire_desktop_key_exchange_and_password_verify(int64_t port_,
                                                   struct wire_uint_8_list *remote_device_id,
                                                   struct wire_uint_8_list *password);

void wire_desktop_start_media_transmission(int64_t port_,
                                           struct wire_uint_8_list *remote_device_id);

void wire_desktop_register_frame_stream(int64_t port_, struct wire_uint_8_list *remote_device_id);

void wire_do_test_for_swift(int64_t port_);

void wire_utility_generate_device_password(int64_t port_);

struct wire_uint_8_list *new_uint_8_list(int32_t len);

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
    dummy_var ^= ((int64_t) (void*) wire_desktop_connect);
    dummy_var ^= ((int64_t) (void*) wire_desktop_key_exchange_and_password_verify);
    dummy_var ^= ((int64_t) (void*) wire_desktop_start_media_transmission);
    dummy_var ^= ((int64_t) (void*) wire_desktop_register_frame_stream);
    dummy_var ^= ((int64_t) (void*) wire_do_test_for_swift);
    dummy_var ^= ((int64_t) (void*) wire_utility_generate_device_password);
    dummy_var ^= ((int64_t) (void*) new_uint_8_list);
    dummy_var ^= ((int64_t) (void*) free_WireSyncReturnStruct);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    return dummy_var;
}