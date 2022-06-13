#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define AV_PARSER_PTS_NB 4

#define AV_CODEC_FLAG2_LOCAL_HEADER (1 << 3)

#define AV_CODEC_CAP_TRUNCATED (1 << 3)

#define AV_NUM_DATA_POINTERS 8

#define AV_LOG_QUIET -8

#define AV_LOG_PANIC 0

#define AV_LOG_FATAL 8

#define AV_LOG_ERROR 16

#define AV_LOG_WARNING 24

#define AV_LOG_INFO 32

#define AV_LOG_VERBOSE 40

#define AV_LOG_DEBUG 48

#define AV_LOG_TRACE 56

#define AV_LOG_SKIP_REPEATED 1

#define AV_LOG_PRINT_LEVEL 2

typedef struct AVBuffer AVBuffer;

typedef struct AVCodecDefault AVCodecDefault;

typedef struct AVCodecInternal AVCodecInternal;

typedef struct AVDictionary AVDictionary;

typedef struct MpegEncContext MpegEncContext;

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

typedef uint32_t AVOptionType;

typedef struct AVRational {
  int32_t num;
  int32_t den;
} AVRational;

typedef union AVOptionDefaultVal {
  int64_t i64_;
  double dbl;
  const char *str_;
  struct AVRational q;
} AVOptionDefaultVal;

typedef struct AVOption {
  const char *name;
  const char *help;
  int32_t offset;
  AVOptionType type_;
  union AVOptionDefaultVal default_val;
  double min;
  double max;
  int32_t flags;
  const char *unit;
} AVOption;

typedef uint32_t AVClassCategory;

typedef struct AVOptionRange {
  const char *str_;
  double value_min;
  double value_max;
  double component_min;
  double component_max;
  int32_t is_range;
} AVOptionRange;

typedef struct AVOptionRanges {
  struct AVOptionRange **range;
  int32_t nb_ranges;
  int32_t nb_components;
} AVOptionRanges;

typedef struct AVClass {
  const char *class_name;
  const char *(*item_name)(void *ctx);
  const struct AVOption *option;
  int32_t version;
  int32_t log_level_offset_offset;
  int32_t parent_log_context_offset;
  AVClassCategory category;
  AVClassCategory (*get_category)(void *ctx);
  int32_t (*query_ranges)(struct AVOptionRanges**, void *obj, const char *key, int32_t flags);
  void *(*child_next)(void *obj, void *prev);
  struct AVClass *(*child_class_iterate)(void **iter);
} AVClass;

typedef int32_t AVMediaType;

typedef int32_t AVPixelFormat;

typedef uint32_t AVPictureType;

typedef struct AVBufferRef {
  struct AVBuffer *buffer;
  uint8_t *data;
  uintptr_t size;
} AVBufferRef;

typedef uint32_t AVFrameSideDataType;

typedef struct AVFrameSideData {
  AVFrameSideDataType typ;
  uint8_t *data;
  uintptr_t size;
  struct AVDictionary *metadata;
  struct AVBufferRef *buf;
} AVFrameSideData;

typedef uint32_t AVColorRange;

typedef uint32_t AVColorPrimaries;

typedef uint32_t AVColorTransferCharacteristic;

typedef uint32_t AVColorSpace;

typedef uint32_t AVChromaLocation;

typedef struct AVFrame {
  uint8_t *data[AV_NUM_DATA_POINTERS];
  int32_t linesize[AV_NUM_DATA_POINTERS];
  uint8_t **extended_data;
  int32_t width;
  int32_t height;
  int32_t nb_samples;
  AVPixelFormat format;
  int32_t key_frame;
  AVPictureType pict_type;
  struct AVRational sample_aspect_ratio;
  int64_t pts;
  int64_t pkt_pts;
  struct AVRational time_base;
  int32_t coded_picture_number;
  int32_t display_picture_number;
  int32_t quality;
  void *opaque;
  int32_t repeat_pict;
  int32_t interlaced_frame;
  int32_t top_field_first;
  int32_t palette_has_changed;
  int64_t reordered_opaque;
  int32_t sample_rate;
  uint64_t channel_layout;
  struct AVBufferRef *buf[AV_NUM_DATA_POINTERS];
  struct AVBufferRef **extended_buf;
  int32_t nb_extended_buf;
  struct AVFrameSideData **side_data;
  int32_t nb_side_data;
  int32_t flags;
  AVColorRange color_range;
  AVColorPrimaries color_primaries;
  AVColorTransferCharacteristic color_trc;
  AVColorSpace color_space;
  AVChromaLocation chroma_location;
  int64_t best_effort_timestamp;
  int64_t pkt_pos;
  int64_t pkt_duration;
  struct AVDictionary *metadata;
  int32_t decode_error_flags;
  int32_t channels;
  int32_t pkt_size;
  struct AVBufferRef *hw_frames_ctx;
  struct AVBufferRef *opaque_ref;
  uintptr_t crop_top;
  uintptr_t crop_bottom;
  uintptr_t crop_left;
  uintptr_t crop_right;
  struct AVBufferRef *private_ref;
} AVFrame;

typedef uint32_t AVFieldOrder;

typedef int32_t AVSampleFormat;

typedef uint32_t AVAudioServiceType;

typedef struct RcOverride {
  int32_t start_frame;
  int32_t end_frame;
  int32_t qscale;
  float quality_factor;
} RcOverride;

typedef uint32_t AVCodecID;

typedef struct AVHWAccel {
  const char *name;
  AVMediaType type_;
  AVCodecID id;
  AVPixelFormat pix_fmt;
  int32_t capabilities;
  int32_t (*alloc_frame)(struct AVCodecContext *avctx, struct AVFrame *frame);
  int32_t (*start_frame)(struct AVCodecContext *avctx, const uint8_t *buf, uint32_t buf_size);
  int32_t (*decode_params)(struct AVCodecContext *avctx, int32_t type_, const uint8_t *buf, uint32_t buf_size);
  int32_t (*decode_slice)(struct AVCodecContext *avctx, const uint8_t *buf, uint32_t buf_size);
  int32_t (*end_frame)(struct AVCodecContext *avctx);
  int32_t frame_priv_data_size;
  void (*decode_mb)(struct MpegEncContext *s);
  int32_t (*init)(struct AVCodecContext *avctx);
  int32_t (*uninit)(struct AVCodecContext *avctx);
  int32_t priv_data_size;
  int32_t caps_internal;
  int32_t (*frame_params)(struct AVCodecContext *avctx, struct AVBufferRef *hw_frames_ctx);
} AVHWAccel;

typedef int32_t AVDiscard;

typedef struct AVProfile {
  int32_t profile;
  const char *name;
} AVProfile;

typedef struct AVCodecDescriptor {
  AVCodecID id;
  AVMediaType type_;
  const char *name;
  const char *long_name;
  int32_t props;
  const char *const *mime_types;
  const struct AVProfile *const *profiles;
} AVCodecDescriptor;

typedef uint32_t AVPacketSideDataType;

typedef struct AVPacketSideData {
  uint8_t *data;
  uintptr_t size;
  AVPacketSideDataType typ;
} AVPacketSideData;

typedef struct AVPacket {
  struct AVBufferRef *buf;
  int64_t pts;
  int64_t dts;
  uint8_t *data;
  int32_t size;
  int32_t stream_index;
  int32_t flags;
  struct AVPacketSideData *side_data;
  int32_t side_data_elems;
  int64_t duration;
  int64_t pos;
  void *opaque;
  struct AVBufferRef *opaque_ref;
  struct AVRational time_base;
} AVPacket;

