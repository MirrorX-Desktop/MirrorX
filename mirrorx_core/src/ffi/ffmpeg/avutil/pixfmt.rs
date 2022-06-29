pub type AVColorRange = u32;
pub const AVCOL_RANGE_UNSPECIFIED: AVColorRange = 0;
pub const AVCOL_RANGE_MPEG: AVColorRange = 1;
pub const AVCOL_RANGE_JPEG: AVColorRange = 2;

pub type AVColorPrimaries = u32;
pub const AVCOL_PRI_RESERVED0: AVColorPrimaries = 0;
pub const AVCOL_PRI_BT709: AVColorPrimaries = 1;
pub const AVCOL_PRI_UNSPECIFIED: AVColorPrimaries = 2;
pub const AVCOL_PRI_RESERVED: AVColorPrimaries = 3;
pub const AVCOL_PRI_BT470M: AVColorPrimaries = 4;
pub const AVCOL_PRI_BT470BG: AVColorPrimaries = 5;
pub const AVCOL_PRI_SMPTE170M: AVColorPrimaries = 6;
pub const AVCOL_PRI_SMPTE240M: AVColorPrimaries = 7;
pub const AVCOL_PRI_FILM: AVColorPrimaries = 8;
pub const AVCOL_PRI_BT2020: AVColorPrimaries = 9;
pub const AVCOL_PRI_SMPTE428: AVColorPrimaries = 10;
pub const AVCOL_PRI_SMPTEST428_1: AVColorPrimaries = AVCOL_PRI_SMPTE428;
pub const AVCOL_PRI_SMPTE431: AVColorPrimaries = 11;
pub const AVCOL_PRI_SMPTE432: AVColorPrimaries = 12;
pub const AVCOL_PRI_EBU3213: AVColorPrimaries = 22;
pub const AVCOL_PRI_JEDEC_P22: AVColorPrimaries = AVCOL_PRI_EBU3213;

pub type AVColorTransferCharacteristic = u32;
pub const AVCOL_TRC_RESERVED0: AVColorTransferCharacteristic = 0;
pub const AVCOL_TRC_BT709: AVColorTransferCharacteristic = 1;
pub const AVCOL_TRC_UNSPECIFIED: AVColorTransferCharacteristic = 2;
pub const AVCOL_TRC_RESERVED: AVColorTransferCharacteristic = 3;
pub const AVCOL_TRC_GAMMA22: AVColorTransferCharacteristic = 4;
pub const AVCOL_TRC_GAMMA28: AVColorTransferCharacteristic = 5;
pub const AVCOL_TRC_SMPTE170M: AVColorTransferCharacteristic = 6;
pub const AVCOL_TRC_SMPTE240M: AVColorTransferCharacteristic = 7;
pub const AVCOL_TRC_LINEAR: AVColorTransferCharacteristic = 8;
pub const AVCOL_TRC_LOG: AVColorTransferCharacteristic = 9;
pub const AVCOL_TRC_LOG_SQRT: AVColorTransferCharacteristic = 10;
pub const AVCOL_TRC_IEC61966_2_4: AVColorTransferCharacteristic = 11;
pub const AVCOL_TRC_BT1361_ECG: AVColorTransferCharacteristic = 12;
pub const AVCOL_TRC_IEC61966_2_1: AVColorTransferCharacteristic = 13;
pub const AVCOL_TRC_BT2020_10: AVColorTransferCharacteristic = 14;
pub const AVCOL_TRC_BT2020_12: AVColorTransferCharacteristic = 15;
pub const AVCOL_TRC_SMPTE2084: AVColorTransferCharacteristic = 16;
pub const AVCOL_TRC_SMPTEST2084: AVColorTransferCharacteristic = AVCOL_TRC_SMPTE2084;
pub const AVCOL_TRC_SMPTE428: AVColorTransferCharacteristic = 17;
pub const AVCOL_TRC_SMPTEST428_1: AVColorTransferCharacteristic = AVCOL_TRC_SMPTE428;
pub const AVCOL_TRC_ARIB_STD_B67: AVColorTransferCharacteristic = 18;

pub type AVColorSpace = u32;
pub const AVCOL_SPC_RGB: AVColorSpace = 0;
pub const AVCOL_SPC_BT709: AVColorSpace = 1;
pub const AVCOL_SPC_UNSPECIFIED: AVColorSpace = 2;
pub const AVCOL_SPC_RESERVED: AVColorSpace = 3;
pub const AVCOL_SPC_FCC: AVColorSpace = 4;
pub const AVCOL_SPC_BT470BG: AVColorSpace = 5;
pub const AVCOL_SPC_SMPTE170M: AVColorSpace = 6;
pub const AVCOL_SPC_SMPTE240M: AVColorSpace = 7;
pub const AVCOL_SPC_YCGCO: AVColorSpace = 8;
pub const AVCOL_SPC_YCOCG: AVColorSpace = AVCOL_SPC_YCGCO;
pub const AVCOL_SPC_BT2020_NCL: AVColorSpace = 9;
pub const AVCOL_SPC_BT2020_CL: AVColorSpace = 10;
pub const AVCOL_SPC_SMPTE2085: AVColorSpace = 11;
pub const AVCOL_SPC_CHROMA_DERIVED_NCL: AVColorSpace = 12;
pub const AVCOL_SPC_CHROMA_DERIVED_CL: AVColorSpace = 13;
pub const AVCOL_SPC_ICTCP: AVColorSpace = 14;

pub type AVChromaLocation = u32;
pub const AVCHROMA_LOC_UNSPECIFIED: AVChromaLocation = 0;
pub const AVCHROMA_LOC_LEFT: AVChromaLocation = 1;
pub const AVCHROMA_LOC_CENTER: AVChromaLocation = 2;
pub const AVCHROMA_LOC_TOPLEFT: AVChromaLocation = 3;
pub const AVCHROMA_LOC_TOP: AVChromaLocation = 4;
pub const AVCHROMA_LOC_BOTTOMLEFT: AVChromaLocation = 5;
pub const AVCHROMA_LOC_BOTTOM: AVChromaLocation = 6;

