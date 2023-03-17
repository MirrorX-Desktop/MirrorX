use crate::window::desktop::render::constant;
use mirrorx_core::{component::frame::DesktopDecodeFrameFormat, DesktopDecodeFrame};
use std::sync::Arc;
use tauri_egui::eframe::{egui_glow::check_for_gl_error, glow::*};

pub struct Canvas {
    gl: Arc<Context>,
    program: Program,
    vao: NativeVertexArray,
    vbo: NativeBuffer,
    ebo: NativeBuffer,
    textures: Vec<NativeTexture>,
    frame_format: DesktopDecodeFrameFormat,
    destroyed: bool,
}

impl Canvas {
    pub fn new(
        gl: Arc<Context>,
        frame_width: i32,
        frame_height: i32,
        frame_format: DesktopDecodeFrameFormat,
    ) -> Result<Self, String> {
        unsafe {
            let (program, vao, vbo, ebo) = prepare_gl(&gl)?;

            let textures = match frame_format {
                DesktopDecodeFrameFormat::NV12 => {
                    vec![
                        create_texture(&gl, RED, frame_width, frame_height)?,
                        create_texture(&gl, RG, frame_width / 2, frame_height / 2)?,
                    ]
                }
                DesktopDecodeFrameFormat::YUV420P => {
                    vec![
                        create_texture(&gl, RED, frame_width, frame_height)?,
                        create_texture(&gl, RED, frame_width / 2, frame_height / 2)?,
                        create_texture(&gl, RED, frame_width / 2, frame_height / 2)?,
                    ]
                }
            };

            Ok(Self {
                gl,
                program,
                vao,
                vbo,
                ebo,
                textures,
                frame_format,
                destroyed: false,
            })
        }
    }

    pub fn paint(
        &self,
        gl: &Context,
        frame: Arc<DesktopDecodeFrame>,
        fbo: Option<tauri_egui::eframe::glow::Framebuffer>,
    ) -> Result<(), String> {
        if self.destroyed {
            return Err("desktop render has destroyed".into());
        }

        unsafe {
            gl.use_program(Some(self.program));
            check_for_gl_error!(gl);

            // disable srgb frame buffer since desktop frame has already adjust
            // to Rec.709
            gl.disable(FRAMEBUFFER_SRGB);
            check_for_gl_error!(gl);

            let use_nv12 = self.frame_format == DesktopDecodeFrameFormat::NV12;

            if use_nv12 {
                self.upload_nv12(gl, &frame);
            } else {
                self.upload_yuv420p(gl, &frame);
            }

            let use_nv12_uniform_location = gl.get_uniform_location(self.program, "use_nv12");
            check_for_gl_error!(gl);

            gl.uniform_1_i32(
                use_nv12_uniform_location.as_ref(),
                if use_nv12 { 1 } else { 0 },
            );
            check_for_gl_error!(gl);

            gl.bind_vertex_array(Some(self.vao));
            check_for_gl_error!(gl);

            gl.bind_framebuffer(tauri_egui::eframe::glow::FRAMEBUFFER, fbo);
            gl.draw_elements(TRIANGLES, 6, UNSIGNED_INT, 0);
            gl.bind_framebuffer(tauri_egui::eframe::glow::FRAMEBUFFER, None);

            Ok(())
        }
    }

    unsafe fn upload_nv12(&self, gl: &Context, frame: &DesktopDecodeFrame) {
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

    unsafe fn upload_yuv420p(&self, gl: &Context, frame: &DesktopDecodeFrame) {
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

impl Drop for Canvas {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_program(self.program);
            check_for_gl_error!(&self.gl);

            self.gl.delete_vertex_array(self.vao);
            check_for_gl_error!(&self.gl);

            self.gl.delete_buffer(self.vbo);
            check_for_gl_error!(&self.gl);

            self.gl.delete_buffer(self.ebo);
            check_for_gl_error!(&self.gl);

            for texture in self.textures.iter_mut() {
                self.gl.delete_texture(*texture);
                check_for_gl_error!(&self.gl);
            }
        }
    }
}

unsafe fn prepare_gl(
    gl: &Context,
) -> Result<(Program, NativeVertexArray, NativeBuffer, NativeBuffer), String> {
    tracing::info!("OpenGL version: {:?}", gl.version());

    let program = gl
        .create_program()
        .map_err(|err| format!("create program failed: {err}"))?;

    let vertex_shader = gl
        .create_shader(VERTEX_SHADER)
        .map_err(|err| format!("create vertex shader failed: {err}"))?;

    gl.shader_source(vertex_shader, constant::VERTEX_SHADER_SOURCE);
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
        .map_err(|err| format!("create fragment shader failed: {err}"))?;

    gl.shader_source(fragment_shader, constant::FRAGMENT_SHADER_SOURCE);
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
        .map_err(|err| format!("create ebo failed: {err}"))?;

    let vao = gl
        .create_vertex_array()
        .map_err(|err| format!("create vao failed: {err}"))?;

    let vbo = gl
        .create_buffer()
        .map_err(|err| format!("create vbo failed: {err}"))?;

    gl.bind_vertex_array(Some(vao));
    check_for_gl_error!(gl);

    gl.bind_buffer(ARRAY_BUFFER, Some(vbo));
    check_for_gl_error!(gl);

    gl.buffer_data_u8_slice(ARRAY_BUFFER, constant::VERTEX_VERTICES_SLICE, STATIC_DRAW);
    check_for_gl_error!(gl);

    gl.vertex_attrib_pointer_f32(0, 3, FLOAT, false, 5 * std::mem::size_of::<f32>() as i32, 0);
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

    gl.buffer_data_u8_slice(
        ELEMENT_ARRAY_BUFFER,
        constant::VERTICES_INDICES_SLICE,
        STATIC_DRAW,
    );
    check_for_gl_error!(gl);

    Ok((program, vao, vbo, ebo))
}

unsafe fn create_texture(
    gl: &Context,
    texture_format: u32,
    width: i32,
    height: i32,
) -> Result<NativeTexture, String> {
    let texture = gl
        .create_texture()
        .map_err(|err| format!("create texture failed: {err}"))?;

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
