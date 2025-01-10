use crate::{display::modes::neomimic::row::DataRow, info::get_option};

use super::Module;

pub struct Kernel;

impl Module for Kernel {
    const NAME: &'static str = "kernel";

    fn get(
        info: &crate::info::SystemInformation,
    ) -> std::io::Result<crate::display::modes::neomimic::row::DataRow> {
        let kernel_version = get_option(
            "kernel version",
            &get_option("kernel info", &info.kernel)?.version,
        )?;

        Ok(DataRow::info("Kernel", kernel_version))
    }
}
