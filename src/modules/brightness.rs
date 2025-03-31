use super::Detection;

use libscu::hardware::display;

pub struct Brightness;

impl Detection for Brightness {
    type Result = display::Brightness;
    const NAME: &'static str = "brightness";

    fn fetch(&self) -> std::io::Result<Self::Result> {
        display::fetch_brightness()
    }
}
