mod event_handler;
mod native;

use crate::App;
use slint::ComponentHandle;

pub fn run() -> anyhow::Result<()> {
    let app = App::new()?;

    event_handler::register_event_handler(&app);

    app.show()?;

    #[cfg(target_os = "windows")]
    native::windows::set_window_shadow();

    app.run()?;

    Ok(())
}
