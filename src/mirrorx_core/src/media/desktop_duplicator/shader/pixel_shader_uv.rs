use anyhow::bail;
use hassle_rs::{compile_hlsl, validate_dxil};
use once_cell::sync::OnceCell;

static PIXEL_SHADER_UV_CODE: &'static str = include_str!("PixelShaderUV.hlsl");
pub static PIXEL_SHADER_UV: OnceCell<Vec<u8>> = OnceCell::new();

pub fn makesure_compile() -> anyhow::Result<()> {
    let _ = PIXEL_SHADER_UV.get_or_try_init::<_, anyhow::Error>(|| {
        let compiled = compile_hlsl(
            "pixel_shader_uv.hlsl",
            PIXEL_SHADER_UV_CODE,
            "PS_UV",
            "ps_4_0",
            &vec![],
            &vec![],
        )
        .or_else(|err| {
            bail!(
                "makesure_compile: compile 'PixelShaderUV.hlsl' failed: {}",
                err
            )
        })?;

        let validated_compiled = validate_dxil(&compiled).or_else(|err| {
            bail!(
                "makesure_compile: validate 'PixelShaderUV.hlsl' compiled target failed: {}",
                err
            )
        })?;

        Ok(validated_compiled)
    })?;

    Ok(())
}