pub type AVPixelFormat = i32;
pub const AV_PIX_FMT_NONE: AVPixelFormat = -1;
pub const AV_PIX_FMT_YUV420P: AVPixelFormat = 0; //  planar YUV 4:2:0, 12bpp, (1 Cr & Cb sample per 2x2 Y samples)
pub const AV_PIX_FMT_YUYV422: AVPixelFormat = 1; //  packed YUV 4:2:2, 16bpp, Y0 Cb Y1 Cr
pub const AV_PIX_FMT_RGB24: AVPixelFormat = 2; //  packed RGB 8:8:8, 24bpp, RGBRGB...
pub const AV_PIX_FMT_BGR24: AVPixelFormat = 3; //  packed RGB 8:8:8, 24bpp, BGRBGR...
pub const AV_PIX_FMT_YUV422P: AVPixelFormat = 4; //  planar YUV 4:2:2, 16bpp, (1 Cr & Cb sample per 2x1 Y samples)
pub const AV_PIX_FMT_YUV444P: AVPixelFormat = 5; //  planar YUV 4:4:4, 24bpp, (1 Cr & Cb sample per 1x1 Y samples)
pub const AV_PIX_FMT_YUV410P: AVPixelFormat = 6; //  planar YUV 4:1:0,  9bpp, (1 Cr & Cb sample per 4x4 Y samples)
pub const AV_PIX_FMT_YUV411P: AVPixelFormat = 7; //  planar YUV 4:1:1, 12bpp, (1 Cr & Cb sample per 4x1 Y samples)
pub const AV_PIX_FMT_GRAY8: AVPixelFormat = 8; //         Y        ,  8bpp
pub const AV_PIX_FMT_MONOWHITE: AVPixelFormat = 9; //         Y        ,  1bpp, 0 is white, 1 is black, in each byte pixels are ordered from the msb to the lsb
pub const AV_PIX_FMT_MONOBLACK: AVPixelFormat = 10; //         Y        ,  1bpp, 0 is black, 1 is white, in each byte pixels are ordered from the msb to the lsb
pub const AV_PIX_FMT_PAL8: AVPixelFormat = 11; //  8 bits with AV_PIX_FMT_RGB32 palette
pub const AV_PIX_FMT_YUVJ420P: AVPixelFormat = 12; //  planar YUV 4:2:0, 12bpp, full scale (JPEG), deprecated in favor of AV_PIX_FMT_YUV420P and setting color_range
pub const AV_PIX_FMT_YUVJ422P: AVPixelFormat = 13; //  planar YUV 4:2:2, 16bpp, full scale (JPEG), deprecated in favor of AV_PIX_FMT_YUV422P and setting color_range
pub const AV_PIX_FMT_YUVJ444P: AVPixelFormat = 14; //  planar YUV 4:4:4, 24bpp, full scale (JPEG), deprecated in favor of AV_PIX_FMT_YUV444P and setting color_range
pub const AV_PIX_FMT_UYVY422: AVPixelFormat = 15; //  packed YUV 4:2:2, 16bpp, Cb Y0 Cr Y1
pub const AV_PIX_FMT_UYYVYY411: AVPixelFormat = 16; //  packed YUV 4:1:1, 12bpp, Cb Y0 Y1 Cr Y2 Y3
pub const AV_PIX_FMT_BGR8: AVPixelFormat = 17; //  packed RGB 3:3:2,  8bpp, (msb)2B 3G 3R(lsb)
pub const AV_PIX_FMT_BGR4: AVPixelFormat = 18; //  packed RGB 1:2:1 bitstream,  4bpp, (msb)1B 2G 1R(lsb), a byte contains two pixels, the first pixel in the byte is the one composed by the 4 msb bits
pub const AV_PIX_FMT_BGR4_BYTE: AVPixelFormat = 19; //  packed RGB 1:2:1,  8bpp, (msb)1B 2G 1R(lsb)
pub const AV_PIX_FMT_RGB8: AVPixelFormat = 20; //  packed RGB 3:3:2,  8bpp, (msb)2R 3G 3B(lsb)
pub const AV_PIX_FMT_RGB4: AVPixelFormat = 21; //  packed RGB 1:2:1 bitstream,  4bpp, (msb)1R 2G 1B(lsb), a byte contains two pixels, the first pixel in the byte is the one composed by the 4 msb bits
pub const AV_PIX_FMT_RGB4_BYTE: AVPixelFormat = 22; //  packed RGB 1:2:1,  8bpp, (msb)1R 2G 1B(lsb)
pub const AV_PIX_FMT_NV12: AVPixelFormat = 23; //  planar YUV 4:2:0, 12bpp, 1 plane for Y and 1 plane for the UV components, which are interleaved (first byte U and the following byte V)
pub const AV_PIX_FMT_NV21: AVPixelFormat = 24; //  as above, but U and V bytes are swapped

pub const AV_PIX_FMT_ARGB: AVPixelFormat = 25; //  packed ARGB 8:8:8:8, 32bpp, ARGBARGB...
pub const AV_PIX_FMT_RGBA: AVPixelFormat = 26; //  packed RGBA 8:8:8:8, 32bpp, RGBARGBA...
pub const AV_PIX_FMT_ABGR: AVPixelFormat = 27; //  packed ABGR 8:8:8:8, 32bpp, ABGRABGR...
pub const AV_PIX_FMT_BGRA: AVPixelFormat = 28; //  packed BGRA 8:8:8:8, 32bpp, BGRABGRA...

