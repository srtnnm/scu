use super::{DataRow, Module};

use crate::info::SystemInformation;

pub struct Header;

impl Module for Header {
    const NAME: &'static str = "header";

    fn get(info: &SystemInformation) -> std::io::Result<DataRow> {
        let username = info.username.clone().unwrap_or_default();
        let hostname = info.hostname.clone().unwrap_or_default();
        Ok(DataRow::nameless(format!("{username}@{hostname}")))
    }
}