typedef struct AVCodecContext {
  const struct AVClass *av_class;
  int32_t log_level_offset;
  AVMediaType codec_type;
  const void *codec;
  int32_t codec_id;
  uint32_t codec_tag;
  void *priv_data;
  struct AVCodecInternal *internal;
  void *opaque;
  int64_t bit_rate;
  int32_t bit_rate_tolerance;
  int32_t global_quality;
  int32_t compression_level;
  int32_t flags;
  int32_t flags2;
  uint8_t *extradata;
  int32_t extradata_size;
  struct AVRational time_base;
  int32_t ticks_per_frame;
  int32_t delay;
  int32_t width;
  int32_t height;
  int32_t coded_width;
  int32_t coded_height;
  int32_t gop_size;
  AVPixelFormat pix_fmt;
  void (*draw_horiz_band)(struct AVCodecContext *s, const struct AVFrame *src, int32_t offset[AV_NUM_DATA_POINTERS], int32_t y, int32_t type_, int32_t height);
  AVPixelFormat (*get_format)(struct AVCodecContext *s, const AVPixelFormat *fmt);
  int32_t max_b_frames;
  float b_quant_factor;
  float b_quant_offset;
  int32_t has_b_frames;
  float i_quant_factor;
  float i_quant_offset;
  float lumi_masking;
  float temporal_cplx_masking;
  float spatial_cplx_masking;
  float p_masking;
  float dark_masking;
  int32_t slice_count;
  int32_t *slice_offset;
  struct AVRational sample_aspect_ratio;
  int32_t me_cmp;
  int32_t me_sub_cmp;
  int32_t mb_cmp;
  int32_t ildct_cmp;
  int32_t dia_size;
  int32_t last_predictor_count;
  int32_t me_pre_cmp;
  int32_t pre_dia_size;
  int32_t me_subpel_quality;
  int32_t me_range;
  int32_t slice_flags;
  int32_t mb_decision;
  uint16_t *intra_matrix;
  uint16_t *inter_matrix;
  int32_t intra_dc_precision;
  int32_t skip_top;
  int32_t skip_bottom;
  int32_t mb_lmin;
  int32_t mb_lmax;
  int32_t bidir_refine;
  int32_t keyint_min;
  int32_t refs;
  int32_t mv0_threshold;
  AVColorPrimaries color_primaries;
  AVColorTransferCharacteristic color_trc;
  AVColorSpace colorspace;
  AVColorRange color_range;
  AVChromaLocation chroma_sample_location;
  int32_t slices;
  AVFieldOrder field_order;
  int32_t sample_rate;
  int32_t channels;
  AVSampleFormat sample_fmt;
  int32_t frame_size;
  int32_t frame_number;
  int32_t block_align;
  int32_t cutoff;
  uint64_t channel_layout;
  uint64_t request_channel_layout;
  AVAudioServiceType audio_service_type;
  AVSampleFormat request_sample_fmt;
  int32_t (*get_buffer2)(struct AVCodecContext *s, struct AVFrame *frame, int32_t flags);
  float qcompress;
  float qblur;
  int32_t qmin;
  int32_t qmax;
  int32_t max_qdiff;
  int32_t rc_buffer_size;
  int32_t rc_override_count;
  struct RcOverride *rc_override;
  int64_t rc_max_rate;
  int64_t rc_min_rate;
  float rc_max_available_vbv_use;
  float rc_min_vbv_overflow_use;
  int32_t rc_initial_buffer_occupancy;
  int32_t trellis;
  char *stats_out;
  char *stats_in;
  int32_t workaround_bugs;
  int32_t strict_std_compliance;
  int32_t error_concealment;
  int32_t debug;
  int32_t err_recognition;
  int64_t reordered_opaque;
  const struct AVHWAccel *hwaccel;
  void *hwaccel_context;
  uint64_t error[AV_NUM_DATA_POINTERS];
  int32_t dct_algo;
  int32_t idct_algo;
  int32_t bits_per_coded_sample;
  int32_t bits_per_raw_sample;
  int32_t lowres;
  int32_t thread_count;
  int32_t thread_type;
  int32_t active_thread_type;
  int32_t thread_safe_callbacks;
  int32_t (*execute)(struct AVCodecContext *c, void (*func)(struct AVCodecContext*, void*), void *arg2, int32_t *ret, int32_t count, int32_t size);
  int32_t (*execute2)(struct AVCodecContext *c, void (*func)(struct AVCodecContext*, void*, int32_t, int32_t), void *arg2, int32_t *ret, int32_t count);
  int32_t nsse_wight;
  int32_t profile;
  int32_t level;
  AVDiscard skip_loop_filter;
  AVDiscard skip_idct;
  AVDiscard skip_frame;
  uint8_t *subtitle_header;
  int32_t subtitle_header_size;
  int32_t initial_padding;
  struct AVRational framerate;
  AVPixelFormat sw_pix_fmt;
  struct AVRational pkt_timebase;
  const struct AVCodecDescriptor *codec_descriptor;
  int64_t pts_correction_num_faulty_pts;
  int64_t pts_correction_num_faulty_dts;
  int64_t pts_correction_last_pts;
  int64_t pts_correction_last_dts;
  char *sub_charenc;
  int32_t sub_charenc_mode;
  int32_t skip_alpha;
  int32_t seek_preroll;
  int32_t debug_mv;
  uint8_t *chroma_intra_matrix;
  uint8_t *dump_separator;
  char *codec_whitelist;
  uint32_t properties;
  struct AVPacketSideData *coded_side_data;
  int32_t nb_codec_side_data;
  struct AVBufferRef *hw_frames_ctx;
  int32_t sub_text_format;
  int32_t trailing_padding;
  int64_t max_pixels;
  struct AVBufferRef *hw_device_ctx;
  int32_t hwaccel_flags;
  int32_t apply_cropping;
  int32_t extra_hw_frames;
  int32_t discard_damaged_percentage;
  int64_t max_samples;
  int32_t export_side_data;
  int32_t (*get_encode_buffer)(struct AVCodecContext *s, struct AVPacket *pkt, int32_t flags);
} AVCodecContext;

typedef uint32_t AVPictureStructure;

typedef struct AVCodecParserContext {
  void *priv_data;
  const void *parser;
  int64_t frame_offset;
  int64_t cur_offset;
  int64_t next_frame_offset;
  AVPixelFormat pict_type;
  int32_t repeat_pict;
  int64_t pts;
  int64_t dts;
  int64_t last_pts;
  int64_t last_dts;
  int32_t fetch_timestamp;
  int32_t cur_frame_start_index;
  int64_t cur_frame_offset[AV_PARSER_PTS_NB];
  int64_t cur_frame_pts[AV_PARSER_PTS_NB];
  int64_t cur_frame_dts[AV_PARSER_PTS_NB];
  int32_t flags;
  int64_t offset;
  int64_t cur_frame_end[AV_PARSER_PTS_NB];
  int32_t key_frame;
  int32_t dts_sync_point;
  int32_t dts_ref_dts_delta;
  int32_t pts_dts_delta;
  int64_t cur_frame_pos[AV_PARSER_PTS_NB];
  int64_t pos;
  int64_t last_pos;
  int32_t duration;
  AVFieldOrder field_order;
  AVPictureStructure picture_structure;
  int32_t output_picture_number;
  int32_t width;
  int32_t height;
  int32_t coded_width;
  int32_t coded_height;
  int32_t format;
} AVCodecParserContext;

typedef uint32_t AVSubtitleType;

typedef struct AVSubtitleRect {
  int32_t x;
  int32_t y;
  int32_t w;
  int32_t h;
  int32_t nb_colors;
  uint8_t *data[4];
  int32_t linesize[4];
  AVSubtitleType type_;
  char *text;
  char *ass;
  int32_t flags;
} AVSubtitleRect;

typedef struct AVSubtitle {
  uint16_t format;
  uint32_t start_display_time;
  uint32_t end_display_time;
  uint32_t num_rects;
  struct AVSubtitleRect **rects;
  int64_t pts;
} AVSubtitle;

typedef struct AVCodec {
  const char *name;
  const char *long_name;
  AVMediaType type_;
  AVCodecID id;
  int32_t capabilities;
  uint8_t max_lowres;
  const struct AVRational *supported_framerates;
  const AVPixelFormat *pix_fmts;
  const int32_t *supported_samplerates;
  const AVSampleFormat *sample_fmts;
  const uint64_t *channel_layouts;
  const struct AVClass *priv_class;
  const struct AVProfile *profiles;
  const char *wrapper_name;
  int32_t caps_internal;
  int32_t priv_data_size;
  int32_t (*update_thread_context)(struct AVCodecContext *dst, const struct AVCodecContext *src);
  int32_t (*update_thread_context_for_user)(struct AVCodecContext *dst, const struct AVCodecContext *src);
  const struct AVCodecDefault *defaults;
  void (*init_static_data)(struct AVCodec *codec);
  int32_t (*init)(struct AVCodecContext *codec);
  int32_t (*encode_sub)(struct AVCodecContext *avctx, uint8_t *buf, int32_t buf_size, const struct AVSubtitle *sub);
  int32_t (*encode2)(struct AVCodecContext *avctx, struct AVPacket *avpkt, const struct AVFrame *frame, int32_t *got_packet_ptr);
  int32_t (*decode)(struct AVCodecContext *avctx, void *outdata, int32_t *got_frame_ptr, struct AVPacket *avpkt);
  int32_t (*close)(struct AVCodecContext *avctx);
  int32_t (*receive_packet)(struct AVCodecContext *avctx, struct AVPacket *avpkt);
  int32_t (*receive_frame)(struct AVCodecContext *avctx, struct AVFrame *frame);
  int32_t (*flush)(struct AVCodecContext *avctx);
  const char *bsfs;
  const void *const *hw_configs;
  const uint32_t *codec_tags;
} AVCodec;

typedef uint32_t AVHWDeviceType;

typedef struct AVCodecHWConfig {
  AVPixelFormat pix_fmt;
  int32_t methods;
  AVHWDeviceType device_type;
} AVCodecHWConfig;

typedef int64_t CMTimeValue;

typedef int32_t CMTimeScale;

typedef uint32_t CMTimeFlags;

typedef int64_t CMTimeEpoch;

typedef struct CMTime {
  CMTimeValue value;
  CMTimeScale time_scale;
  CMTimeFlags flags;
  CMTimeEpoch epoch;
} CMTime;

typedef void *CMSampleBufferRef;

typedef struct CMSampleTimingInfo {
  struct CMTime duration;
  struct CMTime presentation_timestamp;
  struct CMTime decode_timestamp;
} CMSampleTimingInfo;

typedef void *CVImageBufferRef;

typedef CVImageBufferRef CVPixelBufferRef;

#define AV_PICTURE_STRUCTURE_UNKNOWN 0

#define AV_PICTURE_STRUCTURE_TOP_FIELD 1

#define AV_PICTURE_STRUCTURE_BOTTOM_FIELD 2

#define AV_PICTURE_STRUCTURE_FRAME 3

#define SUBTITLE_NONE 0

#define SUBTITLE_BITMAP 1

#define SUBTITLE_TEXT 2

#define SUBTITLE_ASS 3

#define AV_CODEC_ID_NONE 0

#define AV_CODEC_ID_MPEG1VIDEO 1

#define AV_CODEC_ID_MPEG2VIDEO 2

#define AV_CODEC_ID_H261 3

#define AV_CODEC_ID_H263 4

#define AV_CODEC_ID_RV10 5

#define AV_CODEC_ID_RV20 6

#define AV_CODEC_ID_MJPEG 7

#define AV_CODEC_ID_MJPEGB 8

#define AV_CODEC_ID_LJPEG 9

#define AV_CODEC_ID_SP5X 10

#define AV_CODEC_ID_JPEGLS 11

#define AV_CODEC_ID_MPEG4 12

#define AV_CODEC_ID_RAWVIDEO 13

#define AV_CODEC_ID_MSMPEG4V1 14

#define AV_CODEC_ID_MSMPEG4V2 15

#define AV_CODEC_ID_MSMPEG4V3 16

#define AV_CODEC_ID_WMV1 17

#define AV_CODEC_ID_WMV2 18

#define AV_CODEC_ID_H263P 19

#define AV_CODEC_ID_H263I 20

#define AV_CODEC_ID_FLV1 21

