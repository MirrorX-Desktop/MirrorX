use anyhow::bail;
use hassle_rs::{compile_hlsl, validate_dxil};
use once_cell::sync::OnceCell;

static VERTEX_SHADER_CODE: &'static str = include_str!("VertexShader.hlsl");
pub static VERTEX_SHADER: OnceCell<Vec<u8>> = OnceCell::new();

pub fn makesure_compile() -> anyhow::Result<()> {
    let _ = VERTEX_SHADER.get_or_try_init::<_, anyhow::Error>(|| {
        let compiled = compile_hlsl(
            "vertex_shader.hlsl",
            VERTEX_SHADER_CODE,
            "VS",
            "vs_4_0",
            &vec![],
            &vec![],
        )
        .or_else(|err| {
            bail!(
                "makesure_compile: compile 'VertexShader.hlsl' failed: {}",
                err
            )
        })?;

        let validated_compiled = validate_dxil(&compiled).or_else(|err| {
            bail!(
                "makesure_compile: validate 'VertexShader.hlsl' compiled target failed: {}",
                err
            )
        })?;

        Ok(validated_compiled)
    })?;

    Ok(())
}
