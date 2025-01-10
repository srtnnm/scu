use crate::display::modes::neomimic::row::DataRow;

use super::Module;

pub struct Kernel;

impl Module for Kernel {
    const NAME: &'static str = "kernel";

    fn get(
        info: &crate::info::SystemInformation,
    ) -> std::io::Result<crate::display::modes::neomimic::row::DataRow> {
        let kernel_version = info
            .kernel
            .clone()
            .and_then(|kernel| kernel.version)
            .unwrap_or_default();

        Ok(DataRow::info("Kernel", kernel_version))
    }
}
