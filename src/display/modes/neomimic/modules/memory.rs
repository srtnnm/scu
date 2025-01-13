use crate::{display::modes::neomimic::row::DataRow, info::get_option};

use super::ModuleTrait;

pub struct Memory;

impl ModuleTrait for Memory {
    const NAME: &'static str = "memory";

    fn get(
        info: &crate::info::SystemInformation,
    ) -> std::io::Result<crate::display::modes::neomimic::row::DataRow> {
        let memory = get_option("memory", &info.ram)?;

        let memory_str = format!(
            "{used}MiB / {total}MiB",
            used = memory.used.mb,
            total = memory.total.mb
        );

        Ok(DataRow::info("Memory", memory_str))
    }
}
