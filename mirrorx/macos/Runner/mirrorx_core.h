#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define OPUS_APPLICATION_VOIP 2048

#define OPUS_APPLICATION_AUDIO 2049

#define OPUS_APPLICATION_RESTRICTED_LOWDELAY 2051

typedef struct OpusDecoder OpusDecoder;

typedef struct OpusEncoder OpusEncoder;

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

struct wire_uint_8_list *new_uint_8_list(int32_t len);

void free_WireSyncReturnStruct(struct WireSyncReturnStruct val);

void store_dart_post_cobject(DartPostCObjectFnType ptr);

extern intptr_t opus_decoder_get_size(intptr_t channels);

extern struct OpusDecoder *opus_decoder_create(int32_t fs, intptr_t channels, intptr_t *error);

extern intptr_t opus_decoder_init(struct OpusDecoder *st, int32_t fs, intptr_t channels);

extern intptr_t opus_decode(struct OpusDecoder *st,
                            const uint8_t *data,
                            int32_t len,
                            int16_t *pcm,
                            intptr_t frame_size,
                            intptr_t decodec_fec);

extern intptr_t opus_decode_float(struct OpusDecoder *st,
                                  const uint8_t *data,
                                  int32_t len,
                                  float *pcm,
                                  intptr_t frame_size,
                                  intptr_t decodec_fec);

extern void opus_decoder_destroy(struct OpusDecoder *st);

extern intptr_t opus_packet_parse(const uint8_t *data,
                                  int32_t len,
                                  uint8_t *out_toc,
                                  uint8_t *const (*frames)[48],
                                  const int16_t (*size)[48],
                                  intptr_t *payload_offset);

extern intptr_t opus_packet_get_bandwidth(const uint8_t *data);

extern intptr_t opus_packet_get_samples_per_frame(const uint8_t *data, int32_t fs);

extern intptr_t opus_packet_get_nb_channels(const uint8_t *data);

extern intptr_t opus_packet_get_nb_frames(const uint8_t *packet, int32_t len);

extern intptr_t opus_packet_get_nb_samples(const uint8_t *packet, int32_t len, int32_t fs);

extern intptr_t opus_decoder_get_nb_samples(const struct OpusDecoder *dec,
                                            const uint8_t *packet,
                                            int32_t len);

extern void opus_pcm_soft_clip(float *pcm,
                               intptr_t frame_size,
                               intptr_t channels,
                               float *softclip_mem);

extern intptr_t opus_encoder_get_size(intptr_t channels);

extern struct OpusEncoder *opus_encoder_create(int32_t fs,
                                               intptr_t channels,
                                               intptr_t application,
                                               intptr_t *error);

extern intptr_t opus_encoder_init(struct OpusEncoder *st,
                                  int32_t fs,
                                  intptr_t channels,
                                  intptr_t application);

extern int32_t opus_encode(struct OpusEncoder *st,
                           const int16_t *pcm,
                           intptr_t frame_size,
                           uint8_t *data,
                           int32_t max_data_bytes);

extern int32_t opus_encode_float(struct OpusEncoder *st,
                                 const float *pcm,
                                 intptr_t frame_size,
                                 uint8_t *data,
                                 int32_t max_data_bytes);

extern void opus_encoder_destroy(struct OpusEncoder *st);

extern intptr_t opus_encoder_ctl(struct OpusEncoder *st, intptr_t request);

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
    dummy_var ^= ((int64_t) (void*) new_uint_8_list);
    dummy_var ^= ((int64_t) (void*) free_WireSyncReturnStruct);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    return dummy_var;
}