pub const AV_PIX_FMT_GRAY16BE: AVPixelFormat = 29; //         Y        , 16bpp, big-endian
pub const AV_PIX_FMT_GRAY16LE: AVPixelFormat = 30; //         Y        , 16bpp, little-endian
pub const AV_PIX_FMT_YUV440P: AVPixelFormat = 31; //  planar YUV 4:4:0 (1 Cr & Cb sample per 1x2 Y samples)
pub const AV_PIX_FMT_YUVJ440P: AVPixelFormat = 32; //  planar YUV 4:4:0 full scale (JPEG), deprecated in favor of AV_PIX_FMT_YUV440P and setting color_range
pub const AV_PIX_FMT_YUVA420P: AVPixelFormat = 33; //  planar YUV 4:2:0, 20bpp, (1 Cr & Cb sample per 2x2 Y & A samples)
pub const AV_PIX_FMT_RGB48BE: AVPixelFormat = 34; //  packed RGB 16:16:16, 48bpp, 16R, 16G, 16B, the 2-byte value for each R/G/B component is stored as big-endian
pub const AV_PIX_FMT_RGB48LE: AVPixelFormat = 35; //  packed RGB 16:16:16, 48bpp, 16R, 16G, 16B, the 2-byte value for each R/G/B component is stored as little-endian

pub const AV_PIX_FMT_RGB565BE: AVPixelFormat = 36; //  packed RGB 5:6:5, 16bpp, (msb)   5R 6G 5B(lsb), big-endian
pub const AV_PIX_FMT_RGB565LE: AVPixelFormat = 37; //  packed RGB 5:6:5, 16bpp, (msb)   5R 6G 5B(lsb), little-endian
pub const AV_PIX_FMT_RGB555BE: AVPixelFormat = 38; //  packed RGB 5:5:5, 16bpp, (msb)1X 5R 5G 5B(lsb), big-endian   , X=unused/undefined
pub const AV_PIX_FMT_RGB555LE: AVPixelFormat = 39; //  packed RGB 5:5:5, 16bpp, (msb)1X 5R 5G 5B(lsb), little-endian, X=unused/undefined

pub const AV_PIX_FMT_BGR565BE: AVPixelFormat = 40; //  packed BGR 5:6:5, 16bpp, (msb)   5B 6G 5R(lsb), big-endian
pub const AV_PIX_FMT_BGR565LE: AVPixelFormat = 41; //  packed BGR 5:6:5, 16bpp, (msb)   5B 6G 5R(lsb), little-endian
pub const AV_PIX_FMT_BGR555BE: AVPixelFormat = 42; //  packed BGR 5:5:5, 16bpp, (msb)1X 5B 5G 5R(lsb), big-endian   , X=unused/undefined
pub const AV_PIX_FMT_BGR555LE: AVPixelFormat = 43; //  packed BGR 5:5:5, 16bpp, (msb)1X 5B 5G 5R(lsb), little-endian, X=unused/undefined

/**
 *  Hardware acceleration through VA-API, data[3] contains a
 *  VASurfaceID.
 */
pub const AV_PIX_FMT_VAAPI: AVPixelFormat = 44;

pub const AV_PIX_FMT_YUV420P16LE: AVPixelFormat = 45; //  planar YUV 4:2:0, 24bpp, (1 Cr & Cb sample per 2x2 Y samples), little-endian
pub const AV_PIX_FMT_YUV420P16BE: AVPixelFormat = 46; //  planar YUV 4:2:0, 24bpp, (1 Cr & Cb sample per 2x2 Y samples), big-endian
pub const AV_PIX_FMT_YUV422P16LE: AVPixelFormat = 47; //  planar YUV 4:2:2, 32bpp, (1 Cr & Cb sample per 2x1 Y samples), little-endian
pub const AV_PIX_FMT_YUV422P16BE: AVPixelFormat = 48; //  planar YUV 4:2:2, 32bpp, (1 Cr & Cb sample per 2x1 Y samples), big-endian
pub const AV_PIX_FMT_YUV444P16LE: AVPixelFormat = 49; //  planar YUV 4:4:4, 48bpp, (1 Cr & Cb sample per 1x1 Y samples), little-endian
pub const AV_PIX_FMT_YUV444P16BE: AVPixelFormat = 50; //  planar YUV 4:4:4, 48bpp, (1 Cr & Cb sample per 1x1 Y samples), big-endian
pub const AV_PIX_FMT_DXVA2_VLD: AVPixelFormat = 51; //  HW decoding through DXVA2, Picture.data[3] contains a LPDIRECT3DSURFACE9 pointer

pub const AV_PIX_FMT_RGB444LE: AVPixelFormat = 52; //  packed RGB 4:4:4, 16bpp, (msb)4X 4R 4G 4B(lsb), little-endian, X=unused/undefined
pub const AV_PIX_FMT_RGB444BE: AVPixelFormat = 53; //  packed RGB 4:4:4, 16bpp, (msb)4X 4R 4G 4B(lsb), big-endian,    X=unused/undefined
pub const AV_PIX_FMT_BGR444LE: AVPixelFormat = 54; //  packed BGR 4:4:4, 16bpp, (msb)4X 4B 4G 4R(lsb), little-endian, X=unused/undefined
pub const AV_PIX_FMT_BGR444BE: AVPixelFormat = 55; //  packed BGR 4:4:4, 16bpp, (msb)4X 4B 4G 4R(lsb), big-endian,    X=unused/undefined
pub const AV_PIX_FMT_YA8: AVPixelFormat = 56; //  8 bits gray, 8 bits alpha

pub const AV_PIX_FMT_Y400A: AVPixelFormat = AV_PIX_FMT_YA8; //  alias for AV_PIX_FMT_YA8
pub const AV_PIX_FMT_GRAY8A: AVPixelFormat = AV_PIX_FMT_YA8; //  alias for AV_PIX_FMT_YA8

