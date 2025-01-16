use super::Detection;

use libscu::software::locale;

pub(super) fn fetch() -> Option<String> {
    locale::fetch().ok()
}

pub struct Locale;

impl Detection for Locale {
    type Result = String;
    const NAME: &'static str = "locale";

    fn fetch() -> std::io::Result<Self::Result> {
        locale::fetch()
    }
}