#define AV_CODEC_ID_SVQ1 22

#define AV_CODEC_ID_SVQ3 23

#define AV_CODEC_ID_DVVIDEO 24

#define AV_CODEC_ID_HUFFYUV 25

#define AV_CODEC_ID_CYUV 26

#define AV_CODEC_ID_H264 27

#define AV_CODEC_ID_INDEO3 28

#define AV_CODEC_ID_VP3 29

#define AV_CODEC_ID_THEORA 30

#define AV_CODEC_ID_ASV1 31

#define AV_CODEC_ID_ASV2 32

#define AV_CODEC_ID_FFV1 33

#define AV_CODEC_ID_4XM 34

#define AV_CODEC_ID_VCR1 35

#define AV_CODEC_ID_CLJR 36

#define AV_CODEC_ID_MDEC 37

#define AV_CODEC_ID_ROQ 38

#define AV_CODEC_ID_INTERPLAY_VIDEO 39

#define AV_CODEC_ID_XAN_WC3 40

#define AV_CODEC_ID_XAN_WC4 41

#define AV_CODEC_ID_RPZA 42

#define AV_CODEC_ID_CINEPAK 43

#define AV_CODEC_ID_WS_VQA 44

#define AV_CODEC_ID_MSRLE 45

#define AV_CODEC_ID_MSVIDEO1 46

#define AV_CODEC_ID_IDCIN 47

#define AV_CODEC_ID_8BPS 48

#define AV_CODEC_ID_SMC 49

#define AV_CODEC_ID_FLIC 50

#define AV_CODEC_ID_TRUEMOTION1 51

#define AV_CODEC_ID_VMDVIDEO 52

#define AV_CODEC_ID_MSZH 53

#define AV_CODEC_ID_ZLIB 54

#define AV_CODEC_ID_QTRLE 55

#define AV_CODEC_ID_TSCC 56

#define AV_CODEC_ID_ULTI 57

#define AV_CODEC_ID_QDRAW 58

#define AV_CODEC_ID_VIXL 59

#define AV_CODEC_ID_QPEG 60

#define AV_CODEC_ID_PNG 61

#define AV_CODEC_ID_PPM 62

#define AV_CODEC_ID_PBM 63

#define AV_CODEC_ID_PGM 64

#define AV_CODEC_ID_PGMYUV 65

#define AV_CODEC_ID_PAM 66

#define AV_CODEC_ID_FFVHUFF 67

#define AV_CODEC_ID_RV30 68

#define AV_CODEC_ID_RV40 69

#define AV_CODEC_ID_VC1 70

#define AV_CODEC_ID_WMV3 71

#define AV_CODEC_ID_LOCO 72

#define AV_CODEC_ID_WNV1 73

#define AV_CODEC_ID_AASC 74

#define AV_CODEC_ID_INDEO2 75

#define AV_CODEC_ID_FRAPS 76

#define AV_CODEC_ID_TRUEMOTION2 77

#define AV_CODEC_ID_BMP 78

#define AV_CODEC_ID_CSCD 79

#define AV_CODEC_ID_MMVIDEO 80

#define AV_CODEC_ID_ZMBV 81

#define AV_CODEC_ID_AVS 82

#define AV_CODEC_ID_SMACKVIDEO 83

#define AV_CODEC_ID_NUV 84

#define AV_CODEC_ID_KMVC 85

#define AV_CODEC_ID_FLASHSV 86

#define AV_CODEC_ID_CAVS 87

#define AV_CODEC_ID_JPEG2000 88

#define AV_CODEC_ID_VMNC 89

#define AV_CODEC_ID_VP5 90

#define AV_CODEC_ID_VP6 91

#define AV_CODEC_ID_VP6F 92

#define AV_CODEC_ID_TARGA 93

#define AV_CODEC_ID_DSICINVIDEO 94

#define AV_CODEC_ID_TIERTEXSEQVIDEO 95

#define AV_CODEC_ID_TIFF 96

#define AV_CODEC_ID_GIF 97

#define AV_CODEC_ID_DXA 98

#define AV_CODEC_ID_DNXHD 99

#define AV_CODEC_ID_THP 100

#define AV_CODEC_ID_SGI 101

#define AV_CODEC_ID_C93 102

#define AV_CODEC_ID_BETHSOFTVID 103

#define AV_CODEC_ID_PTX 104

#define AV_CODEC_ID_TXD 105

#define AV_CODEC_ID_VP6A 106

#define AV_CODEC_ID_AMV 107

#define AV_CODEC_ID_VB 108

#define AV_CODEC_ID_PCX 109

#define AV_CODEC_ID_SUNRAST 110

#define AV_CODEC_ID_INDEO4 111

#define AV_CODEC_ID_INDEO5 112

#define AV_CODEC_ID_MIMIC 113

#define AV_CODEC_ID_RL2 114

#define AV_CODEC_ID_ESCAPE124 115

#define AV_CODEC_ID_DIRAC 116

#define AV_CODEC_ID_BFI 117

#define AV_CODEC_ID_CMV 118

#define AV_CODEC_ID_MOTIONPIXELS 119

#define AV_CODEC_ID_TGV 120

#define AV_CODEC_ID_TGQ 121

#define AV_CODEC_ID_TQI 122

#define AV_CODEC_ID_AURA 123

#define AV_CODEC_ID_AURA2 124

#define AV_CODEC_ID_V210X 125

#define AV_CODEC_ID_TMV 126

#define AV_CODEC_ID_V210 127

#define AV_CODEC_ID_DPX 128

#define AV_CODEC_ID_MAD 129

#define AV_CODEC_ID_FRWU 130

#define AV_CODEC_ID_FLASHSV2 131

#define AV_CODEC_ID_CDGRAPHICS 132

#define AV_CODEC_ID_R210 133

#define AV_CODEC_ID_ANM 134

#define AV_CODEC_ID_BINKVIDEO 135

#define AV_CODEC_ID_IFF_ILBM 136

#define AV_CODEC_ID_IFF_BYTERUN1 AV_CODEC_ID_IFF_ILBM

#define AV_CODEC_ID_KGV1 137

#define AV_CODEC_ID_YOP 138

#define AV_CODEC_ID_VP8 139

#define AV_CODEC_ID_PICTOR 140

#define AV_CODEC_ID_ANSI 141

#define AV_CODEC_ID_A64_MULTI 142

#define AV_CODEC_ID_A64_MULTI5 143

#define AV_CODEC_ID_R10K 144

#define AV_CODEC_ID_MXPEG 145

#define AV_CODEC_ID_LAGARITH 146

#define AV_CODEC_ID_PRORES 147

#define AV_CODEC_ID_JV 148

#define AV_CODEC_ID_DFA 149

#define AV_CODEC_ID_WMV3IMAGE 150

#define AV_CODEC_ID_VC1IMAGE 151

#define AV_CODEC_ID_UTVIDEO 152

#define AV_CODEC_ID_BMV_VIDEO 153

#define AV_CODEC_ID_VBLE 154

#define AV_CODEC_ID_DXTORY 155

#define AV_CODEC_ID_V410 156

#define AV_CODEC_ID_XWD 157

#define AV_CODEC_ID_CDXL 158

#define AV_CODEC_ID_XBM 159

#define AV_CODEC_ID_ZEROCODEC 160

#define AV_CODEC_ID_MSS1 161

#define AV_CODEC_ID_MSA1 162

#define AV_CODEC_ID_TSCC2 163

#define AV_CODEC_ID_MTS2 164

#define AV_CODEC_ID_CLLC 165

#define AV_CODEC_ID_MSS2 166

#define AV_CODEC_ID_VP9 167

#define AV_CODEC_ID_AIC 168

#define AV_CODEC_ID_ESCAPE130 169

#define AV_CODEC_ID_G2M 170

#define AV_CODEC_ID_WEBP 171

#define AV_CODEC_ID_HNM4_VIDEO 172

#define AV_CODEC_ID_HEVC 173

#define AV_CODEC_ID_H265 AV_CODEC_ID_HEVC

#define AV_CODEC_ID_FIC 174

#define AV_CODEC_ID_ALIAS_PIX 175

#define AV_CODEC_ID_BRENDER_PIX 176

#define AV_CODEC_ID_PAF_VIDEO 177

#define AV_CODEC_ID_EXR 178

#define AV_CODEC_ID_VP7 179

#define AV_CODEC_ID_SANM 180

#define AV_CODEC_ID_SGIRLE 181

#define AV_CODEC_ID_MVC1 182

#define AV_CODEC_ID_MVC2 183

#define AV_CODEC_ID_HQX 184

#define AV_CODEC_ID_TDSC 185

#define AV_CODEC_ID_HQ_HQA 186

#define AV_CODEC_ID_HAP 187

#define AV_CODEC_ID_DDS 188

#define AV_CODEC_ID_DXV 189

#define AV_CODEC_ID_SCREENPRESSO 190

#define AV_CODEC_ID_RSCC 191

#define AV_CODEC_ID_AVS2 192

#define AV_CODEC_ID_PGX 193

#define AV_CODEC_ID_AVS3 194

#define AV_CODEC_ID_MSP2 195

#define AV_CODEC_ID_VVC 196

#define AV_CODEC_ID_H266 AV_CODEC_ID_VVC

#define AV_CODEC_ID_Y41P 197

#define AV_CODEC_ID_AVRP 198

#define AV_CODEC_ID_012V 199

#define AV_CODEC_ID_AVUI 200

#define AV_CODEC_ID_AYUV 201

#define AV_CODEC_ID_TARGA_Y216 202

#define AV_CODEC_ID_V308 203

#define AV_CODEC_ID_V408 204

#define AV_CODEC_ID_YUV4 205

#define AV_CODEC_ID_AVRN 206

#define AV_CODEC_ID_CPIA 207

#define AV_CODEC_ID_XFACE 208

#define AV_CODEC_ID_SNOW 209

#define AV_CODEC_ID_SMVJPEG 210

