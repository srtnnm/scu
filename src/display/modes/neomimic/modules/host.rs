use crate::display::modes::neomimic::row::DataRow;

use super::Module;

pub struct Host;

impl Module for Host {
    const NAME: &'static str = "host";

    fn get(
        info: &crate::info::SystemInformation,
    ) -> std::io::Result<crate::display::modes::neomimic::row::DataRow> {
        let device_name = info.device_name.clone().unwrap_or_default();

        Ok(DataRow::info("Host", device_name))
    }
}
