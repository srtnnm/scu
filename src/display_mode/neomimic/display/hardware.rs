use super::{super::row::DataRow, Display, RowSenderT};

use crate::{
    modules::{Battery, Brightness, Device, Disks, Memory, CPU, GPU},
    util::percentage,
};

impl Display for Battery {
    fn display(batteries: Self::Result, sender: &RowSenderT) -> std::io::Result<()> {
        let len = batteries
            .iter()
            .map(|battery| {
                DataRow::info(
                    &format!("Battery ({})", battery.model),
                    &format!("{}% [{}]", battery.level, battery.status.to_str()),
                    sender,
                )
            })
            .max()
            .unwrap_or_default();

        Ok(len)
    }
}

impl Display for Brightness {
    fn display(brightness: Self::Result, sender: &RowSenderT) -> std::io::Result<()> {
        let percentage = percentage(brightness.max as u64, brightness.current as u64);
        Ok(DataRow::info(
            "Brightness",
            &format!("{percentage}%"),
            sender,
        ))
    }
}

impl Display for CPU {
    fn display(cpus: Self::Result, sender: &RowSenderT) -> std::io::Result<()> {
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
                    sender,
                )
            })
            .max()
            .unwrap_or_default();

        Ok(len)
    }
}

impl Display for Device {
    fn display(device: Self::Result, sender: &RowSenderT) -> std::io::Result<()> {
        Ok(DataRow::info("Device", &device, sender))
    }
}

impl Display for Disks {
    fn display(disks: Self::Result, sender: &RowSenderT) -> std::io::Result<()> {
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
                    sender,
                )
            })
            .max()
            .unwrap_or_default();
        Ok(len)
    }
}

impl Display for GPU {
    fn display(gpus: Self::Result, sender: &RowSenderT) -> std::io::Result<()> {
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
                    sender,
                )
            })
            .max()
            .unwrap_or_default();

        Ok(len)
    }
}

impl Display for Memory {
    fn display(memory: Self::Result, sender: &RowSenderT) -> std::io::Result<()> {
        let len = [
            Some(DataRow::info(
                "Memory",
                &format!(
                    "{used}MiB / {total}MiB",
                    used = memory.used.mb,
                    total = memory.total.mb
                ),
                sender,
            )),
            memory.swap.map(|swap_info| {
                DataRow::info(
                    "Swap",
                    &format!(
                        "{used}MiB / {total}MiB",
                        used = swap_info.used.mb,
                        total = swap_info.total.mb
                    ),
                    sender,
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
