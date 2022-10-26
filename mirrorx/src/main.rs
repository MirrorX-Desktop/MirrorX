#[macro_use]
extern crate rust_i18n;

mod gui;
mod utility;

i18n!("locales");

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "warn,mirrorx=trace,mirrorx_core=trace");

    tracing_subscriber::fmt()
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let locale = if let Ok(locale) = select_language() {
        locale
    } else {
        tracing::error!("get system locale failed, fallback to 'en'");
        String::from("en")
    };

    rust_i18n::set_locale(&locale);

    if let Err(err) = crate::gui::run_app() {
        tracing::error!(?err, "run app failed");
    }
}

fn select_language() -> anyhow::Result<String> {
    let locale = sys_locale::get_locale().unwrap_or_else(|| String::from("en-US"));

    let language_tag = language_tags::LanguageTag::parse(&locale)?;

    let locale = if language_tag.primary_language() == "zh" {
        if let Some(script) = language_tag.script() {
            if script == "Hans" {
                "zh-CHS"
            } else {
                "zh-CHT"
            }
        } else if let Some(region) = language_tag.region() {
            if region == "CN" || region == "SG" {
                "zh-CHS"
            } else {
                "zh-CHT"
            }
        } else {
            "zh-CHS"
        }
    } else {
        language_tag.primary_language()
    };

    Ok(locale.to_owned())
}
