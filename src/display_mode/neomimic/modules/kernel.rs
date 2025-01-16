use super::{DataRow, ModuleTrait};

use crate::modules::get_option;

pub struct Kernel;

impl ModuleTrait for Kernel {
    const NAME: &'static str = "kernel";

    fn run(info: &crate::modules::SystemInformation) -> std::io::Result<usize> {
        let kernel_version = get_option("kernel version", &info.kernel)?.version;

        Ok(DataRow::info("Kernel", &kernel_version))
    }
}
