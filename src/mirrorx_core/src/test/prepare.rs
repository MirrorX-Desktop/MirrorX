use env_logger::{Builder, Target};
use log::LevelFilter;
use std::io::Write;

pub fn init_log() {
    Builder::new()
        .filter_level(LevelFilter::Info)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}] [{}({})] {} {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S.%3f"),
                record.module_path().unwrap_or(""),
                record.file().unwrap_or(""),
                record.level(),
                record.args()
            )
        })
        .target(Target::Stdout)
        .init();
}
