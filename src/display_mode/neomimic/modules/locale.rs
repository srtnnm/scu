use super::{DataRow, ModuleTrait};

use crate::modules::{get_option, SystemInformation};

pub struct Locale;

impl ModuleTrait for Locale {
    const NAME: &'static str = "locale";

    fn run(info: &SystemInformation) -> std::io::Result<usize> {
        let locale = get_option("locale", &info.locale)?;
        Ok(DataRow::info("Locale", &locale))
    }
}
