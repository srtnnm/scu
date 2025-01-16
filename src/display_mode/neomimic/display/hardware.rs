/*
disks
*/

use super::{super::row::DataRow, Display};

use crate::modules::{Battery, Brightness, Device, Memory, CPU, GPU};

impl Display for Battery {
    fn display(batteries: Self::Result) -> std::io::Result<usize> {
        let len = batteries
            .iter()
            .map(|battery| {
                DataRow::info(
                    &format!("Battery ({})", battery.model),
                    &format!("{}% [{}]", battery.level, battery.status.to_str()),
                )
            })
            .max()
            .unwrap_or_default();

        Ok(len)
    }
}

impl Display for Brightness {
    fn display(brightness: Self::Result) -> std::io::Result<usize> {
        Ok(DataRow::info(
            "Brightness",
            &format!("{}%", brightness.current),
        ))
    }
}

impl Display for CPU {
    fn display(cpu: Self::Result) -> std::io::Result<usize> {
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

impl Display for Device {
    fn display(device: Self::Result) -> std::io::Result<usize> {
        Ok(DataRow::info("Device", &device))
    }
}

// TODO: disks

impl Display for GPU {
    fn display(gpus: Self::Result) -> std::io::Result<usize> {
        let len = gpus
            .iter()
            .map(|gpu| {
                DataRow::info(
                    "GPU",
                    &format!(
                        "{vendor} {model}",
                        vendor = gpu.vendor.to_string(),
                        model = gpu.model
                    ),
                )
            })
            .max()
            .unwrap_or_default();

        Ok(len)
    }
}

impl Display for Memory {
    fn display(memory: Self::Result) -> std::io::Result<usize> {
        let memory_str = format!(
            "{used}MiB / {total}MiB",
            used = memory.used.mb,
            total = memory.total.mb
        );

        Ok(DataRow::info("Memory", &memory_str))
    }
}