pub const AV_PIX_FMT_BGR48BE: AVPixelFormat = 57; //  packed RGB 16:16:16, 48bpp, 16B, 16G, 16R, the 2-byte value for each R/G/B component is stored as big-endian
pub const AV_PIX_FMT_BGR48LE: AVPixelFormat = 58; //  packed RGB 16:16:16, 48bpp, 16B, 16G, 16R, the 2-byte value for each R/G/B component is stored as little-endian

/**
 * The following 12 formats have the disadvantage of needing 1 format for each bit depth.
 * Notice that each 9/10 bits sample is stored in 16 bits with extra padding.
 * If you want to support multiple bit depths, then using AV_PIX_FMT_YUV420P16* with the bpp stored separately is better.
 */
pub const AV_PIX_FMT_YUV420P9BE: AVPixelFormat = 59; //  planar YUV 4:2:0, 13.5bpp, (1 Cr & Cb sample per 2x2 Y samples), big-endian
pub const AV_PIX_FMT_YUV420P9LE: AVPixelFormat = 60; //  planar YUV 4:2:0, 13.5bpp, (1 Cr & Cb sample per 2x2 Y samples), little-endian
pub const AV_PIX_FMT_YUV420P10BE: AVPixelFormat = 61; //  planar YUV 4:2:0, 15bpp, (1 Cr & Cb sample per 2x2 Y samples), big-endian
pub const AV_PIX_FMT_YUV420P10LE: AVPixelFormat = 62; //  planar YUV 4:2:0, 15bpp, (1 Cr & Cb sample per 2x2 Y samples), little-endian
pub const AV_PIX_FMT_YUV422P10BE: AVPixelFormat = 63; //  planar YUV 4:2:2, 20bpp, (1 Cr & Cb sample per 2x1 Y samples), big-endian
pub const AV_PIX_FMT_YUV422P10LE: AVPixelFormat = 64; //  planar YUV 4:2:2, 20bpp, (1 Cr & Cb sample per 2x1 Y samples), little-endian
pub const AV_PIX_FMT_YUV444P9BE: AVPixelFormat = 65; //  planar YUV 4:4:4, 27bpp, (1 Cr & Cb sample per 1x1 Y samples), big-endian
pub const AV_PIX_FMT_YUV444P9LE: AVPixelFormat = 66; //  planar YUV 4:4:4, 27bpp, (1 Cr & Cb sample per 1x1 Y samples), little-endian
pub const AV_PIX_FMT_YUV444P10BE: AVPixelFormat = 67; //  planar YUV 4:4:4, 30bpp, (1 Cr & Cb sample per 1x1 Y samples), big-endian
pub const AV_PIX_FMT_YUV444P10LE: AVPixelFormat = 68; //  planar YUV 4:4:4, 30bpp, (1 Cr & Cb sample per 1x1 Y samples), little-endian
pub const AV_PIX_FMT_YUV422P9BE: AVPixelFormat = 69; //  planar YUV 4:2:2, 18bpp, (1 Cr & Cb sample per 2x1 Y samples), big-endian
pub const AV_PIX_FMT_YUV422P9LE: AVPixelFormat = 70; //  planar YUV 4:2:2, 18bpp, (1 Cr & Cb sample per 2x1 Y samples), little-endian
pub const AV_PIX_FMT_GBRP: AVPixelFormat = 71; //  planar GBR 4:4:4 24bpp
pub const AV_PIX_FMT_GBR24P: AVPixelFormat = AV_PIX_FMT_GBRP; // alias for #AV_PIX_FMT_GBRP
pub const AV_PIX_FMT_GBRP9BE: AVPixelFormat = 72; //  planar GBR 4:4:4 27bpp, big-endian
pub const AV_PIX_FMT_GBRP9LE: AVPixelFormat = 73; //  planar GBR 4:4:4 27bpp, little-endian
pub const AV_PIX_FMT_GBRP10BE: AVPixelFormat = 74; //  planar GBR 4:4:4 30bpp, big-endian
pub const AV_PIX_FMT_GBRP10LE: AVPixelFormat = 75; //  planar GBR 4:4:4 30bpp, little-endian
pub const AV_PIX_FMT_GBRP16BE: AVPixelFormat = 76; //  planar GBR 4:4:4 48bpp, big-endian
pub const AV_PIX_FMT_GBRP16LE: AVPixelFormat = 77; //  planar GBR 4:4:4 48bpp, little-endian
pub const AV_PIX_FMT_YUVA422P: AVPixelFormat = 78; //  planar YUV 4:2:2 24bpp, (1 Cr & Cb sample per 2x1 Y & A samples)
pub const AV_PIX_FMT_YUVA444P: AVPixelFormat = 79; //  planar YUV 4:4:4 32bpp, (1 Cr & Cb sample per 1x1 Y & A samples)
pub const AV_PIX_FMT_YUVA420P9BE: AVPixelFormat = 80; //  planar YUV 4:2:0 22.5bpp, (1 Cr & Cb sample per 2x2 Y & A samples), big-endian
pub const AV_PIX_FMT_YUVA420P9LE: AVPixelFormat = 81; //  planar YUV 4:2:0 22.5bpp, (1 Cr & Cb sample per 2x2 Y & A samples), little-endian
pub const AV_PIX_FMT_YUVA422P9BE: AVPixelFormat = 82; //  planar YUV 4:2:2 27bpp, (1 Cr & Cb sample per 2x1 Y & A samples), big-endian
pub const AV_PIX_FMT_YUVA422P9LE: AVPixelFormat = 83; //  planar YUV 4:2:2 27bpp, (1 Cr & Cb sample per 2x1 Y & A samples), little-endian
pub const AV_PIX_FMT_YUVA444P9BE: AVPixelFormat = 84; //  planar YUV 4:4:4 36bpp, (1 Cr & Cb sample per 1x1 Y & A samples), big-endian
pub const AV_PIX_FMT_YUVA444P9LE: AVPixelFormat = 85; //  planar YUV 4:4:4 36bpp, (1 Cr & Cb sample per 1x1 Y & A samples), little-endian
pub const AV_PIX_FMT_YUVA420P10BE: AVPixelFormat = 86; //  planar YUV 4:2:0 25bpp, (1 Cr & Cb sample per 2x2 Y & A samples, big-endian)
pub const AV_PIX_FMT_YUVA420P10LE: AVPixelFormat = 87; //  planar YUV 4:2:0 25bpp, (1 Cr & Cb sample per 2x2 Y & A samples, little-endian)
pub const AV_PIX_FMT_YUVA422P10BE: AVPixelFormat = 88; //  planar YUV 4:2:2 30bpp, (1 Cr & Cb sample per 2x1 Y & A samples, big-endian)
pub const AV_PIX_FMT_YUVA422P10LE: AVPixelFormat = 89; //  planar YUV 4:2:2 30bpp, (1 Cr & Cb sample per 2x1 Y & A samples, little-endian)
pub const AV_PIX_FMT_YUVA444P10BE: AVPixelFormat = 90; //  planar YUV 4:4:4 40bpp, (1 Cr & Cb sample per 1x1 Y & A samples, big-endian)
pub const AV_PIX_FMT_YUVA444P10LE: AVPixelFormat = 91; //  planar YUV 4:4:4 40bpp, (1 Cr & Cb sample per 1x1 Y & A samples, little-endian)
pub const AV_PIX_FMT_YUVA420P16BE: AVPixelFormat = 92; //  planar YUV 4:2:0 40bpp, (1 Cr & Cb sample per 2x2 Y & A samples, big-endian)
pub const AV_PIX_FMT_YUVA420P16LE: AVPixelFormat = 93; //  planar YUV 4:2:0 40bpp, (1 Cr & Cb sample per 2x2 Y & A samples, little-endian)
pub const AV_PIX_FMT_YUVA422P16BE: AVPixelFormat = 94; //  planar YUV 4:2:2 48bpp, (1 Cr & Cb sample per 2x1 Y & A samples, big-endian)
pub const AV_PIX_FMT_YUVA422P16LE: AVPixelFormat = 95; //  planar YUV 4:2:2 48bpp, (1 Cr & Cb sample per 2x1 Y & A samples, little-endian)
pub const AV_PIX_FMT_YUVA444P16BE: AVPixelFormat = 96; //  planar YUV 4:4:4 64bpp, (1 Cr & Cb sample per 1x1 Y & A samples, big-endian)
pub const AV_PIX_FMT_YUVA444P16LE: AVPixelFormat = 97; //  planar YUV 4:4:4 64bpp, (1 Cr & Cb sample per 1x1 Y & A samples, little-endian)

