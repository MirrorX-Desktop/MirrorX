pub static VERTEX_SHADER_BYTES: &'static [u8] = include_bytes!("shader/vertex_shader.cso");
pub static PIXEL_SHADER_LUMINA_BYTES: &'static [u8] = include_bytes!("shader/pixel_shader_y.cso");
pub static PIXEL_SHADER_CHROMINANCE_BYTES: &'static [u8] =
    include_bytes!("shader/pixel_shader_uv.cso");
