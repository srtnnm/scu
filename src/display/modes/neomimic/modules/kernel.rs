use super::{DataRow, ModuleTrait};

use crate::info::get_option;

pub struct Kernel;

impl ModuleTrait for Kernel {
    const NAME: &'static str = "kernel";

    fn run(info: &crate::info::SystemInformation) -> std::io::Result<usize> {
        let kernel_version = get_option(
            "kernel version",
            &get_option("kernel info", &info.kernel)?.version,
        )?;

        Ok(DataRow::info("Kernel", &kernel_version))
    }
}
