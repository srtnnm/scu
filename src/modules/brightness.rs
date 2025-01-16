use super::Detection;

use libscu::hardware::display;

pub(super) fn fetch() -> Option<display::Brightness> {
    display::fetch_brightness().ok()
}

pub struct Brightness;

impl Detection for Brightness {
    type Result = display::Brightness;
    const NAME: &'static str = "brightness";

    fn fetch() -> std::io::Result<Self::Result> {
        display::fetch_brightness()
    }
}