#define AV_CODEC_ID_APNG 211

#define AV_CODEC_ID_DAALA 212

#define AV_CODEC_ID_CFHD 213

#define AV_CODEC_ID_TRUEMOTION2RT 214

#define AV_CODEC_ID_M101 215

#define AV_CODEC_ID_MAGICYUV 216

#define AV_CODEC_ID_SHEERVIDEO 217

#define AV_CODEC_ID_YLC 218

#define AV_CODEC_ID_PSD 219

#define AV_CODEC_ID_PIXLET 220

#define AV_CODEC_ID_SPEEDHQ 221

#define AV_CODEC_ID_FMVC 222

#define AV_CODEC_ID_SCPR 223

#define AV_CODEC_ID_CLEARVIDEO 224

#define AV_CODEC_ID_XPM 225

#define AV_CODEC_ID_AV1 226

#define AV_CODEC_ID_BITPACKED 227

#define AV_CODEC_ID_MSCC 228

#define AV_CODEC_ID_SRGC 229

#define AV_CODEC_ID_SVG 230

#define AV_CODEC_ID_GDV 231

#define AV_CODEC_ID_FITS 232

#define AV_CODEC_ID_IMM4 233

#define AV_CODEC_ID_PROSUMER 234

#define AV_CODEC_ID_MWSC 235

#define AV_CODEC_ID_WCMV 236

#define AV_CODEC_ID_RASC 237

#define AV_CODEC_ID_HYMT 238

#define AV_CODEC_ID_ARBC 239

#define AV_CODEC_ID_AGM 240

#define AV_CODEC_ID_LSCR 241

#define AV_CODEC_ID_VP4 242

#define AV_CODEC_ID_IMM5 243

#define AV_CODEC_ID_MVDV 244

#define AV_CODEC_ID_MVHA 245

#define AV_CODEC_ID_CDTOONS 246

#define AV_CODEC_ID_MV30 247

#define AV_CODEC_ID_NOTCHLC 248

#define AV_CODEC_ID_PFM 249

#define AV_CODEC_ID_MOBICLIP 250

#define AV_CODEC_ID_PHOTOCD 251

#define AV_CODEC_ID_IPU 252

#define AV_CODEC_ID_ARGO 253

#define AV_CODEC_ID_CRI 254

#define AV_CODEC_ID_SIMBIOSIS_IMX 255

#define AV_CODEC_ID_SGA_VIDEO 256

#define AV_CODEC_ID_GEM 257

#define AV_CODEC_ID_FIRST_AUDIO 65536

#define AV_CODEC_ID_PCM_S16LE 65536

#define AV_CODEC_ID_PCM_S16BE 65537

#define AV_CODEC_ID_PCM_U16LE 65538

#define AV_CODEC_ID_PCM_U16BE 65539

#define AV_CODEC_ID_PCM_S8 65540

#define AV_CODEC_ID_PCM_U8 65541

#define AV_CODEC_ID_PCM_MULAW 65542

#define AV_CODEC_ID_PCM_ALAW 65543

#define AV_CODEC_ID_PCM_S32LE 65544

#define AV_CODEC_ID_PCM_S32BE 65545

#define AV_CODEC_ID_PCM_U32LE 65546

#define AV_CODEC_ID_PCM_U32BE 65547

#define AV_CODEC_ID_PCM_S24LE 65548

#define AV_CODEC_ID_PCM_S24BE 65549

#define AV_CODEC_ID_PCM_U24LE 65550

#define AV_CODEC_ID_PCM_U24BE 65551

#define AV_CODEC_ID_PCM_S24DAUD 65552

#define AV_CODEC_ID_PCM_ZORK 65553

#define AV_CODEC_ID_PCM_S16LE_PLANAR 65554

#define AV_CODEC_ID_PCM_DVD 65555

#define AV_CODEC_ID_PCM_F32BE 65556

#define AV_CODEC_ID_PCM_F32LE 65557

#define AV_CODEC_ID_PCM_F64BE 65558

#define AV_CODEC_ID_PCM_F64LE 65559

#define AV_CODEC_ID_PCM_BLURAY 65560

#define AV_CODEC_ID_PCM_LXF 65561

#define AV_CODEC_ID_S302M 65562

#define AV_CODEC_ID_PCM_S8_PLANAR 65563

#define AV_CODEC_ID_PCM_S24LE_PLANAR 65564

#define AV_CODEC_ID_PCM_S32LE_PLANAR 65565

#define AV_CODEC_ID_PCM_S16BE_PLANAR 65566

#define AV_CODEC_ID_PCM_S64LE 65567

#define AV_CODEC_ID_PCM_S64BE 65568

#define AV_CODEC_ID_PCM_F16LE 65569

#define AV_CODEC_ID_PCM_F24LE 65570

#define AV_CODEC_ID_PCM_VIDC 65571

#define AV_CODEC_ID_PCM_SGA 65572

#define AV_CODEC_ID_ADPCM_IMA_QT 69632

#define AV_CODEC_ID_ADPCM_IMA_WAV 69633

#define AV_CODEC_ID_ADPCM_IMA_DK3 69634

#define AV_CODEC_ID_ADPCM_IMA_DK4 69635

#define AV_CODEC_ID_ADPCM_IMA_WS 69636

#define AV_CODEC_ID_ADPCM_IMA_SMJPEG 69637

#define AV_CODEC_ID_ADPCM_MS 69638

#define AV_CODEC_ID_ADPCM_4XM 69639

#define AV_CODEC_ID_ADPCM_XA 69640

#define AV_CODEC_ID_ADPCM_ADX 69641

#define AV_CODEC_ID_ADPCM_EA 69642

#define AV_CODEC_ID_ADPCM_G726 69643

#define AV_CODEC_ID_ADPCM_CT 69644

#define AV_CODEC_ID_ADPCM_SWF 69645

#define AV_CODEC_ID_ADPCM_YAMAHA 69646

#define AV_CODEC_ID_ADPCM_SBPRO_4 69647

#define AV_CODEC_ID_ADPCM_SBPRO_3 69648

#define AV_CODEC_ID_ADPCM_SBPRO_2 69649

#define AV_CODEC_ID_ADPCM_THP 69650

#define AV_CODEC_ID_ADPCM_IMA_AMV 69651

#define AV_CODEC_ID_ADPCM_EA_R1 69652

#define AV_CODEC_ID_ADPCM_EA_R3 69653

#define AV_CODEC_ID_ADPCM_EA_R2 69654

#define AV_CODEC_ID_ADPCM_IMA_EA_SEAD 69655

#define AV_CODEC_ID_ADPCM_IMA_EA_EACS 69656

#define AV_CODEC_ID_ADPCM_EA_XAS 69657

#define AV_CODEC_ID_ADPCM_EA_MAXIS_XA 69658

#define AV_CODEC_ID_ADPCM_IMA_ISS 69659

#define AV_CODEC_ID_ADPCM_G722 69660

#define AV_CODEC_ID_ADPCM_IMA_APC 69661

#define AV_CODEC_ID_ADPCM_VIMA 69662

#define AV_CODEC_ID_ADPCM_AFC 69663

#define AV_CODEC_ID_ADPCM_IMA_OKI 69664

#define AV_CODEC_ID_ADPCM_DTK 69665

#define AV_CODEC_ID_ADPCM_IMA_RAD 69666

#define AV_CODEC_ID_ADPCM_G726LE 69667

#define AV_CODEC_ID_ADPCM_THP_LE 69668

#define AV_CODEC_ID_ADPCM_PSX 69669

#define AV_CODEC_ID_ADPCM_AICA 69670

#define AV_CODEC_ID_ADPCM_IMA_DAT4 69671

#define AV_CODEC_ID_ADPCM_MTAF 69672

#define AV_CODEC_ID_ADPCM_AGM 69673

#define AV_CODEC_ID_ADPCM_ARGO 69674

#define AV_CODEC_ID_ADPCM_IMA_SSI 69675

#define AV_CODEC_ID_ADPCM_ZORK 69676

#define AV_CODEC_ID_ADPCM_IMA_APM 69677

#define AV_CODEC_ID_ADPCM_IMA_ALP 69678

#define AV_CODEC_ID_ADPCM_IMA_MTF 69679

#define AV_CODEC_ID_ADPCM_IMA_CUNNING 69680

#define AV_CODEC_ID_ADPCM_IMA_MOFLEX 69681

#define AV_CODEC_ID_ADPCM_IMA_ACORN 69682

#define AV_CODEC_ID_AMR_NB 73728

#define AV_CODEC_ID_AMR_WB 73729

#define AV_CODEC_ID_RA_144 77824

#define AV_CODEC_ID_RA_288 77825

#define AV_CODEC_ID_ROQ_DPCM 81920

#define AV_CODEC_ID_INTERPLAY_DPCM 81921

#define AV_CODEC_ID_XAN_DPCM 81922

#define AV_CODEC_ID_SOL_DPCM 81923

#define AV_CODEC_ID_SDX2_DPCM 81924

#define AV_CODEC_ID_GREMLIN_DPCM 81925

#define AV_CODEC_ID_DERF_DPCM 81926

#define AV_CODEC_ID_MP2 86016

#define AV_CODEC_ID_MP3 86017

#define AV_CODEC_ID_AAC 86018

#define AV_CODEC_ID_AC3 86019

#define AV_CODEC_ID_DTS 86020

#define AV_CODEC_ID_VORBIS 86021

#define AV_CODEC_ID_DVAUDIO 86022

#define AV_CODEC_ID_WMAV1 86023

#define AV_CODEC_ID_WMAV2 86024

#define AV_CODEC_ID_MACE3 86025

#define AV_CODEC_ID_MACE6 86026

#define AV_CODEC_ID_VMDAUDIO 86027

#define AV_CODEC_ID_FLAC 86028

#define AV_CODEC_ID_MP3ADU 86029

#define AV_CODEC_ID_MP3ON4 86030

#define AV_CODEC_ID_SHORTEN 86031

