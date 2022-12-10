#[tauri::command]
#[tracing::instrument]
pub fn utility_generate_random_password() -> String {
    mirrorx_core::utility::rand::generate_random_password()
}
