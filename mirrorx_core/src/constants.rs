use once_cell::sync::OnceCell;

pub static OS_NAME: OnceCell<String> = OnceCell::new();
pub static OS_VERSION: OnceCell<String> = OnceCell::new();
