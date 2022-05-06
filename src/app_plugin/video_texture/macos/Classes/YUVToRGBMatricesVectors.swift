import simd

// BT.601, which is the standard for SDTV.
public let kColorConversion601: [Float] = [
    1.164, 1.164, 1.164,
    0.0, -0.392, 2.017,
    1.596, -0.813, 0.0
]

public let kColorConversion601SIMD: simd_float3x3 = simd_float3x3.init([
    simd_float3.init(1.164, 1.164, 1.164),
    simd_float3.init(0.0, -0.392, 2.017),
    simd_float3.init(1.596, -0.813, 0.0)
])

// BT.709, which is the standard for HDTV.
public let kColorConversion709: [Float] = [
    1.164, 1.164, 1.164,
    0.0, -0.213, 2.112,
    1.793, -0.533, 0.0
]

public let kColorConversion709SIMD: simd_float3x3 = simd_float3x3.init([
    simd_float3.init(1.164, 1.164, 1.164),
    simd_float3.init(0.0, -0.213, 2.112),
    simd_float3.init(1.793, -0.533, 0.0)
])

// BT.601 full range (ref: http://www.equasys.de/colorconversion.html)
public let kColorConversion601FullRange: [Float] = [
    1.0, 1.0, 1.0,
    0.0, -0.343, 1.765,
    1.4, -0.711, 0.0
]

public let kColorConversion601FullRangeSIMD: simd_float3x3 = simd_float3x3.init([
    simd_float3.init(1.0, 1.0, 1.0),
    simd_float3.init(0.0, -0.343, 1.765),
    simd_float3.init(1.4, -0.711, 0.0)
])

/*
 YUV 转 RGB 的变换 Translation

 Inspire by GPUImage
 */

public let kColorTranslationFullRange: [Float] = [
    0.0, -0.5, -0.5
]

public let kColorTranslationFullRangeSIMD: simd_float3 = simd_float3.init(0.0, -0.5, -0.5)

public let kColorTranslationVideoRange: [Float] = [
    -16.0 / 255.0, -0.5, -0.5
]

public let kColorTranslationVideoRangeSIMD: simd_float3 = simd_float3.init(-16.0 / 255.0, -0.5, -0.5)
