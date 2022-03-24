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

void wire_init_sdk(int64_t port_, struct wire_uint_8_list *config_db_path);

void wire_read_device_id(int64_t port_);

void wire_read_device_password(int64_t port_);

void wire_save_device_password(int64_t port_, struct wire_uint_8_list *device_password);

void wire_generate_random_device_password(int64_t port_);

void wire_device_goes_online(int64_t port_);

void wire_desktop_connect_to(int64_t port_, struct wire_uint_8_list *device_id);

struct wire_uint_8_list *new_uint_8_list(int32_t len);

void free_WireSyncReturnStruct(struct WireSyncReturnStruct val);

void store_dart_post_cobject(DartPostCObjectFnType ptr);

static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) wire_init_sdk);
    dummy_var ^= ((int64_t) (void*) wire_read_device_id);
    dummy_var ^= ((int64_t) (void*) wire_read_device_password);
    dummy_var ^= ((int64_t) (void*) wire_save_device_password);
    dummy_var ^= ((int64_t) (void*) wire_generate_random_device_password);
    dummy_var ^= ((int64_t) (void*) wire_device_goes_online);
    dummy_var ^= ((int64_t) (void*) wire_desktop_connect_to);
    dummy_var ^= ((int64_t) (void*) new_uint_8_list);
    dummy_var ^= ((int64_t) (void*) free_WireSyncReturnStruct);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    return dummy_var;
}