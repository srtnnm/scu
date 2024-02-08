use crate::data::table::*;
use crate::utils::{colorize::colorize_by_num, percentage};

use libscu::hardware::cpu;

pub fn collect(simplify: bool) -> Table {
    let mut result = Table::new("Processor");

    let cpu_info = cpu::fetch_info();
    result.add(
        "Model",
        format!("{} {}", cpu_info.vendor, cpu_info.model).as_str(),
    );
    result.add(
        "Frequency",
        format!("{:.2}GHz", cpu_info.frequency.ghz).as_str(),
    );
    if cpu_info.cores > 0 {
        result.add(
            "Computing units",
            format!("{} Cores / {} Threads", cpu_info.cores, cpu_info.threads).as_str(),
        );
    }
    if let Some(temp) = cpu_info.temperature {
        result.add(
            "Temperature",
            if !simplify {
                colorize_by_num(
                    format!("{:.1}°C", temp).as_str(),
                    percentage(90, temp as u64) as u16,
                    100,
                    false,
                )
            } else {
                format!("{:.1}°C", temp)
            }
            .as_str(),
        );
    }

    result
}
