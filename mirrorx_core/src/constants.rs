use once_cell::sync::OnceCell;

pub static OS_TYPE: OnceCell<String> = OnceCell::new();
pub static OS_VERSION: OnceCell<String> = OnceCell::new();
