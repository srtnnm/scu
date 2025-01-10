use super::{DataRow, Module};

use crate::info::{get_option, SystemInformation};

pub struct Header;

impl Module for Header {
    const NAME: &'static str = "header";

    fn get(info: &SystemInformation) -> std::io::Result<DataRow> {
        let username = get_option("username", &info.username)?;
        let hostname = info.hostname.clone().unwrap_or_default();
        Ok(DataRow::nameless(format!("{username}@{hostname}")))
    }
}
