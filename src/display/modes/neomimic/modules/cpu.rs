use crate::{display::modes::neomimic::row::DataRow, info::get_option};

use super::ModuleTrait;

pub struct CPU;

impl ModuleTrait for CPU {
    const NAME: &'static str = "cpu";

    fn get(
        info: &crate::info::SystemInformation,
    ) -> std::io::Result<crate::display::modes::neomimic::row::DataRow> {
        let cpu = get_option("cpu", &info.cpu)?;

        let cpu_str = format!(
            "{vendor} {model} ({units}) @ {frequency:.3}GHz",
            vendor = cpu.vendor.to_string(),
            model = cpu.model,
            units = cpu.cores.max(cpu.threads),
            frequency = cpu.frequency.ghz
        );

        Ok(DataRow::info("CPU", cpu_str))
    }
}