pub const AV_PIX_FMT_VDPAU: AVPixelFormat = 98; //  HW acceleration through VDPAU, Picture.data[3] contains a VdpVideoSurface

pub const AV_PIX_FMT_XYZ12LE: AVPixelFormat = 99; //  packed XYZ 4:4:4, 36 bpp, (msb) 12X, 12Y, 12Z (lsb), the 2-byte value for each X/Y/Z is stored as little-endian, the 4 lower bits are set to 0
pub const AV_PIX_FMT_XYZ12BE: AVPixelFormat = 100; //  packed XYZ 4:4:4, 36 bpp, (msb) 12X, 12Y, 12Z (lsb), the 2-byte value for each X/Y/Z is stored as big-endian, the 4 lower bits are set to 0
pub const AV_PIX_FMT_NV16: AVPixelFormat = 101; //  interleaved chroma YUV 4:2:2, 16bpp, (1 Cr & Cb sample per 2x1 Y samples)
pub const AV_PIX_FMT_NV20LE: AVPixelFormat = 102; //  interleaved chroma YUV 4:2:2, 20bpp, (1 Cr & Cb sample per 2x1 Y samples), little-endian
pub const AV_PIX_FMT_NV20BE: AVPixelFormat = 103; //  interleaved chroma YUV 4:2:2, 20bpp, (1 Cr & Cb sample per 2x1 Y samples), big-endian

pub const AV_PIX_FMT_RGBA64BE: AVPixelFormat = 104; //  packed RGBA 16:16:16:16, 64bpp, 16R, 16G, 16B, 16A, the 2-byte value for each R/G/B/A component is stored as big-endian
pub const AV_PIX_FMT_RGBA64LE: AVPixelFormat = 105; //  packed RGBA 16:16:16:16, 64bpp, 16R, 16G, 16B, 16A, the 2-byte value for each R/G/B/A component is stored as little-endian
pub const AV_PIX_FMT_BGRA64BE: AVPixelFormat = 106; //  packed RGBA 16:16:16:16, 64bpp, 16B, 16G, 16R, 16A, the 2-byte value for each R/G/B/A component is stored as big-endian
pub const AV_PIX_FMT_BGRA64LE: AVPixelFormat = 107; //  packed RGBA 16:16:16:16, 64bpp, 16B, 16G, 16R, 16A, the 2-byte value for each R/G/B/A component is stored as little-endian

pub const AV_PIX_FMT_YVYU422: AVPixelFormat = 108; //  packed YUV 4:2:2, 16bpp, Y0 Cr Y1 Cb

pub const AV_PIX_FMT_YA16BE: AVPixelFormat = 109; //  16 bits gray, 16 bits alpha (big-endian)
pub const AV_PIX_FMT_YA16LE: AVPixelFormat = 110; //  16 bits gray, 16 bits alpha (little-endian)

