use super::{DataRow, ModuleTrait};

use crate::info::get_option;

pub struct Host;

impl ModuleTrait for Host {
    const NAME: &'static str = "host";

    fn run(info: &crate::info::SystemInformation) -> std::io::Result<usize> {
        let device_name = get_option("device name", &info.device_name)?;

        Ok(DataRow::info("Host", &device_name))
    }
}
