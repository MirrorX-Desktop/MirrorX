#include <metal_stdlib>
//using namespace metal;
//
//constexpr sampler texSampler;
//
//typedef struct
//{
//    float4 position [[position]];
//    float2 texCoor;
//
//} VertexOut;
//
//vertex VertexOut
//RenderCameraYUVVertexShader(
//                            uint vertexID [[ vertex_id ]],
//                            constant float4 *position [[ buffer(0) ]],
//                            constant float2 *texCoor [[ buffer(1) ]]
//                            ) {
//    VertexOut out;
//
//    out.position = position[vertexID];
//    out.texCoor = texCoor[vertexID];
//
//    return out;
//}
//
//fragment float4
//RenderCameraYUVFragmentShader(
//                              VertexOut in [[ stage_in ]],
//                              texture2d<float, access::sample> lumaTex [[ texture(0) ]],
//                              texture2d<float, access::sample> chromaTex [[ texture(1) ]],
//                              constant float3x3 *YUV_To_RGB_Matrix [[ buffer(0) ]],
//                              constant float3 *YUV_Translation [[ buffer(1) ]]
//                              ) {
//    float3 yuv, rgb;
//
//    yuv.r = lumaTex.sample(texSampler, in.texCoor).r;
//    yuv.gb = chromaTex.sample(texSampler, in.texCoor).rg;
//
//    rgb = YUV_To_RGB_Matrix[0] * (yuv + YUV_Translation[0]);
//
//    return float4(rgb, 1);
//}
using namespace metal;

typedef struct {
    packed_float2 position;
    packed_float2 texcoord;
} Vertex;

typedef struct {
    float4 position[[position]];
    float2 texcoord;
} Varyings;

vertex Varyings vertexPassthrough( device Vertex * verticies[[buffer(0)]],
                                  unsigned int vid[[vertex_id]]) {
    Varyings out;
     device Vertex &v = verticies[vid];
    out.position = float4(float2(v.position), 0.0, 1.0);
    out.texcoord = v.texcoord;
    return out;
}

// Receiving YCrCb textures.
fragment half4 fragmentColorConversion(Varyings in[[stage_in]], texture2d<float, access::sample> textureY[[texture(0)]],
                                       texture2d<float, access::sample> textureCbCr[[texture(1)]]) {
    constexpr sampler s(address::clamp_to_edge, filter::linear);
    float y;
    float2 uv;
    y = textureY.sample(s, in.texcoord).r;
    uv = textureCbCr.sample(s, in.texcoord).rg - float2(0.5, 0.5);
    
    // Conversion for YUV to rgb from http://www.fourcc.org/fccyvrgb.php
    // float4 out = float4(y + 1.402 * uv.y, y - 0.344 * uv.x - 0.714 * uv.y, y + 1.772 * uv.x, 1.0);
    // https://forum.videolan.org/viewtopic.php?t=119032
    float4 out = float4(y + 1.5748 * uv.y, y - 0.187324 * uv.x - 0.468124 * uv.y, y + 1.8556 * uv.x, 1.0);
    return half4(out);
}