#define AV_CODEC_ID_ALAC 86032

#define AV_CODEC_ID_WESTWOOD_SND1 86033

#define AV_CODEC_ID_GSM 86034

#define AV_CODEC_ID_QDM2 86035

#define AV_CODEC_ID_COOK 86036

#define AV_CODEC_ID_TRUESPEECH 86037

#define AV_CODEC_ID_TTA 86038

#define AV_CODEC_ID_SMACKAUDIO 86039

#define AV_CODEC_ID_QCELP 86040

#define AV_CODEC_ID_WAVPACK 86041

#define AV_CODEC_ID_DSICINAUDIO 86042

#define AV_CODEC_ID_IMC 86043

#define AV_CODEC_ID_MUSEPACK7 86044

#define AV_CODEC_ID_MLP 86045

#define AV_CODEC_ID_GSM_MS 86046

#define AV_CODEC_ID_ATRAC3 86047

#define AV_CODEC_ID_APE 86048

#define AV_CODEC_ID_NELLYMOSER 86049

#define AV_CODEC_ID_MUSEPACK8 86050

#define AV_CODEC_ID_SPEEX 86051

#define AV_CODEC_ID_WMAVOICE 86052

#define AV_CODEC_ID_WMAPRO 86053

#define AV_CODEC_ID_WMALOSSLESS 86054

#define AV_CODEC_ID_ATRAC3P 86055

#define AV_CODEC_ID_EAC3 86056

#define AV_CODEC_ID_SIPR 86057

#define AV_CODEC_ID_MP1 86058

#define AV_CODEC_ID_TWINVQ 86059

#define AV_CODEC_ID_TRUEHD 86060

#define AV_CODEC_ID_MP4ALS 86061

#define AV_CODEC_ID_ATRAC1 86062

#define AV_CODEC_ID_BINKAUDIO_RDFT 86063

#define AV_CODEC_ID_BINKAUDIO_DCT 86064

#define AV_CODEC_ID_AAC_LATM 86065

#define AV_CODEC_ID_QDMC 86066

#define AV_CODEC_ID_CELT 86067

#define AV_CODEC_ID_G723_1 86068

#define AV_CODEC_ID_G729 86069

#define AV_CODEC_ID_8SVX_EXP 86070

#define AV_CODEC_ID_8SVX_FIB 86071

#define AV_CODEC_ID_BMV_AUDIO 86072

#define AV_CODEC_ID_RALF 86073

#define AV_CODEC_ID_IAC 86074

#define AV_CODEC_ID_ILBC 86075

#define AV_CODEC_ID_OPUS 86076

#define AV_CODEC_ID_COMFORT_NOISE 86077

#define AV_CODEC_ID_TAK 86078

#define AV_CODEC_ID_METASOUND 86079

#define AV_CODEC_ID_PAF_AUDIO 86080

#define AV_CODEC_ID_ON2AVC 86081

#define AV_CODEC_ID_DSS_SP 86082

#define AV_CODEC_ID_CODEC2 86083

#define AV_CODEC_ID_FFWAVESYNTH 86084

#define AV_CODEC_ID_SONIC 86085

#define AV_CODEC_ID_SONIC_LS 86086

#define AV_CODEC_ID_EVRC 86087

#define AV_CODEC_ID_SMV 86088

#define AV_CODEC_ID_DSD_LSBF 86089

#define AV_CODEC_ID_DSD_MSBF 86090

#define AV_CODEC_ID_DSD_LSBF_PLANAR 86091

#define AV_CODEC_ID_DSD_MSBF_PLANAR 86092

#define AV_CODEC_ID_4GV 86093

#define AV_CODEC_ID_INTERPLAY_ACM 86094

#define AV_CODEC_ID_XMA1 86095

#define AV_CODEC_ID_XMA2 86096

#define AV_CODEC_ID_DST 86097

#define AV_CODEC_ID_ATRAC3AL 86098

#define AV_CODEC_ID_ATRAC3PAL 86099

#define AV_CODEC_ID_DOLBY_E 86100

#define AV_CODEC_ID_APTX 86101

#define AV_CODEC_ID_APTX_HD 86102

#define AV_CODEC_ID_SBC 86103

#define AV_CODEC_ID_ATRAC9 86104

#define AV_CODEC_ID_HCOM 86105

#define AV_CODEC_ID_ACELP_KELVIN 86106

#define AV_CODEC_ID_MPEGH_3D_AUDIO 86107

#define AV_CODEC_ID_SIREN 86108

#define AV_CODEC_ID_HCA 86109

#define AV_CODEC_ID_FASTAUDIO 86110

#define AV_CODEC_ID_MSNSIREN 86111

#define AV_CODEC_ID_FIRST_SUBTITLE 94208

#define AV_CODEC_ID_DVB_SUBTITLE 94209

#define AV_CODEC_ID_TEXT 94210

#define AV_CODEC_ID_XSUB 94211

#define AV_CODEC_ID_SSA 94212

#define AV_CODEC_ID_MOV_TEXT 94213

#define AV_CODEC_ID_HDMV_PGS_SUBTITLE 94214

#define AV_CODEC_ID_DVB_TELETEXT 94215

#define AV_CODEC_ID_SRT 94216

#define AV_CODEC_ID_MICRODVD 94217

#define AV_CODEC_ID_EIA_608 94218

#define AV_CODEC_ID_JACOSUB 94219

#define AV_CODEC_ID_SAMI 94220

#define AV_CODEC_ID_REALTEXT 94221

#define AV_CODEC_ID_STL 94222

#define AV_CODEC_ID_SUBVIEWER1 94223

#define AV_CODEC_ID_SUBVIEWER 94224

#define AV_CODEC_ID_SUBRIP 94225

#define AV_CODEC_ID_WEBVTT 94226

#define AV_CODEC_ID_MPL2 94227

#define AV_CODEC_ID_VPLAYER 94228

#define AV_CODEC_ID_PJS 94229

#define AV_CODEC_ID_ASS 94230

#define AV_CODEC_ID_HDMV_TEXT_SUBTITLE 94231

#define AV_CODEC_ID_TTML 94232

#define AV_CODEC_ID_ARIB_CAPTION 94233

#define AV_CODEC_ID_FIRST_UNKNOWN 98304

#define AV_CODEC_ID_TTF 98304

#define AV_CODEC_ID_SCTE_35 98305

#define AV_CODEC_ID_EPG 98306

#define AV_CODEC_ID_BINTEXT 98307

#define AV_CODEC_ID_XBIN 98308

#define AV_CODEC_ID_IDF 98309

#define AV_CODEC_ID_OTF 98310

#define AV_CODEC_ID_SMPTE_KLV 98311

#define AV_CODEC_ID_DVD_NAV 98312

#define AV_CODEC_ID_TIMED_ID3 98313

#define AV_CODEC_ID_BIN_DATA 98314

#define AV_CODEC_ID_PROBE 102400

#define AV_CODEC_ID_MPEG2TS 131072

#define AV_CODEC_ID_MPEG4SYSTEMS 131073

#define AV_CODEC_ID_FFMETADATA 135168

#define AV_CODEC_ID_WRAPPED_AVFRAME 135169

#define AV_FIELD_UNKNOWN 0

#define AV_FIELD_PROGRESSIVE 1

#define AV_FIELD_TT 2

#define AV_FIELD_BB 3

#define AV_FIELD_TB 4

#define AV_FIELD_BT 5

#define AV_AUDIO_SERVICE_TYPE_MAIN 0

#define AV_AUDIO_SERVICE_TYPE_EFFECTS 1

#define AV_AUDIO_SERVICE_TYPE_VISUALLY_IMPAIRED 2

#define AV_AUDIO_SERVICE_TYPE_HEARING_IMPAIRED 3

#define AV_AUDIO_SERVICE_TYPE_DIALOGUE 4

#define AV_AUDIO_SERVICE_TYPE_COMMENTARY 5

#define AV_AUDIO_SERVICE_TYPE_EMERGENCY 6

#define AV_AUDIO_SERVICE_TYPE_VOICE_OVER 7

#define AV_AUDIO_SERVICE_TYPE_KARAOKE 8

#define AVDISCARD_NONE -16

#define AVDISCARD_DEFAULT 0

#define AVDISCARD_NONREF 8

#define AVDISCARD_BIDIR 16

#define AVDISCARD_NONINTRA 24

#define AVDISCARD_NONKEY 32

#define AVDISCARD_ALL 48

#define AV_PKT_DATA_PALETTE 0

#define AV_PKT_DATA_NEW_EXTRADATA 1

#define AV_PKT_DATA_PARAM_CHANGE 2

#define AV_PKT_DATA_H263_MB_INFO 3

#define AV_PKT_DATA_REPLAYGAIN 4

#define AV_PKT_DATA_DISPLAYMATRIX 5

#define AV_PKT_DATA_STEREO3D 6

#define AV_PKT_DATA_AUDIO_SERVICE_TYPE 7

#define AV_PKT_DATA_QUALITY_STATS 8

#define AV_PKT_DATA_FALLBACK_TRACK 9

#define AV_PKT_DATA_CPB_PROPERTIES 10

#define AV_PKT_DATA_SKIP_SAMPLES 11

#define AV_PKT_DATA_JP_DUALMONO 12

#define AV_PKT_DATA_STRINGS_METADATA 13

#define AV_PKT_DATA_SUBTITLE_POSITION 14

#define AV_PKT_DATA_MATROSKA_BLOCKADDITIONAL 15

#define AV_PKT_DATA_WEBVTT_IDENTIFIER 16

#define AV_PKT_DATA_WEBVTT_SETTINGS 17

#define AV_PKT_DATA_METADATA_UPDATE 18

#define AV_PKT_DATA_MPEGTS_STREAM_ID 19

#define AV_PKT_DATA_MASTERING_DISPLAY_METADATA 20

#define AV_PKT_DATA_SPHERICAL 21

#define AV_PKT_DATA_CONTENT_LIGHT_LEVEL 22

