use mirrorx_core::{component::frame::DesktopDecodeFrameFormat, DesktopDecodeFrame};
use tauri_egui::eframe::{egui_glow::check_for_gl_error, glow::*};

#[rustfmt::skip]
const VERTEX_VERTICES : [f32;20] = [
     1.0,  1.0, 0.0, 1.0, 1.0, 
     1.0, -1.0, 0.0, 1.0, 0.0, 
    -1.0, -1.0, 0.0, 0.0, 0.0, 
    -1.0,  1.0, 0.0, 0.0, 1.0,
];

const VERTEX_VERTICES_SLICE: &[u8] = unsafe {
    std::slice::from_raw_parts(
        VERTEX_VERTICES.as_ptr() as *const u8,
        VERTEX_VERTICES.len() * std::mem::size_of::<f32>(),
    )
};

#[rustfmt::skip]
const VERTICES_INDICES: [u32; 6] = [
    0, 1, 3, 
    1, 2, 3,
];

const VERTICES_INDICES_SLICE: &[u8] = unsafe {
    std::slice::from_raw_parts(
        VERTICES_INDICES.as_ptr() as *const u8,
        VERTICES_INDICES.len() * std::mem::size_of::<f32>(),
    )
};

pub struct Render {
    program: Program,
    textures: Vec<NativeTexture>,
    vao: NativeVertexArray,
    vbo: NativeBuffer,
    ebo: NativeBuffer,
    destroyed: bool,
    frame_rate: u16,
    frame_count: u16,
    frame_count_instant: Option<std::time::Instant>,
}

