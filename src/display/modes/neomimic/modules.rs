mod header;
pub use header::Header;

use super::row::DataRow;

use crate::info::SystemInformation;

use std::io;

pub(crate) trait Module {
    const NAME: &'static str;

    fn get(info: &SystemInformation) -> io::Result<DataRow>;
}
