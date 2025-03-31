use super::Detection;

use libscu::software::locale;

pub struct Locale;

impl Detection for Locale {
    type Result = String;
    const NAME: &'static str = "locale";

    fn fetch(&self) -> std::io::Result<Self::Result> {
        locale::fetch()
    }
}
