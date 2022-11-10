use mirrorx_core::{api::config::LocalStorage, core_error, error::CoreResult};

#[tauri::command]
#[tracing::instrument]
pub fn init_language() -> CoreResult<()> {
    let storage = LocalStorage::current()?;
    if storage.kv().get_language()?.is_some() {
        return Ok(());
    }

    let system_locale = sys_locale::get_locale().unwrap_or_else(|| "en".into());
    let language_tag = language_tags::LanguageTag::parse(&system_locale)
        .map_err(|err| core_error!("parse language tag failed ({})", err))?;

    let mut language_code = language_tag.primary_language().to_string();

    if language_code == "zh" {
        if let Some(script) = language_tag.script() {
            language_code.push('-');
            language_code.push_str(script);
        }
    }

    LocalStorage::current()?.kv().set_language(&language_code)
}
