use mirrorx_core::{api::config::LocalStorage, core_error, error::CoreResult};

#[tauri::command]
#[tracing::instrument]
pub async fn get_language() -> CoreResult<String> {
    let storage = LocalStorage::current()?;
    if let Some(language_code) = storage.kv().get_language()? {
        return Ok(language_code);
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

    storage.kv().set_language(&language_code)?;

    Ok(language_code)
}
