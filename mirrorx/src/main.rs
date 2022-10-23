mod gui;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    if let Err(err) = crate::gui::run_app() {
        tracing::error!(?err, "run app failed");
    }
}
