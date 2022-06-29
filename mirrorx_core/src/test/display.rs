use tracing::info;

#[test]
fn test_get_active_displays() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let displays = match crate::component::display::get_active_displays() {
        Ok(displays) => displays,
        Err(err) => return Err(anyhow::anyhow!(err)),
    };

    if displays.len() == 0 {
        return Err(anyhow::anyhow!("display count is zero"));
    }

    for (index, dp) in displays.iter().enumerate() {
        info!(
            id = ?dp.id,
            is_main = ?dp.is_main,
            screen_shot_buffer_length = ?dp.screen_shot.len(),
            "display {}",
            index
        );
    }

    Ok(())
}