#define AV_PKT_DATA_A53_CC 23

#define AV_PKT_DATA_ENCRYPTION_INIT_INFO 24

#define AV_PKT_DATA_ENCRYPTION_INFO 25

#define AV_PKT_DATA_AFD 26

#define AV_PKT_DATA_PRFT 27

#define AV_PKT_DATA_ICC_PROFILE 28

#define AV_PKT_DATA_DOVI_CONF 29

#define AV_PKT_DATA_S12M_TIMECODE 30

#define AV_PKT_DATA_DYNAMIC_HDR10_PLUS 31

#define AV_PICTURE_TYPE_NONE 0

#define AV_PICTURE_TYPE_I 1

#define AV_PICTURE_TYPE_P 2

#define AV_PICTURE_TYPE_B 3

#define AV_PICTURE_TYPE_S 4

#define AV_PICTURE_TYPE_SI 5

#define AV_PICTURE_TYPE_SP 6

#define AV_PICTURE_TYPE_BI 7

#define AVMEDIA_TYPE_UNKNOWN -1

#define AVMEDIA_TYPE_VIDEO 0

#define AVMEDIA_TYPE_AUDIO 1

#define AVMEDIA_TYPE_DATA 2

#define AVMEDIA_TYPE_SUBTITLE 3

#define AVMEDIA_TYPE_ATTACHMENT 4































#define AV_FRAME_DATA_PANSCAN 0

#define AV_FRAME_DATA_A53_CC 1

#define AV_FRAME_DATA_STEREO3D 2

#define AV_FRAME_DATA_MATRIXENCODING 3

#define AV_FRAME_DATA_DOWNMIX_INFO 4

#define AV_FRAME_DATA_REPLAYGAIN 5

#define AV_FRAME_DATA_DISPLAYMATRIX 6

#define AV_FRAME_DATA_AFD 7

#define AV_FRAME_DATA_MOTION_VECTORS 8

#define AV_FRAME_DATA_SKIP_SAMPLES 9

#define AV_FRAME_DATA_AUDIO_SERVICE_TYPE 10

#define AV_FRAME_DATA_MASTERING_DISPLAY_METADATA 11

#define AV_FRAME_DATA_GOP_TIMECODE 12

#define AV_FRAME_DATA_SPHERICAL 13

#define AV_FRAME_DATA_CONTENT_LIGHT_LEVEL 14

#define AV_FRAME_DATA_ICC_PROFILE 15

#define AV_FRAME_DATA_S12M_TIMECODE 16

#define AV_FRAME_DATA_DYNAMIC_HDR_PLUS 17

#define AV_FRAME_DATA_REGIONS_OF_INTEREST 18

#define AV_FRAME_DATA_VIDEO_ENC_PARAMS 19

#define AV_FRAME_DATA_SEI_UNREGISTERED 20

#define AV_FRAME_DATA_FILM_GRAIN_PARAMS 21

#define AV_FRAME_DATA_DETECTION_BBOXES 22

#define AV_FRAME_DATA_DOVI_RPU_BUFFER 23

#define AV_FRAME_DATA_DOVI_METADATA 24

#define AV_HWDEVICE_TYPE_NONE 0

#define AV_HWDEVICE_TYPE_VDPAU 1

#define AV_HWDEVICE_TYPE_CUDA 2

#define AV_HWDEVICE_TYPE_VAAPI 3

#define AV_HWDEVICE_TYPE_DXVA2 4

#define AV_HWDEVICE_TYPE_QSV 5

#define AV_HWDEVICE_TYPE_VIDEOTOOLBOX 6

#define AV_HWDEVICE_TYPE_D3D11VA 7

#define AV_HWDEVICE_TYPE_DRM 8

#define AV_HWDEVICE_TYPE_OPENCL 9

#define AV_HWDEVICE_TYPE_MEDIACODEC 10

#define AV_HWDEVICE_TYPE_VULKAN 11

#define AV_CLASS_CATEGORY_NA 0

#define AV_CLASS_CATEGORY_INPUT 1

#define AV_CLASS_CATEGORY_OUTPUT 2

#define AV_CLASS_CATEGORY_MUXER 3

#define AV_CLASS_CATEGORY_DEMUXER 4

#define AV_CLASS_CATEGORY_ENCODER 5

#define AV_CLASS_CATEGORY_DECODER 6

#define AV_CLASS_CATEGORY_FILTER 7

#define AV_CLASS_CATEGORY_BITSTREAM_FILTER 8

#define AV_CLASS_CATEGORY_SWSCALER 9

#define AV_CLASS_CATEGORY_SWRESAMPLER 10

#define AV_CLASS_CATEGORY_DEVICE_VIDEO_OUTPUT 40

#define AV_CLASS_CATEGORY_DEVICE_VIDEO_INPUT 41

#define AV_CLASS_CATEGORY_DEVICE_AUDIO_OUTPUT 42

#define AV_CLASS_CATEGORY_DEVICE_AUDIO_INPUT 43

#define AV_CLASS_CATEGORY_DEVICE_OUTPUT 44

#define AV_CLASS_CATEGORY_DEVICE_INPUT 45

#define AV_OPT_TYPE_FLAGS 0

#define AV_OPT_TYPE_INT 1

#define AV_OPT_TYPE_INT64 2

#define AV_OPT_TYPE_DOUBLE 3

#define AV_OPT_TYPE_FLOAT 4

#define AV_OPT_TYPE_STRING 5

#define AV_OPT_TYPE_RATIONAL 6

#define AV_OPT_TYPE_BINARY 7

#define AV_OPT_TYPE_DICT 8

#define AV_OPT_TYPE_UINT64 9

#define AV_OPT_TYPE_CONST 10

#define AV_OPT_TYPE_IMAGE_SIZE 11

#define AV_OPT_TYPE_PIXEL_FMT 12

#define AV_OPT_TYPE_SAMPLE_FMT 13

#define AV_OPT_TYPE_VIDEO_RATE 14

#define AV_OPT_TYPE_DURATION 15

#define AV_OPT_TYPE_COLOR 16

#define AV_OPT_TYPE_CHANNEL_LAYOUT 17

#define AV_OPT_TYPE_BOOL 18

#define AVCOL_RANGE_UNSPECIFIED 0

#define AVCOL_RANGE_MPEG 1

#define AVCOL_RANGE_JPEG 2

#define AVCOL_PRI_RESERVED0 0

#define AVCOL_PRI_BT709 1

#define AVCOL_PRI_UNSPECIFIED 2

#define AVCOL_PRI_RESERVED 3

#define AVCOL_PRI_BT470M 4

#define AVCOL_PRI_BT470BG 5

#define AVCOL_PRI_SMPTE170M 6

#define AVCOL_PRI_SMPTE240M 7

#define AVCOL_PRI_FILM 8

#define AVCOL_PRI_BT2020 9

#define AVCOL_PRI_SMPTE428 10

#define AVCOL_PRI_SMPTEST428_1 AVCOL_PRI_SMPTE428

#define AVCOL_PRI_SMPTE431 11

#define AVCOL_PRI_SMPTE432 12

#define AVCOL_PRI_EBU3213 22

#define AVCOL_PRI_JEDEC_P22 AVCOL_PRI_EBU3213

#define AVCOL_TRC_RESERVED0 0

#define AVCOL_TRC_BT709 1

#define AVCOL_TRC_UNSPECIFIED 2

#define AVCOL_TRC_RESERVED 3

#define AVCOL_TRC_GAMMA22 4

#define AVCOL_TRC_GAMMA28 5

#define AVCOL_TRC_SMPTE170M 6

#define AVCOL_TRC_SMPTE240M 7

#define AVCOL_TRC_LINEAR 8

#define AVCOL_TRC_LOG 9

#define AVCOL_TRC_LOG_SQRT 10

#define AVCOL_TRC_IEC61966_2_4 11

#define AVCOL_TRC_BT1361_ECG 12

#define AVCOL_TRC_IEC61966_2_1 13

#define AVCOL_TRC_BT2020_10 14

#define AVCOL_TRC_BT2020_12 15

#define AVCOL_TRC_SMPTE2084 16

#define AVCOL_TRC_SMPTEST2084 AVCOL_TRC_SMPTE2084

#define AVCOL_TRC_SMPTE428 17

#define AVCOL_TRC_SMPTEST428_1 AVCOL_TRC_SMPTE428

#define AVCOL_TRC_ARIB_STD_B67 18

#define AVCOL_SPC_RGB 0

#define AVCOL_SPC_BT709 1

#define AVCOL_SPC_UNSPECIFIED 2

#define AVCOL_SPC_RESERVED 3

#define AVCOL_SPC_FCC 4

#define AVCOL_SPC_BT470BG 5

#define AVCOL_SPC_SMPTE170M 6

#define AVCOL_SPC_SMPTE240M 7

#define AVCOL_SPC_YCGCO 8

#define AVCOL_SPC_YCOCG AVCOL_SPC_YCGCO

#define AVCOL_SPC_BT2020_NCL 9

#define AVCOL_SPC_BT2020_CL 10

#define AVCOL_SPC_SMPTE2085 11

#define AVCOL_SPC_CHROMA_DERIVED_NCL 12

#define AVCOL_SPC_CHROMA_DERIVED_CL 13

#define AVCOL_SPC_ICTCP 14

#define AVCHROMA_LOC_UNSPECIFIED 0

#define AVCHROMA_LOC_LEFT 1

#define AVCHROMA_LOC_CENTER 2

#define AVCHROMA_LOC_TOPLEFT 3

#define AVCHROMA_LOC_TOP 4

#define AVCHROMA_LOC_BOTTOMLEFT 5

#define AVCHROMA_LOC_BOTTOM 6

#define AV_PIX_FMT_NONE -1

#define AV_PIX_FMT_YUV420P 0

#define AV_PIX_FMT_YUYV422 1

#define AV_PIX_FMT_RGB24 2

#define AV_PIX_FMT_BGR24 3