pub const AV_PIX_FMT_GBRAP: AVPixelFormat = 111; //  planar GBRA 4:4:4:4 32bpp
pub const AV_PIX_FMT_GBRAP16BE: AVPixelFormat = 112; //  planar GBRA 4:4:4:4 64bpp, big-endian
pub const AV_PIX_FMT_GBRAP16LE: AVPixelFormat = 113; //  planar GBRA 4:4:4:4 64bpp, little-endian
/**
 *  HW acceleration through QSV, data[3] contains a pointer to the
 *  mfxFrameSurface1 structure.
 */
pub const AV_PIX_FMT_QSV: AVPixelFormat = 114;
/**
 * HW acceleration though MMAL, data[3] contains a pointer to the
 * MMAL_BUFFER_HEADER_T structure.
 */
pub const AV_PIX_FMT_MMAL: AVPixelFormat = 115;

pub const AV_PIX_FMT_D3D11VA_VLD: AVPixelFormat = 116; //  HW decoding through Direct3D11 via old API, Picture.data[3] contains a ID3D11VideoDecoderOutputView pointer

/**
 * HW acceleration through CUDA. data[i] contain CUdeviceptr pointers
 * exactly as for system memory frames.
 */
pub const AV_PIX_FMT_CUDA: AVPixelFormat = 117;

pub const AV_PIX_FMT_0RGB: AVPixelFormat = 118; //  packed RGB 8:8:8, 32bpp, XRGBXRGB...   X=unused/undefined
pub const AV_PIX_FMT_RGB0: AVPixelFormat = 119; //  packed RGB 8:8:8, 32bpp, RGBXRGBX...   X=unused/undefined
pub const AV_PIX_FMT_0BGR: AVPixelFormat = 120; //  packed BGR 8:8:8, 32bpp, XBGRXBGR...   X=unused/undefined
pub const AV_PIX_FMT_BGR0: AVPixelFormat = 121; //  packed BGR 8:8:8, 32bpp, BGRXBGRX...   X=unused/undefined

pub const AV_PIX_FMT_YUV420P12BE: AVPixelFormat = 122; //  planar YUV 4:2:0,18bpp, (1 Cr & Cb sample per 2x2 Y samples), big-endian
pub const AV_PIX_FMT_YUV420P12LE: AVPixelFormat = 123; //  planar YUV 4:2:0,18bpp, (1 Cr & Cb sample per 2x2 Y samples), little-endian
pub const AV_PIX_FMT_YUV420P14BE: AVPixelFormat = 124; //  planar YUV 4:2:0,21bpp, (1 Cr & Cb sample per 2x2 Y samples), big-endian
pub const AV_PIX_FMT_YUV420P14LE: AVPixelFormat = 125; //  planar YUV 4:2:0,21bpp, (1 Cr & Cb sample per 2x2 Y samples), little-endian
pub const AV_PIX_FMT_YUV422P12BE: AVPixelFormat = 126; //  planar YUV 4:2:2,24bpp, (1 Cr & Cb sample per 2x1 Y samples), big-endian
pub const AV_PIX_FMT_YUV422P12LE: AVPixelFormat = 127; //  planar YUV 4:2:2,24bpp, (1 Cr & Cb sample per 2x1 Y samples), little-endian
pub const AV_PIX_FMT_YUV422P14BE: AVPixelFormat = 128; //  planar YUV 4:2:2,28bpp, (1 Cr & Cb sample per 2x1 Y samples), big-endian
pub const AV_PIX_FMT_YUV422P14LE: AVPixelFormat = 129; //  planar YUV 4:2:2,28bpp, (1 Cr & Cb sample per 2x1 Y samples), little-endian
pub const AV_PIX_FMT_YUV444P12BE: AVPixelFormat = 130; //  planar YUV 4:4:4,36bpp, (1 Cr & Cb sample per 1x1 Y samples), big-endian
pub const AV_PIX_FMT_YUV444P12LE: AVPixelFormat = 131; //  planar YUV 4:4:4,36bpp, (1 Cr & Cb sample per 1x1 Y samples), little-endian
pub const AV_PIX_FMT_YUV444P14BE: AVPixelFormat = 132; //  planar YUV 4:4:4,42bpp, (1 Cr & Cb sample per 1x1 Y samples), big-endian
pub const AV_PIX_FMT_YUV444P14LE: AVPixelFormat = 133; //  planar YUV 4:4:4,42bpp, (1 Cr & Cb sample per 1x1 Y samples), little-endian
pub const AV_PIX_FMT_GBRP12BE: AVPixelFormat = 134; //  planar GBR 4:4:4 36bpp, big-endian
pub const AV_PIX_FMT_GBRP12LE: AVPixelFormat = 135; //  planar GBR 4:4:4 36bpp, little-endian
pub const AV_PIX_FMT_GBRP14BE: AVPixelFormat = 136; //  planar GBR 4:4:4 42bpp, big-endian
pub const AV_PIX_FMT_GBRP14LE: AVPixelFormat = 137; //  planar GBR 4:4:4 42bpp, little-endian
pub const AV_PIX_FMT_YUVJ411P: AVPixelFormat = 138; //  planar YUV 4:1:1, 12bpp, (1 Cr & Cb sample per 4x1 Y samples) full scale (JPEG), deprecated in favor of AV_PIX_FMT_YUV411P and setting color_range

