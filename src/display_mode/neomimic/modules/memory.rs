use super::{DataRow, ModuleTrait};

use crate::info::get_option;

pub struct Memory;

impl ModuleTrait for Memory {
    const NAME: &'static str = "memory";

    fn run(info: &crate::info::SystemInformation) -> std::io::Result<usize> {
        let memory = get_option("memory", &info.ram)?;

        let memory_str = format!(
            "{used}MiB / {total}MiB",
            used = memory.used.mb,
            total = memory.total.mb
        );

        Ok(DataRow::info("Memory", &memory_str))
    }
}