#define AV_PIX_FMT_YUV422P 4

#define AV_PIX_FMT_YUV444P 5

#define AV_PIX_FMT_YUV410P 6

#define AV_PIX_FMT_YUV411P 7

#define AV_PIX_FMT_GRAY8 8

#define AV_PIX_FMT_MONOWHITE 9

#define AV_PIX_FMT_MONOBLACK 10

#define AV_PIX_FMT_PAL8 11

#define AV_PIX_FMT_YUVJ420P 12

#define AV_PIX_FMT_YUVJ422P 13

#define AV_PIX_FMT_YUVJ444P 14

#define AV_PIX_FMT_UYVY422 15

#define AV_PIX_FMT_UYYVYY411 16

#define AV_PIX_FMT_BGR8 17

#define AV_PIX_FMT_BGR4 18

#define AV_PIX_FMT_BGR4_BYTE 19

#define AV_PIX_FMT_RGB8 20

#define AV_PIX_FMT_RGB4 21

#define AV_PIX_FMT_RGB4_BYTE 22

#define AV_PIX_FMT_NV12 23

#define AV_PIX_FMT_NV21 24

#define AV_PIX_FMT_ARGB 25

#define AV_PIX_FMT_RGBA 26

#define AV_PIX_FMT_ABGR 27

#define AV_PIX_FMT_BGRA 28

#define AV_PIX_FMT_GRAY16BE 29

#define AV_PIX_FMT_GRAY16LE 30

#define AV_PIX_FMT_YUV440P 31

#define AV_PIX_FMT_YUVJ440P 32

#define AV_PIX_FMT_YUVA420P 33

#define AV_PIX_FMT_RGB48BE 34

#define AV_PIX_FMT_RGB48LE 35

#define AV_PIX_FMT_RGB565BE 36

#define AV_PIX_FMT_RGB565LE 37

#define AV_PIX_FMT_RGB555BE 38

#define AV_PIX_FMT_RGB555LE 39

#define AV_PIX_FMT_BGR565BE 40

#define AV_PIX_FMT_BGR565LE 41

#define AV_PIX_FMT_BGR555BE 42

#define AV_PIX_FMT_BGR555LE 43

/**
 *  *  Hardware acceleration through VA-API, data[3] contains a  *  VASurfaceID.
 */
#define AV_PIX_FMT_VAAPI 44

#define AV_PIX_FMT_YUV420P16LE 45

#define AV_PIX_FMT_YUV420P16BE 46

#define AV_PIX_FMT_YUV422P16LE 47

#define AV_PIX_FMT_YUV422P16BE 48

#define AV_PIX_FMT_YUV444P16LE 49

#define AV_PIX_FMT_YUV444P16BE 50

#define AV_PIX_FMT_DXVA2_VLD 51

#define AV_PIX_FMT_RGB444LE 52

#define AV_PIX_FMT_RGB444BE 53

#define AV_PIX_FMT_BGR444LE 54

#define AV_PIX_FMT_BGR444BE 55

#define AV_PIX_FMT_YA8 56

#define AV_PIX_FMT_Y400A AV_PIX_FMT_YA8

#define AV_PIX_FMT_GRAY8A AV_PIX_FMT_YA8

#define AV_PIX_FMT_BGR48BE 57

#define AV_PIX_FMT_BGR48LE 58

/**
 *  * The following 12 formats have the disadvantage of needing 1 format for each bit depth.  * Notice that each 9/10 bits sample is stored in 16 bits with extra padding.  * If you want to support multiple bit depths, then using AV_PIX_FMT_YUV420P16* with the bpp stored separately is better.
 */
#define AV_PIX_FMT_YUV420P9BE 59

#define AV_PIX_FMT_YUV420P9LE 60

#define AV_PIX_FMT_YUV420P10BE 61

#define AV_PIX_FMT_YUV420P10LE 62

#define AV_PIX_FMT_YUV422P10BE 63

#define AV_PIX_FMT_YUV422P10LE 64

#define AV_PIX_FMT_YUV444P9BE 65

#define AV_PIX_FMT_YUV444P9LE 66

#define AV_PIX_FMT_YUV444P10BE 67

#define AV_PIX_FMT_YUV444P10LE 68

#define AV_PIX_FMT_YUV422P9BE 69

#define AV_PIX_FMT_YUV422P9LE 70

#define AV_PIX_FMT_GBRP 71

#define AV_PIX_FMT_GBR24P AV_PIX_FMT_GBRP

#define AV_PIX_FMT_GBRP9BE 72

#define AV_PIX_FMT_GBRP9LE 73

#define AV_PIX_FMT_GBRP10BE 74

#define AV_PIX_FMT_GBRP10LE 75

#define AV_PIX_FMT_GBRP16BE 76

#define AV_PIX_FMT_GBRP16LE 77

#define AV_PIX_FMT_YUVA422P 78

#define AV_PIX_FMT_YUVA444P 79

#define AV_PIX_FMT_YUVA420P9BE 80

#define AV_PIX_FMT_YUVA420P9LE 81

#define AV_PIX_FMT_YUVA422P9BE 82

#define AV_PIX_FMT_YUVA422P9LE 83

#define AV_PIX_FMT_YUVA444P9BE 84

#define AV_PIX_FMT_YUVA444P9LE 85

#define AV_PIX_FMT_YUVA420P10BE 86

#define AV_PIX_FMT_YUVA420P10LE 87

#define AV_PIX_FMT_YUVA422P10BE 88

#define AV_PIX_FMT_YUVA422P10LE 89

#define AV_PIX_FMT_YUVA444P10BE 90

#define AV_PIX_FMT_YUVA444P10LE 91

#define AV_PIX_FMT_YUVA420P16BE 92

#define AV_PIX_FMT_YUVA420P16LE 93

#define AV_PIX_FMT_YUVA422P16BE 94

#define AV_PIX_FMT_YUVA422P16LE 95

#define AV_PIX_FMT_YUVA444P16BE 96

#define AV_PIX_FMT_YUVA444P16LE 97

#define AV_PIX_FMT_VDPAU 98

#define AV_PIX_FMT_XYZ12LE 99

#define AV_PIX_FMT_XYZ12BE 100

#define AV_PIX_FMT_NV16 101

#define AV_PIX_FMT_NV20LE 102

#define AV_PIX_FMT_NV20BE 103

#define AV_PIX_FMT_RGBA64BE 104

#define AV_PIX_FMT_RGBA64LE 105

#define AV_PIX_FMT_BGRA64BE 106

#define AV_PIX_FMT_BGRA64LE 107

#define AV_PIX_FMT_YVYU422 108

#define AV_PIX_FMT_YA16BE 109

#define AV_PIX_FMT_YA16LE 110

#define AV_PIX_FMT_GBRAP 111

#define AV_PIX_FMT_GBRAP16BE 112

#define AV_PIX_FMT_GBRAP16LE 113

/**
 *  *  HW acceleration through QSV, data[3] contains a pointer to the  *  mfxFrameSurface1 structure.
 */
#define AV_PIX_FMT_QSV 114

/**
 *  * HW acceleration though MMAL, data[3] contains a pointer to the  * MMAL_BUFFER_HEADER_T structure.
 */
#define AV_PIX_FMT_MMAL 115

#define AV_PIX_FMT_D3D11VA_VLD 116

/**
 *  * HW acceleration through CUDA. data[i] contain CUdeviceptr pointers  * exactly as for system memory frames.
 */
#define AV_PIX_FMT_CUDA 117

#define AV_PIX_FMT_0RGB 118

#define AV_PIX_FMT_RGB0 119

#define AV_PIX_FMT_0BGR 120

#define AV_PIX_FMT_BGR0 121

#define AV_PIX_FMT_YUV420P12BE 122

#define AV_PIX_FMT_YUV420P12LE 123

#define AV_PIX_FMT_YUV420P14BE 124

#define AV_PIX_FMT_YUV420P14LE 125

#define AV_PIX_FMT_YUV422P12BE 126

#define AV_PIX_FMT_YUV422P12LE 127

#define AV_PIX_FMT_YUV422P14BE 128

#define AV_PIX_FMT_YUV422P14LE 129

#define AV_PIX_FMT_YUV444P12BE 130

#define AV_PIX_FMT_YUV444P12LE 131

#define AV_PIX_FMT_YUV444P14BE 132

#define AV_PIX_FMT_YUV444P14LE 133

#define AV_PIX_FMT_GBRP12BE 134

#define AV_PIX_FMT_GBRP12LE 135

#define AV_PIX_FMT_GBRP14BE 136

#define AV_PIX_FMT_GBRP14LE 137

#define AV_PIX_FMT_YUVJ411P 138

#define AV_PIX_FMT_BAYER_BGGR8 139

#define AV_PIX_FMT_BAYER_RGGB8 140

#define AV_PIX_FMT_BAYER_GBRG8 141

#define AV_PIX_FMT_BAYER_GRBG8 142

#define AV_PIX_FMT_BAYER_BGGR16LE 143

#define AV_PIX_FMT_BAYER_BGGR16BE 144

#define AV_PIX_FMT_BAYER_RGGB16LE 145

#define AV_PIX_FMT_BAYER_RGGB16BE 146

#define AV_PIX_FMT_BAYER_GBRG16LE 147

#define AV_PIX_FMT_BAYER_GBRG16BE 148

#define AV_PIX_FMT_BAYER_GRBG16LE 149

#define AV_PIX_FMT_BAYER_GRBG16BE 150

#define AV_PIX_FMT_XVMC 151

#define AV_PIX_FMT_YUV440P10LE 152

#define AV_PIX_FMT_YUV440P10BE 153

#define AV_PIX_FMT_YUV440P12LE 154

#define AV_PIX_FMT_YUV440P12BE 155

#define AV_PIX_FMT_AYUV64LE 156

#define AV_PIX_FMT_AYUV64BE 157

#define AV_PIX_FMT_VIDEOTOOLBOX 158

#define AV_PIX_FMT_P010LE 159

