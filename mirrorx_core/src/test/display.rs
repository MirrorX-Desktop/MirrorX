use sha2::{Digest, Sha256};
use tracing::info;

#[test]
fn test_get_active_displays() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let temp_dir = std::env::temp_dir();
    info!("screen shot will wirte to temp dir");

    let monitors = crate::component::desktop::monitor::get_active_monitors()?;
    for monitor in monitors {
        info!(id=?monitor.id,name=?monitor.name,refresh_rate=?monitor.refresh_rate,width=?monitor.width,height=?monitor.height,is_primary=?monitor.is_primary,screen_shot_buffer_length=?monitor.screen_shot.len(), "monitor");

        let mut hasher = Sha256::new();
        hasher.update(&monitor.screen_shot);
        let result = hasher.finalize();
        let screen_shot_hash = hex::encode_upper(result);

        let filename = temp_dir.join(format!("{}.png", screen_shot_hash));

        info!(?filename, "write screen shot");

        std::fs::write(filename, monitor.screen_shot)?;
    }

    Ok(())
}