impl Render {
    pub fn new(gl: &Context) -> Result<Self, String> {
        unsafe {
            tracing::info!("OpenGL version: {:?}", gl.version());

            let program = gl
                .create_program()
                .map_err(|err| format!("create program failed: {}", err))?;

            let vertex_shader_source = r#"
            #version 330 core
            layout (location = 0) in vec3 aPos;
            layout (location = 1) in vec2 aTexCoord;

            out vec2 texCoord;
            
            void main(void)
            {
                gl_Position = vec4(aPos, 1.0);
                texCoord = vec2(aTexCoord.x, 1 - aTexCoord.y);
            }"#;

            let fragment_shader_source = r#"
            #version 330 core

            uniform int use_nv12;

            uniform sampler2D nv12_textureY;
            uniform sampler2D nv12_textureUV;

            uniform sampler2D yuv420p_textureY;
            uniform sampler2D yuv420p_textureU;
            uniform sampler2D yuv420p_textureV;

            in vec2 texCoord;
            layout (location = 0) out vec4 fragColor;

            // const mat3 YCbCrToRGBmatrix = mat3(
            //     1.164, 0.000, 1.857,
            //     1.164,-0.217,-0.543,
            //     1.164, 2.016, 0.000
            // );

            const mat3 YCbCrToRGBmatrix = mat3(
                1.000,   0.000,   1.570,
                1.000,  -0.187,  -0.467,
                1.000,   1.856,   0.000
            );

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

            let vertex_shader = gl
                .create_shader(VERTEX_SHADER)
                .map_err(|err| format!("create vertex shader failed: {}", err))?;

            gl.shader_source(vertex_shader, vertex_shader_source);
            check_for_gl_error!(gl);

            gl.compile_shader(vertex_shader);
            check_for_gl_error!(gl);

            if !gl.get_shader_compile_status(vertex_shader) {
                return Err(format!(
                    "compile vertex shader failed: {}",
                    gl.get_shader_info_log(vertex_shader)
                ));
            }

            let fragment_shader = gl
                .create_shader(FRAGMENT_SHADER)
                .map_err(|err| format!("create fragment shader failed: {}", err))?;

            gl.shader_source(fragment_shader, fragment_shader_source);
            check_for_gl_error!(gl);

            gl.compile_shader(fragment_shader);
            check_for_gl_error!(gl);

            if !gl.get_shader_compile_status(fragment_shader) {
                return Err(format!(
                    "compile fragment shader failed: {}",
                    gl.get_shader_info_log(vertex_shader)
                ));
            }

            gl.attach_shader(program, vertex_shader);
            check_for_gl_error!(gl);

            gl.attach_shader(program, fragment_shader);
            check_for_gl_error!(gl);

            gl.bind_attrib_location(program, 0, "aPos");
            check_for_gl_error!(gl);

            gl.bind_attrib_location(program, 1, "aTexCoord");
            check_for_gl_error!(gl);

            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                return Err(format!(
                    "link program failed: {}",
                    gl.get_program_info_log(program)
                ));
            }

            gl.detach_shader(program, vertex_shader);
            check_for_gl_error!(gl);

            gl.detach_shader(program, fragment_shader);
            check_for_gl_error!(gl);

            gl.delete_shader(vertex_shader);
            check_for_gl_error!(gl);

            gl.delete_shader(fragment_shader);
            check_for_gl_error!(gl);

            let ebo = gl
                .create_buffer()
                .map_err(|err| format!("create ebo failed: {}", err))?;

            let vao = gl
                .create_vertex_array()
                .map_err(|err| format!("create vao failed: {}", err))?;

            let vbo = gl
                .create_buffer()
                .map_err(|err| format!("create vbo failed: {}", err))?;

            gl.bind_vertex_array(Some(vao));
            check_for_gl_error!(gl);

            gl.bind_buffer(ARRAY_BUFFER, Some(vbo));
            check_for_gl_error!(gl);

            gl.buffer_data_u8_slice(ARRAY_BUFFER, VERTEX_VERTICES_SLICE, STATIC_DRAW);
            check_for_gl_error!(gl);

            gl.vertex_attrib_pointer_f32(
                0,
                3,
                FLOAT,
                false,
                5 * std::mem::size_of::<f32>() as i32,
                0,
            );
            check_for_gl_error!(gl);

            gl.vertex_attrib_pointer_f32(
                1,
                2,
                FLOAT,
                false,
                5 * std::mem::size_of::<f32>() as i32,
                3 * std::mem::size_of::<f32>() as i32,
            );
            check_for_gl_error!(gl);

            gl.enable_vertex_attrib_array(0);
            check_for_gl_error!(gl);

            gl.enable_vertex_attrib_array(1);
            check_for_gl_error!(gl);

            gl.bind_buffer(ELEMENT_ARRAY_BUFFER, Some(ebo));
            check_for_gl_error!(gl);

            gl.buffer_data_u8_slice(ELEMENT_ARRAY_BUFFER, VERTICES_INDICES_SLICE, STATIC_DRAW);
            check_for_gl_error!(gl);

            Ok(Self {
                program,
                textures: Vec::new(),
                vao,
                vbo,
                ebo,
                destroyed: false,
                frame_rate: 0,
                frame_count: 0,
                frame_count_instant: None,
            })
        }
    }

    pub fn frame_rate(&self) -> u16 {
        self.frame_rate
    }

    pub fn destroy(&mut self, gl: &Context) {
        self.destroyed = true;

        unsafe {
            gl.delete_program(self.program);
            check_for_gl_error!(gl);

            gl.delete_vertex_array(self.vao);
            check_for_gl_error!(gl);

            gl.delete_buffer(self.vbo);
            check_for_gl_error!(gl);

            gl.delete_buffer(self.ebo);
            check_for_gl_error!(gl);

            for texture in self.textures.iter_mut() {
                gl.delete_texture(*texture);
                check_for_gl_error!(gl);
            }
        }
    }

    pub fn paint(
        &mut self,
        gl: &Context,
        frame: DesktopDecodeFrame,
        fbo: Option<tauri_egui::eframe::glow::Framebuffer>,
    ) -> Result<(), String> {
        if self.destroyed {
            return Err("desktop render has destroyed".into());
        }

        unsafe {
            if self.textures.is_empty() {
                match frame.format {
                    DesktopDecodeFrameFormat::NV12 => {
                        self.textures
                            .push(create_texture(gl, RED, frame.width, frame.height)?);

                        self.textures.push(create_texture(
                            gl,
                            RG,
                            frame.width / 2,
                            frame.height / 2,
                        )?);
                    }
                    DesktopDecodeFrameFormat::YUV420P => {
                        self.textures
                            .push(create_texture(gl, RED, frame.width, frame.height)?);

                        self.textures.push(create_texture(
                            gl,
                            RED,
                            frame.width / 2,
                            frame.height / 2,
                        )?);

                        self.textures.push(create_texture(
                            gl,
                            RED,
                            frame.width / 2,
                            frame.height / 2,
                        )?);
                    }
                }
            };

            if self.frame_count_instant.is_none() {
                self.frame_count_instant = Some(std::time::Instant::now());
            }

            gl.use_program(Some(self.program));
            check_for_gl_error!(gl);

            // disable srgb frame buffer since desktop frame has already adjust
            // to Rec.709
            gl.disable(FRAMEBUFFER_SRGB);
            check_for_gl_error!(gl);

            let use_nv12_value = match frame.format {
                DesktopDecodeFrameFormat::NV12 => {
                    self.upload_nv12(gl, &frame);
                    1
                }
                DesktopDecodeFrameFormat::YUV420P => {
                    self.upload_yuv420p(gl, &frame);
                    0
                }
            };

            let use_nv12_uniform_location = gl.get_uniform_location(self.program, "use_nv12");
            check_for_gl_error!(gl);

            gl.uniform_1_i32(use_nv12_uniform_location.as_ref(), use_nv12_value);
            check_for_gl_error!(gl);

            gl.bind_vertex_array(Some(self.vao));
            check_for_gl_error!(gl);

            gl.bind_framebuffer(tauri_egui::eframe::glow::FRAMEBUFFER, fbo);
            gl.draw_elements(TRIANGLES, 6, UNSIGNED_INT, 0);
            gl.bind_framebuffer(tauri_egui::eframe::glow::FRAMEBUFFER, None);

            if let Some(instant) = self.frame_count_instant {
                if instant.elapsed().as_secs() >= 1 {
                    self.frame_rate = self.frame_count;
                    self.frame_count = 0;
                    self.frame_count_instant = Some(std::time::Instant::now());
                }
            }

            self.frame_count += 1;

            Ok(())
        }
    }

    unsafe fn upload_nv12(&mut self, gl: &Context, frame: &DesktopDecodeFrame) {
        // upload Y plane
        gl.active_texture(TEXTURE0);
        check_for_gl_error!(gl);

        gl.bind_texture(TEXTURE_2D, Some(self.textures[0]));
        check_for_gl_error!(gl);

        gl.pixel_store_i32(UNPACK_ROW_LENGTH, frame.line_sizes[0]);
        check_for_gl_error!(gl);

        gl.tex_sub_image_2d(
            TEXTURE_2D,
            0,
            0,
            0,
            frame.width,
            frame.height,
            RED,
            UNSIGNED_BYTE,
            PixelUnpackData::Slice(&frame.plane_data[0]),
        );
        check_for_gl_error!(gl);

        let y_uniform_location = gl.get_uniform_location(self.program, "nv12_textureY");
        check_for_gl_error!(gl);

        gl.uniform_1_i32(y_uniform_location.as_ref(), 0);
        check_for_gl_error!(gl);

        // upload UV plane
        gl.active_texture(TEXTURE1);
        check_for_gl_error!(gl);

        gl.bind_texture(TEXTURE_2D, Some(self.textures[1]));
        check_for_gl_error!(gl);

        gl.pixel_store_i32(UNPACK_ROW_LENGTH, frame.line_sizes[1]);
        check_for_gl_error!(gl);

        gl.tex_sub_image_2d(
            TEXTURE_2D,
            0,
            0,
            0,
            frame.width / 2,
            frame.height / 2,
            RG,
            UNSIGNED_BYTE,
            PixelUnpackData::Slice(&frame.plane_data[1]),
        );
        check_for_gl_error!(gl);

        let uv_uniform_location = gl.get_uniform_location(self.program, "nv12_textureUV");
        check_for_gl_error!(gl);

        gl.uniform_1_i32(uv_uniform_location.as_ref(), 1);
        check_for_gl_error!(gl);

        // important: reset UNPACK_ROW_LENGTH to zero otherwise it will affect egui texture upload and cause unexpected behavior
        gl.pixel_store_i32(UNPACK_ROW_LENGTH, 0);
    }

    unsafe fn upload_yuv420p(&mut self, gl: &Context, frame: &DesktopDecodeFrame) {
        // upload Y plane
        gl.active_texture(TEXTURE0);
        check_for_gl_error!(gl);

        gl.bind_texture(TEXTURE_2D, Some(self.textures[0]));
        check_for_gl_error!(gl);

        gl.pixel_store_i32(UNPACK_ROW_LENGTH, frame.line_sizes[0]);
        check_for_gl_error!(gl);

        gl.tex_sub_image_2d(
            TEXTURE_2D,
            0,
            0,
            0,
            frame.width,
            frame.height,
            RED,
            UNSIGNED_BYTE,
            PixelUnpackData::Slice(&frame.plane_data[0]),
        );
        check_for_gl_error!(gl);

        let y_uniform_location = gl.get_uniform_location(self.program, "yuv420p_textureY");
        check_for_gl_error!(gl);

        gl.uniform_1_i32(y_uniform_location.as_ref(), 0);
        check_for_gl_error!(gl);

        // upload U plane
        gl.active_texture(TEXTURE1);
        check_for_gl_error!(gl);

        gl.bind_texture(TEXTURE_2D, Some(self.textures[1]));
        check_for_gl_error!(gl);

        gl.pixel_store_i32(UNPACK_ROW_LENGTH, frame.line_sizes[1]);
        check_for_gl_error!(gl);

        gl.tex_sub_image_2d(
            TEXTURE_2D,
            0,
            0,
            0,
            frame.width / 2,
            frame.height / 2,
            RED,
            UNSIGNED_BYTE,
            PixelUnpackData::Slice(&frame.plane_data[1]),
        );
        check_for_gl_error!(gl);

        let u_uniform_location = gl.get_uniform_location(self.program, "yuv420p_textureU");
        check_for_gl_error!(gl);

        gl.uniform_1_i32(u_uniform_location.as_ref(), 1);
        check_for_gl_error!(gl);

        // upload V plane
        gl.active_texture(TEXTURE2);
        check_for_gl_error!(gl);

        gl.bind_texture(TEXTURE_2D, Some(self.textures[2]));
        check_for_gl_error!(gl);

        gl.pixel_store_i32(UNPACK_ROW_LENGTH, frame.line_sizes[2]);
        check_for_gl_error!(gl);

        gl.tex_sub_image_2d(
            TEXTURE_2D,
            0,
            0,
            0,
            frame.width / 2,
            frame.height / 2,
            RED,
            UNSIGNED_BYTE,
            PixelUnpackData::Slice(&frame.plane_data[2]),
        );
        check_for_gl_error!(gl);

        let v_uniform_location = gl.get_uniform_location(self.program, "yuv420p_textureV");
        check_for_gl_error!(gl);

        gl.uniform_1_i32(v_uniform_location.as_ref(), 2);
        check_for_gl_error!(gl);

        // important: reset UNPACK_ROW_LENGTH to zero otherwise it will affect egui texture upload and cause unexpected behavior
        gl.pixel_store_i32(UNPACK_ROW_LENGTH, 0);
    }
}

unsafe fn create_texture(
    gl: &Context,
    texture_format: u32,
    width: i32,
    height: i32,
) -> Result<NativeTexture, String> {
    let texture = gl
        .create_texture()
        .map_err(|err| format!("create texture failed: {}", err))?;

    gl.bind_texture(TEXTURE_2D, Some(texture));
    check_for_gl_error!(gl);

    gl.tex_image_2d(
        TEXTURE_2D,
        0,
        texture_format as i32,
        width,
        height,
        0,
        texture_format,
        UNSIGNED_BYTE,
        None,
    );
    check_for_gl_error!(gl);

    gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR as i32);
    check_for_gl_error!(gl);

    gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as i32);
    check_for_gl_error!(gl);

    gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_S, CLAMP_TO_EDGE as i32);
    check_for_gl_error!(gl);

    gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_T, CLAMP_TO_EDGE as i32);
    check_for_gl_error!(gl);

    Ok(texture)
}
