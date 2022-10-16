mod gui;
mod utility;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    crate::gui::run_app()
}
