use egui_extras::RetainedImage;
use once_cell::sync::{Lazy, OnceCell};

// Fonts

pub static FONT_MATERIAL_SYMBOLS: &[u8] =
    include_bytes!("../../assets/fonts/MaterialSymbolsRounded[FILL,GRAD,opsz,wght].ttf");
pub static FONT_NOTO_SANS: &[u8] = include_bytes!("../../assets/fonts/NotoSans-Regular.ttf");
pub static FONT_NOTO_SANS_MONO: &[u8] =
    include_bytes!("../../assets/fonts/NotoSansMono-Regular.ttf");
pub static FONT_NOTO_SANS_SC: &[u8] = include_bytes!("../../assets/fonts/NotoSansSC-Regular.otf");
pub static FONT_NOTO_SANS_TC: &[u8] = include_bytes!("../../assets/fonts/NotoSansTC-Regular.otf");
pub static FONT_NOTO_SANS_JP: &[u8] = include_bytes!("../../assets/fonts/NotoSansJP-Regular.otf");
pub static FONT_NOTO_SANS_KR: &[u8] = include_bytes!("../../assets/fonts/NotoSansKR-Regular.otf");

// Images

static IMAGE_LOGO_1024: &[u8] = include_bytes!("../../assets/images/logo_1024.svg");
static IMAGE_CLOSE_48: &[u8] =
    include_bytes!("../../assets/images/close_FILL0_wght400_GRAD0_opsz48.svg");
static IMAGE_REMOVE_48: &[u8] =
    include_bytes!("../../assets/images/remove_FILL0_wght400_GRAD0_opsz48.svg");
static IMAGE_ARROW_FORWARD_48: &[u8] =
    include_bytes!("../../assets/images/arrow_forward_FILL0_wght400_GRAD0_opsz48.svg");
static IMAGE_DESKTOP_WINDOWS_48: &[u8] =
    include_bytes!("../../assets/images/desktop_windows_FILL0_wght400_GRAD0_opsz48.svg");
static IMAGE_EXPAND_MORE_48: &[u8] =
    include_bytes!("../../assets/images/expand_more_FILL0_wght400_GRAD0_opsz48.svg");
static IMAGE_FOLDER_48: &[u8] =
    include_bytes!("../../assets/images/folder_FILL0_wght400_GRAD0_opsz48.svg");
static IMAGE_HISTORY_TOGGLE_OFF_48: &[u8] =
    include_bytes!("../../assets/images/history_toggle_off_FILL0_wght400_GRAD0_opsz48.svg");
static IMAGE_LAN_48: &[u8] =
    include_bytes!("../../assets/images/lan_FILL0_wght400_GRAD0_opsz48.svg");
static IMAGE_TUNE_48: &[u8] =
    include_bytes!("../../assets/images/tune_FILL0_wght400_GRAD0_opsz48.svg");

// Cache

static STATIC_IMAGE_CACHE: Lazy<OnceCell<StaticImageCache>> = Lazy::new(OnceCell::new);

pub struct StaticImageCache {
    pub logo_1024: RetainedImage,
    pub arrow_forward_48: RetainedImage,
    pub close_48: RetainedImage,
    pub desktop_windows_48: RetainedImage,
    pub expand_more_48: RetainedImage,
    pub folder_48: RetainedImage,
    pub history_toggle_off_48: RetainedImage,
    pub lan_48: RetainedImage,
    pub remove_48: RetainedImage,
    pub tune_48: RetainedImage,
}

impl StaticImageCache {
    pub fn load() -> anyhow::Result<()> {
        let image_logo_1024 = RetainedImage::from_svg_bytes("image_logo_1024", IMAGE_LOGO_1024)
            .map_err(|err| anyhow::anyhow!(err))?;

        let image_arrow_forward_48 =
            RetainedImage::from_svg_bytes("image_arrow_forward_48", IMAGE_ARROW_FORWARD_48)
                .map_err(|err| anyhow::anyhow!(err))?;

        let image_close_48 = RetainedImage::from_svg_bytes("image_close_48", IMAGE_CLOSE_48)
            .map_err(|err| anyhow::anyhow!(err))?;

        let image_desktop_windows_48 =
            RetainedImage::from_svg_bytes("image_desktop_windows_48", IMAGE_DESKTOP_WINDOWS_48)
                .map_err(|err| anyhow::anyhow!(err))?;

        let image_expand_more_48 =
            RetainedImage::from_svg_bytes("image_expand_more_48", IMAGE_EXPAND_MORE_48)
                .map_err(|err| anyhow::anyhow!(err))?;

        let image_folder_48 = RetainedImage::from_svg_bytes("image_folder_48", IMAGE_FOLDER_48)
            .map_err(|err| anyhow::anyhow!(err))?;

        let image_history_toggle_off_48 = RetainedImage::from_svg_bytes(
            "image_history_toggle_off_48",
            IMAGE_HISTORY_TOGGLE_OFF_48,
        )
        .map_err(|err| anyhow::anyhow!(err))?;

        let image_lan_48 = RetainedImage::from_svg_bytes("image_lan_48", IMAGE_LAN_48)
            .map_err(|err| anyhow::anyhow!(err))?;

        let image_remove_48 = RetainedImage::from_svg_bytes("image_remove_48", IMAGE_REMOVE_48)
            .map_err(|err| anyhow::anyhow!(err))?;

        let image_tune_48 = RetainedImage::from_svg_bytes("image_tune_48", IMAGE_TUNE_48)
            .map_err(|err| anyhow::anyhow!(err))?;

        let cache = StaticImageCache {
            logo_1024: image_logo_1024,
            arrow_forward_48: image_arrow_forward_48,
            close_48: image_close_48,
            desktop_windows_48: image_desktop_windows_48,
            expand_more_48: image_expand_more_48,
            folder_48: image_folder_48,
            history_toggle_off_48: image_history_toggle_off_48,
            lan_48: image_lan_48,
            remove_48: image_remove_48,
            tune_48: image_tune_48,
        };

        anyhow::ensure!(
            STATIC_IMAGE_CACHE.set(cache).is_ok(),
            "static image cache should be initialized only once"
        );

        Ok(())
    }

    pub fn current<'a>() -> &'a StaticImageCache {
        match STATIC_IMAGE_CACHE.get() {
            Some(cache) => cache,
            None => panic!("static image cache hasn't initialized"),
        }
    }
}
