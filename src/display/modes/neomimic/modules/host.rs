use crate::{display::modes::neomimic::row::DataRow, info::get_option};

use super::ModuleTrait;

pub struct Host;

impl ModuleTrait for Host {
    const NAME: &'static str = "host";

    fn get(
        info: &crate::info::SystemInformation,
    ) -> std::io::Result<crate::display::modes::neomimic::row::DataRow> {
        let device_name = get_option("device name", &info.device_name)?;

        Ok(DataRow::info("Host", device_name))
    }
}