#define AV_PIX_FMT_P010BE 160

#define AV_PIX_FMT_GBRAP12BE 161

#define AV_PIX_FMT_GBRAP12LE 162

#define AV_PIX_FMT_GBRAP10BE 163

#define AV_PIX_FMT_GBRAP10LE 164

#define AV_PIX_FMT_MEDIACODEC 165

#define AV_PIX_FMT_GRAY12BE 166

#define AV_PIX_FMT_GRAY12LE 167

#define AV_PIX_FMT_GRAY10BE 168

#define AV_PIX_FMT_GRAY10LE 169

#define AV_PIX_FMT_P016LE 170

#define AV_PIX_FMT_P016BE 171

/**
 *  * Hardware surfaces for Direct3D11.  *  * This is preferred over the legacy AV_PIX_FMT_D3D11VA_VLD. The new D3D11  * hwaccel API and filtering support AV_PIX_FMT_D3D11 only.  *  * data[0] contains a ID3D11Texture2D pointer, and data[1] contains the  * texture array index of the frame as intptr_t if the ID3D11Texture2D is  * an array texture (or always 0 if it's a normal texture).
 */
#define AV_PIX_FMT_D3D11 172

#define AV_PIX_FMT_GRAY9BE 173

#define AV_PIX_FMT_GRAY9LE 174

#define AV_PIX_FMT_GBRPF32BE 175

#define AV_PIX_FMT_GBRPF32LE 176

#define AV_PIX_FMT_GBRAPF32BE 177

#define AV_PIX_FMT_GBRAPF32LE 178

/**
 *  * DRM-managed buffers exposed through PRIME buffer sharing.  *  * data[0] points to an AVDRMFrameDescriptor.
 */
#define AV_PIX_FMT_DRM_PRIME 179

/**
 *  * Hardware surfaces for OpenCL.  *  * data[i] contain 2D image objects (typed in C as cl_mem, used  * in OpenCL as image2d_t) for each plane of the surface.
 */
#define AV_PIX_FMT_OPENCL 180

#define AV_PIX_FMT_GRAY14BE 181

#define AV_PIX_FMT_GRAY14LE 182

#define AV_PIX_FMT_GRAYF32BE 183

#define AV_PIX_FMT_GRAYF32LE 184

#define AV_PIX_FMT_YUVA422P12BE 185

#define AV_PIX_FMT_YUVA422P12LE 186

#define AV_PIX_FMT_YUVA444P12BE 187

#define AV_PIX_FMT_YUVA444P12LE 188

#define AV_PIX_FMT_NV24 189

#define AV_PIX_FMT_NV42 190

/**
 *  * Vulkan hardware images.  *  * data[0] points to an AVVkFrame
 */
#define AV_PIX_FMT_VULKAN 191

#define AV_PIX_FMT_Y210BE 192

#define AV_PIX_FMT_Y210LE 193

#define AV_PIX_FMT_X2RGB10LE 194

#define AV_PIX_FMT_X2RGB10BE 195

#define AV_PIX_FMT_X2BGR10LE 196

#define AV_PIX_FMT_X2BGR10BE 197

#define AV_PIX_FMT_P210BE 198

#define AV_PIX_FMT_P210LE 199

#define AV_PIX_FMT_P410BE 200

#define AV_PIX_FMT_P410LE 201

#define AV_PIX_FMT_P216BE 202

#define AV_PIX_FMT_P216LE 203

#define AV_PIX_FMT_P416BE 204

#define AV_PIX_FMT_P416LE 205

#define AV_SAMPLE_FMT_NONE -1

#define AV_SAMPLE_FMT_U8 0

#define AV_SAMPLE_FMT_S16 1

#define AV_SAMPLE_FMT_S32 2

#define AV_SAMPLE_FMT_FLT 3

#define AV_SAMPLE_FMT_DBL 4

#define AV_SAMPLE_FMT_U8P 5

#define AV_SAMPLE_FMT_S16P 6

#define AV_SAMPLE_FMT_S32P 7

#define AV_SAMPLE_FMT_FLTP 8

#define AV_SAMPLE_FMT_DBLP 9

#define AV_SAMPLE_FMT_S64 10

#define AV_SAMPLE_FMT_S64P 11

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
                                           struct wire_uint_8_list *remote_device_id,
                                           int64_t texture_id,
                                           int64_t video_texture_ptr,
                                           int64_t update_frame_callback_ptr);

void wire_utility_generate_device_password(int64_t port_);

struct wire_uint_8_list *new_uint_8_list(int32_t len);

void free_WireSyncReturnStruct(struct WireSyncReturnStruct val);

void store_dart_post_cobject(DartPostCObjectFnType ptr);

extern int32_t avcodec_receive_packet(struct AVCodecContext *avctx, struct AVPacket *avpkt);

extern int32_t avcodec_send_frame(struct AVCodecContext *avctx, struct AVFrame *frame);

extern int32_t avcodec_send_packet(struct AVCodecContext *avctx, const struct AVPacket *avpkt);

extern int32_t avcodec_receive_frame(struct AVCodecContext *avctx, struct AVFrame *frame);

extern int32_t av_parser_parse2(struct AVCodecParserContext *s,
                                struct AVCodecContext *avctx,
                                uint8_t **poutbuf,
                                int32_t *poutbuf_size,
                                const uint8_t *buf,
                                int32_t buf_size,
                                int64_t pts,
                                int64_t dts,
                                int64_t pos);

extern void av_parser_close(struct AVCodecParserContext *s);

extern struct AVCodecContext *avcodec_alloc_context3(const struct AVCodec *codec);

extern int32_t avcodec_open2(struct AVCodecContext *avctx,
                             const struct AVCodec *codec,
                             const void *options);

extern void avcodec_free_context(struct AVCodecContext **avctx);

extern struct AVCodecParserContext *av_parser_init(uint32_t codec_id);

extern const struct AVCodec *avcodec_find_encoder_by_name(const char *name);

extern const struct AVCodec *avcodec_find_decoder_by_name(const char *name);

extern const struct AVCodecHWConfig *avcodec_get_hw_config(const struct AVCodec *codec,
                                                           int32_t index);

extern void av_packet_free(struct AVPacket **pkt);

extern struct AVPacket *av_packet_alloc(void);

extern int32_t av_new_packet(struct AVPacket *pkt, int32_t size);

extern void av_packet_unref(struct AVPacket *pkt);

extern struct AVBufferRef *av_buffer_ref(const struct AVBufferRef *buf);

extern void av_frame_free(struct AVFrame **frame);

extern struct AVFrame *av_frame_alloc(void);

extern int32_t av_frame_get_buffer(struct AVFrame *frame, int32_t align);

extern int32_t av_frame_make_writable(struct AVFrame *frame);

extern int32_t av_hwframe_transfer_data(struct AVFrame *dst,
                                        const struct AVFrame *src,
                                        int32_t flags);

extern AVHWDeviceType av_hwdevice_iterate_types(AVHWDeviceType prev);

extern const char *av_hwdevice_get_type_name(AVHWDeviceType type_);

extern int32_t av_hwdevice_ctx_create(struct AVBufferRef **device_ctx,
                                      AVHWDeviceType type_,
                                      const char *device,
                                      void *opts,
                                      int32_t flags);

extern int32_t av_image_get_buffer_size(AVPixelFormat pix_fmt,
                                        int32_t width,
                                        int32_t height,
                                        int32_t align);

extern void av_log_set_level(int32_t level);

extern int32_t av_log_get_level(void);

extern void av_log_set_flags(int32_t arg);

extern int32_t av_log_get_flags(void);

extern int32_t av_opt_set(void *obj, const char *name, const char *val, int32_t search_flags);

extern struct CMTime CMTimeMake(int64_t value, int32_t time_scale);

extern bool CMSampleBufferIsValid(CMSampleBufferRef sample_buffer);

extern int CMSampleBufferGetSampleTimingInfo(CMSampleBufferRef sample_buffer,
                                             CFIndex sampleIndex,
                                             struct CMSampleTimingInfo *timing_info_out);

extern CVImageBufferRef CMSampleBufferGetImageBuffer(CMSampleBufferRef sample_buffer);

extern uint32_t CVPixelBufferGetPixelFormatType(CVPixelBufferRef pixel_buffer);

extern int32_t CVPixelBufferLockBaseAddress(CVPixelBufferRef pixel_buffer, uint32_t lock_flags);

extern int32_t CVPixelBufferUnlockBaseAddress(CVPixelBufferRef pixel_buffer, uint32_t unlock_flags);

extern size_t CVPixelBufferGetWidth(CVPixelBufferRef pixel_buffer);

extern size_t CVPixelBufferGetHeight(CVPixelBufferRef pixel_buffer);

extern size_t CVPixelBufferGetBytesPerRowOfPlane(CVPixelBufferRef pixel_buffer, size_t planeIndex);

extern void *CVPixelBufferGetBaseAddressOfPlane(CVPixelBufferRef pixel_buffer, size_t planeIndex);

extern uint32_t CVPixelBufferGetHeightOfPlane(CVPixelBufferRef pixel_buffer, uint32_t planeIndex);

extern CVPixelBufferRef CVPixelBufferRetain(CVPixelBufferRef texture);

extern CFTypeRef CVBufferGetAttachment(void *buffer, CFStringRef key, void *attachmentMode);

extern intptr_t NV12ToARGBMatrix(const uint8_t *src_y,
                                 intptr_t src_stride_y,
                                 const uint8_t *src_uv,
                                 intptr_t src_stride_uv,
                                 uint8_t *dst_argb,
                                 intptr_t dst_stride_argb,
                                 const void *yuvconstants,
                                 intptr_t width,
                                 intptr_t height);

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
    dummy_var ^= ((int64_t) (void*) wire_utility_generate_device_password);
    dummy_var ^= ((int64_t) (void*) new_uint_8_list);
    dummy_var ^= ((int64_t) (void*) free_WireSyncReturnStruct);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    return dummy_var;
}