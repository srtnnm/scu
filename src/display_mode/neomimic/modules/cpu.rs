use super::{DataRow, ModuleTrait};

use crate::modules::get_option;

pub struct CPU;

impl ModuleTrait for CPU {
    const NAME: &'static str = "cpu";

    fn run(info: &crate::modules::SystemInformation) -> std::io::Result<usize> {
        let cpu = get_option("cpu", &info.cpu)?;

        let cpu_str = format!(
            "{vendor} {model} ({units}) @ {frequency:.3}GHz",
            vendor = cpu.vendor.to_string(),
            model = cpu.model,
            units = cpu.cores.max(cpu.threads),
            frequency = cpu.frequency.ghz
        );

        Ok(DataRow::info("CPU", &cpu_str))
    }
}
