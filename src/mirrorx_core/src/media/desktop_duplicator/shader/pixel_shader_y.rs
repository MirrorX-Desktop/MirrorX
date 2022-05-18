use anyhow::bail;
use hassle_rs::{compile_hlsl, validate_dxil};
use once_cell::sync::OnceCell;

static PIXEL_SHADER_Y_CODE: &'static str = include_str!("PixelShaderY.hlsl");
pub static PIXEL_SHADER_Y: OnceCell<Vec<u8>> = OnceCell::new();

pub fn makesure_compile() -> anyhow::Result<()> {
    let _ = PIXEL_SHADER_Y.get_or_try_init::<_, anyhow::Error>(|| {
        let compiled = compile_hlsl(
            "pixel_shader_y.hlsl",
            PIXEL_SHADER_Y_CODE,
            "PS_Y",
            "ps_4_0",
            &vec![],
            &vec![],
        )
        .or_else(|err| {
            bail!(
                "makesure_compile: compile 'PixelShaderY.hlsl' failed: {}",
                err
            )
        })?;

        let validated_compiled = validate_dxil(&compiled).or_else(|err| {
            bail!(
                "makesure_compile: validate 'PixelShaderY.hlsl' compiled target failed: {}",
                err
            )
        })?;

        Ok(validated_compiled)
    })?;

    Ok(())
}
