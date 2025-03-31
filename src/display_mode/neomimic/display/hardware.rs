use super::{super::row::DataRow, Display};

use crate::{
    modules::{Battery, Brightness, Device, Disks, Memory, CPU, GPU},
    util::percentage,
};

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
        let percentage = percentage(brightness.max as u64, brightness.current as u64);
        Ok(DataRow::info("Brightness", &format!("{percentage}%")))
    }
}

impl Display for CPU {
    fn display(cpus: Self::Result) -> std::io::Result<usize> {
        let len = cpus
            .iter()
            .map(|unit| {
                DataRow::info(
                    "CPU",
                    &format!(
                        "{vendor} {model} ({units}) @ {frequency:.3}GHz",
                        vendor = unit.cpuinfo.vendor.to_string(),
                        model = unit.cpuinfo.model,
                        units = unit.cpuinfo.cores.max(unit.cpuinfo.threads),
                        frequency = unit.cpuinfo.frequency.ghz
                    ),
                )
            })
            .max()
            .unwrap_or_default();

        Ok(len)
    }
}

impl Display for Device {
    fn display(device: Self::Result) -> std::io::Result<usize> {
        Ok(DataRow::info("Device", &device))
    }
}

impl Display for Disks {
    fn display(disks: Self::Result) -> std::io::Result<usize> {
        let len = disks
            .iter()
            .map(|disk| {
                DataRow::info(
                    &format!(
                        "Disk ({})",
                        if let Some(ref model) = disk.model {
                            model.clone()
                        } else {
                            disk.dev_path.to_string_lossy().to_string()
                        }
                    ),
                    &format!("{:.2}GiB", disk.size.gb),
                )
            })
            .max()
            .unwrap_or_default();
        Ok(len)
    }
}

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
        let len = [
            Some(DataRow::info(
                "Memory",
                &format!(
                    "{used}MiB / {total}MiB",
                    used = memory.used.mb,
                    total = memory.total.mb
                ),
            )),
            memory.swap.map(|swap_info| {
                DataRow::info(
                    "Swap",
                    &format!(
                        "{used}MiB / {total}MiB",
                        used = swap_info.used.mb,
                        total = swap_info.total.mb
                    ),
                )
            }),
        ]
        .into_iter()
        .flatten()
        .max()
        .unwrap_or_default();
        Ok(len)
    }
}