pub const AV_PIX_FMT_BAYER_BGGR8: AVPixelFormat = 139; //  bayer, BGBG..(odd line), GRGR..(even line), 8-bit samples
pub const AV_PIX_FMT_BAYER_RGGB8: AVPixelFormat = 140; //  bayer, RGRG..(odd line), GBGB..(even line), 8-bit samples
pub const AV_PIX_FMT_BAYER_GBRG8: AVPixelFormat = 141; //  bayer, GBGB..(odd line), RGRG..(even line), 8-bit samples
pub const AV_PIX_FMT_BAYER_GRBG8: AVPixelFormat = 142; //  bayer, GRGR..(odd line), BGBG..(even line), 8-bit samples
pub const AV_PIX_FMT_BAYER_BGGR16LE: AVPixelFormat = 143; //  bayer, BGBG..(odd line), GRGR..(even line), 16-bit samples, little-endian
pub const AV_PIX_FMT_BAYER_BGGR16BE: AVPixelFormat = 144; //  bayer, BGBG..(odd line), GRGR..(even line), 16-bit samples, big-endian
pub const AV_PIX_FMT_BAYER_RGGB16LE: AVPixelFormat = 145; //  bayer, RGRG..(odd line), GBGB..(even line), 16-bit samples, little-endian
pub const AV_PIX_FMT_BAYER_RGGB16BE: AVPixelFormat = 146; //  bayer, RGRG..(odd line), GBGB..(even line), 16-bit samples, big-endian
pub const AV_PIX_FMT_BAYER_GBRG16LE: AVPixelFormat = 147; //  bayer, GBGB..(odd line), RGRG..(even line), 16-bit samples, little-endian
pub const AV_PIX_FMT_BAYER_GBRG16BE: AVPixelFormat = 148; //  bayer, GBGB..(odd line), RGRG..(even line), 16-bit samples, big-endian
pub const AV_PIX_FMT_BAYER_GRBG16LE: AVPixelFormat = 149; //  bayer, GRGR..(odd line), BGBG..(even line), 16-bit samples, little-endian
pub const AV_PIX_FMT_BAYER_GRBG16BE: AVPixelFormat = 150; //  bayer, GRGR..(odd line), BGBG..(even line), 16-bit samples, big-endian

pub const AV_PIX_FMT_XVMC: AVPixelFormat = 151; //  XVideo Motion Acceleration via common packet passing

pub const AV_PIX_FMT_YUV440P10LE: AVPixelFormat = 152; //  planar YUV 4:4:0,20bpp, (1 Cr & Cb sample per 1x2 Y samples), little-endian
pub const AV_PIX_FMT_YUV440P10BE: AVPixelFormat = 153; //  planar YUV 4:4:0,20bpp, (1 Cr & Cb sample per 1x2 Y samples), big-endian
pub const AV_PIX_FMT_YUV440P12LE: AVPixelFormat = 154; //  planar YUV 4:4:0,24bpp, (1 Cr & Cb sample per 1x2 Y samples), little-endian
pub const AV_PIX_FMT_YUV440P12BE: AVPixelFormat = 155; //  planar YUV 4:4:0,24bpp, (1 Cr & Cb sample per 1x2 Y samples), big-endian
pub const AV_PIX_FMT_AYUV64LE: AVPixelFormat = 156; //  packed AYUV 4:4:4,64bpp (1 Cr & Cb sample per 1x1 Y & A samples), little-endian
pub const AV_PIX_FMT_AYUV64BE: AVPixelFormat = 157; //  packed AYUV 4:4:4,64bpp (1 Cr & Cb sample per 1x1 Y & A samples), big-endian

pub const AV_PIX_FMT_VIDEOTOOLBOX: AVPixelFormat = 158; //  hardware decoding through Videotoolbox

pub const AV_PIX_FMT_P010LE: AVPixelFormat = 159; //  like NV12, with 10bpp per component, data in the high bits, zeros in the low bits, little-endian
pub const AV_PIX_FMT_P010BE: AVPixelFormat = 160; //  like NV12, with 10bpp per component, data in the high bits, zeros in the low bits, big-endian

pub const AV_PIX_FMT_GBRAP12BE: AVPixelFormat = 161; //  planar GBR 4:4:4:4 48bpp, big-endian
pub const AV_PIX_FMT_GBRAP12LE: AVPixelFormat = 162; //  planar GBR 4:4:4:4 48bpp, little-endian

pub const AV_PIX_FMT_GBRAP10BE: AVPixelFormat = 163; //  planar GBR 4:4:4:4 40bpp, big-endian
pub const AV_PIX_FMT_GBRAP10LE: AVPixelFormat = 164; //  planar GBR 4:4:4:4 40bpp, little-endian

pub const AV_PIX_FMT_MEDIACODEC: AVPixelFormat = 165; //  hardware decoding through MediaCodec

pub const AV_PIX_FMT_GRAY12BE: AVPixelFormat = 166; //         Y        , 12bpp, big-endian
pub const AV_PIX_FMT_GRAY12LE: AVPixelFormat = 167; //         Y        , 12bpp, little-endian
pub const AV_PIX_FMT_GRAY10BE: AVPixelFormat = 168; //         Y        , 10bpp, big-endian
pub const AV_PIX_FMT_GRAY10LE: AVPixelFormat = 169; //         Y        , 10bpp, little-endian

pub const AV_PIX_FMT_P016LE: AVPixelFormat = 170; //  like NV12, with 16bpp per component, little-endian
pub const AV_PIX_FMT_P016BE: AVPixelFormat = 171; //  like NV12, with 16bpp per component, big-endian

/**
 * Hardware surfaces for Direct3D11.
 *
 * This is preferred over the legacy AV_PIX_FMT_D3D11VA_VLD. The new D3D11
 * hwaccel API and filtering support AV_PIX_FMT_D3D11 only.
 *
 * data[0] contains a ID3D11Texture2D pointer, and data[1] contains the
 * texture array index of the frame as intptr_t if the ID3D11Texture2D is
 * an array texture (or always 0 if it's a normal texture).
 */
pub const AV_PIX_FMT_D3D11: AVPixelFormat = 172;

pub const AV_PIX_FMT_GRAY9BE: AVPixelFormat = 173; //         Y        , 9bpp, big-endian
pub const AV_PIX_FMT_GRAY9LE: AVPixelFormat = 174; //         Y        , 9bpp, little-endian

