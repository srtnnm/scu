use super::{DataRow, ModuleTrait};

use crate::info::{get_option, SystemInformation};

pub struct Header;

impl ModuleTrait for Header {
    const NAME: &'static str = "header";

    fn run(info: &SystemInformation) -> std::io::Result<usize> {
        let username = get_option("username", &info.username)?;
        let hostname = info.hostname.clone().unwrap_or_default();
        Ok(DataRow::nameless(&format!("{username}@{hostname}")))
    }
}
