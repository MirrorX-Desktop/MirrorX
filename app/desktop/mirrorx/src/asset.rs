use egui_extras::RetainedImage;
use once_cell::sync::Lazy;

pub static IMAGE_LOGO: Lazy<RetainedImage> = Lazy::new(|| {
    RetainedImage::from_image_bytes(
        "logo",
        include_bytes!("../assets/images/logo-1024x1024.png"),
    )
    .unwrap()
});

pub static IMAGE_SMB_SHARE: Lazy<RetainedImage> = Lazy::new(|| {
    RetainedImage::from_image_bytes(
        "google_icons_smb_share",
        include_bytes!("../assets/images/google_icons/smb_share_FILL1_wght400_GRAD0_opsz48.png"),
    )
    .unwrap()
});

pub static IMAGE_HUB: Lazy<RetainedImage> = Lazy::new(|| {
    RetainedImage::from_image_bytes(
        "google_icons_hub",
        include_bytes!("../assets/images/google_icons/hub_FILL1_wght400_GRAD0_opsz48.png"),
    )
    .unwrap()
});

pub static IMAGE_FAMILY_HISTORY: Lazy<RetainedImage> = Lazy::new(|| {
    RetainedImage::from_image_bytes(
        "google_icons_family_history",
        include_bytes!(
            "../assets/images/google_icons/family_history_FILL1_wght400_GRAD0_opsz48.png"
        ),
    )
    .unwrap()
});

pub static IMAGE_HISTORY: Lazy<RetainedImage> = Lazy::new(|| {
    RetainedImage::from_image_bytes(
        "google_icons_history",
        include_bytes!("../assets/images/google_icons/history_FILL1_wght400_GRAD0_opsz48.png"),
    )
    .unwrap()
});

pub static IMAGE_TUNE: Lazy<RetainedImage> = Lazy::new(|| {
    RetainedImage::from_image_bytes(
        "google_icons_tune",
        include_bytes!("../assets/images/google_icons/tune_FILL1_wght400_GRAD0_opsz48.png"),
    )
    .unwrap()
});
