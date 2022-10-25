use egui_extras::RetainedImage;
use once_cell::sync::Lazy;

pub static FA_DOWN_LEFT_AND_UP_RIGHT_TO_CENTER_SOLID: Lazy<RetainedImage> = Lazy::new(|| {
    RetainedImage::from_svg_bytes(
        "fa_down-left-and-up-right-to-center-solid.svg",
        include_bytes!("../../../assets/icons/fa_down-left-and-up-right-to-center-solid.svg"),
    )
    .unwrap()
});

pub static FA_UP_RIGHT_AND_DOWN_LEFT_FROM_CENTER_SOLID: Lazy<RetainedImage> = Lazy::new(|| {
    RetainedImage::from_svg_bytes(
        "fa_up-right-and-down-left-from-center-solid.svg",
        include_bytes!("../../../assets/icons/fa_up-right-and-down-left-from-center-solid.svg"),
    )
    .unwrap()
});
