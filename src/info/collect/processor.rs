use crate::info::r#struct::Processor;

use libscu::hardware::cpu;

pub fn collect() -> Option<Processor> {
    let mut result = Processor::default();

    if let Ok(cpu_info) = cpu::fetch_info() {
        result.model = format!("{} {}", cpu_info.vendor, cpu_info.model);
        result.frequency = cpu_info.frequency;
        (result.cores, result.threads) = (cpu_info.cores, cpu_info.threads);
        result.temperature = cpu_info.temperature;

        Some(result)
    } else {
        None
    }
}