pub const AV_PIX_FMT_GBRPF32BE: AVPixelFormat = 175; //  IEEE-754 single precision planar GBR 4:4:4,     96bpp, big-endian
pub const AV_PIX_FMT_GBRPF32LE: AVPixelFormat = 176; //  IEEE-754 single precision planar GBR 4:4:4,     96bpp, little-endian
pub const AV_PIX_FMT_GBRAPF32BE: AVPixelFormat = 177; //  IEEE-754 single precision planar GBRA 4:4:4:4, 128bpp, big-endian
pub const AV_PIX_FMT_GBRAPF32LE: AVPixelFormat = 178; //  IEEE-754 single precision planar GBRA 4:4:4:4, 128bpp, little-endian

/**
 * DRM-managed buffers exposed through PRIME buffer sharing.
 *
 * data[0] points to an AVDRMFrameDescriptor.
 */
pub const AV_PIX_FMT_DRM_PRIME: AVPixelFormat = 179;
/**
 * Hardware surfaces for OpenCL.
 *
 * data[i] contain 2D image objects (typed in C as cl_mem, used
 * in OpenCL as image2d_t) for each plane of the surface.
 */
pub const AV_PIX_FMT_OPENCL: AVPixelFormat = 180;

pub const AV_PIX_FMT_GRAY14BE: AVPixelFormat = 181; //         Y        , 14bpp, big-endian
pub const AV_PIX_FMT_GRAY14LE: AVPixelFormat = 182; //         Y        , 14bpp, little-endian

pub const AV_PIX_FMT_GRAYF32BE: AVPixelFormat = 183; //  IEEE-754 single precision Y, 32bpp, big-endian
pub const AV_PIX_FMT_GRAYF32LE: AVPixelFormat = 184; //  IEEE-754 single precision Y, 32bpp, little-endian

pub const AV_PIX_FMT_YUVA422P12BE: AVPixelFormat = 185; //  planar YUV 4:2:2,24bpp, (1 Cr & Cb sample per 2x1 Y samples), 12b alpha, big-endian
pub const AV_PIX_FMT_YUVA422P12LE: AVPixelFormat = 186; //  planar YUV 4:2:2,24bpp, (1 Cr & Cb sample per 2x1 Y samples), 12b alpha, little-endian
pub const AV_PIX_FMT_YUVA444P12BE: AVPixelFormat = 187; //  planar YUV 4:4:4,36bpp, (1 Cr & Cb sample per 1x1 Y samples), 12b alpha, big-endian
pub const AV_PIX_FMT_YUVA444P12LE: AVPixelFormat = 188; //  planar YUV 4:4:4,36bpp, (1 Cr & Cb sample per 1x1 Y samples), 12b alpha, little-endian

pub const AV_PIX_FMT_NV24: AVPixelFormat = 189; //  planar YUV 4:4:4, 24bpp, 1 plane for Y and 1 plane for the UV components, which are interleaved (first byte U and the following byte V)
pub const AV_PIX_FMT_NV42: AVPixelFormat = 190; //  as above, but U and V bytes are swapped

/**
 * Vulkan hardware images.
 *
 * data[0] points to an AVVkFrame
 */
pub const AV_PIX_FMT_VULKAN: AVPixelFormat = 191;

pub const AV_PIX_FMT_Y210BE: AVPixelFormat = 192; //  packed YUV 4:2:2 like YUYV422, 20bpp, data in the high bits, big-endian
pub const AV_PIX_FMT_Y210LE: AVPixelFormat = 193; //  packed YUV 4:2:2 like YUYV422, 20bpp, data in the high bits, little-endian

pub const AV_PIX_FMT_X2RGB10LE: AVPixelFormat = 194; //  packed RGB 10:10:10, 30bpp, (msb)2X 10R 10G 10B(lsb), little-endian, X=unused/undefined
pub const AV_PIX_FMT_X2RGB10BE: AVPixelFormat = 195; //  packed RGB 10:10:10, 30bpp, (msb)2X 10R 10G 10B(lsb), big-endian, X=unused/undefined
pub const AV_PIX_FMT_X2BGR10LE: AVPixelFormat = 196; //  packed BGR 10:10:10, 30bpp, (msb)2X 10B 10G 10R(lsb), little-endian, X=unused/undefined
pub const AV_PIX_FMT_X2BGR10BE: AVPixelFormat = 197; //  packed BGR 10:10:10, 30bpp, (msb)2X 10B 10G 10R(lsb), big-endian, X=unused/undefined

pub const AV_PIX_FMT_P210BE: AVPixelFormat = 198; //  interleaved chroma YUV 4:2:2, 20bpp, data in the high bits, big-endian
pub const AV_PIX_FMT_P210LE: AVPixelFormat = 199; //  interleaved chroma YUV 4:2:2, 20bpp, data in the high bits, little-endian

pub const AV_PIX_FMT_P410BE: AVPixelFormat = 200; //  interleaved chroma YUV 4:4:4, 30bpp, data in the high bits, big-endian
pub const AV_PIX_FMT_P410LE: AVPixelFormat = 201; //  interleaved chroma YUV 4:4:4, 30bpp, data in the high bits, little-endian

pub const AV_PIX_FMT_P216BE: AVPixelFormat = 202; //  interleaved chroma YUV 4:2:2, 32bpp, big-endian
pub const AV_PIX_FMT_P216LE: AVPixelFormat = 203; //  interleaved chroma YUV 4:2:2, 32bpp, liddle-endian

pub const AV_PIX_FMT_P416BE: AVPixelFormat = 204; //  interleaved chroma YUV 4:4:4, 48bpp, big-endian
pub const AV_PIX_FMT_P416LE: AVPixelFormat = 205; //  interleaved chroma YUV 4:4:4, 48bpp, little-endian
