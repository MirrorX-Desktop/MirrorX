#[rustfmt::skip]
pub const VERTEX_VERTICES : [f32;20] = [
     1.0,  1.0, 0.0, 1.0, 1.0, 
     1.0, -1.0, 0.0, 1.0, 0.0, 
    -1.0, -1.0, 0.0, 0.0, 0.0, 
    -1.0,  1.0, 0.0, 0.0, 1.0,
];

pub const VERTEX_VERTICES_SLICE: &[u8] = unsafe {
    std::slice::from_raw_parts(
        VERTEX_VERTICES.as_ptr() as *const u8,
        VERTEX_VERTICES.len() * std::mem::size_of::<f32>(),
    )
};

#[rustfmt::skip]
pub const VERTICES_INDICES: [u32; 6] = [
    0, 1, 3, 
    1, 2, 3,
];

pub const VERTICES_INDICES_SLICE: &[u8] = unsafe {
    std::slice::from_raw_parts(
        VERTICES_INDICES.as_ptr() as *const u8,
        VERTICES_INDICES.len() * std::mem::size_of::<f32>(),
    )
};

pub const VERTEX_SHADER_SOURCE: &str = r#"
#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoord;

out vec2 texCoord;

void main(void)
{
    gl_Position = vec4(aPos, 1.0);
    texCoord = vec2(aTexCoord.x, 1 - aTexCoord.y);
}"#;

pub const FRAGMENT_SHADER_SOURCE: &str = r#"
#version 330 core

uniform int use_nv12;

uniform sampler2D nv12_textureY;
uniform sampler2D nv12_textureUV;

uniform sampler2D yuv420p_textureY;
uniform sampler2D yuv420p_textureU;
uniform sampler2D yuv420p_textureV;

in vec2 texCoord;
layout (location = 0) out vec4 fragColor;

const mat3 YCbCrToRGBmatrix = mat3(
    1.164, 0.000, 1.857,
    1.164,-0.217,-0.543,
    1.164, 2.016, 0.000
);

// const mat3 YCbCrToRGBmatrix = mat3(
//     1.000,   0.000,   1.570,
//     1.000,  -0.187,  -0.467,
//     1.000,   1.856,   0.000
// );

void main(void)
{
    vec3 yuv;
    vec3 rgb;
    if (use_nv12 == 1) {
        yuv.x = texture(nv12_textureY, texCoord).r - 0.0625;
        yuv.y = texture(nv12_textureUV, texCoord).r - 0.5;
        yuv.z = texture(nv12_textureUV, texCoord).g - 0.5;
    } else {
        yuv.x = texture(yuv420p_textureY, texCoord).r - 0.0625;
        yuv.y = texture(yuv420p_textureU, texCoord).r - 0.5;
        yuv.z = texture(yuv420p_textureV, texCoord).r - 0.5;
    }
    
    rgb = yuv * YCbCrToRGBmatrix;
    fragColor = vec4(rgb, 1.0);
}"#;
