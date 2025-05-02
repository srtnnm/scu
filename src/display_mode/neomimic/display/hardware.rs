use super::{super::row::DataRow, RowSenderT};

use crate::{
    modules::{Battery, Brightness, Device, Disks, DisplayModule, Memory, CPU, GPU},
    util::percentage,
};

impl DisplayModule<RowSenderT> for Battery {
    fn display(batteries: Self::Result, sender: &RowSenderT) {
        for battery in batteries {
            DataRow::info(
                &format!("Battery ({})", battery.model),
                &format!("{}% [{}]", battery.level, battery.status.to_str()),
                sender,
            )
        }
    }
}

impl DisplayModule<RowSenderT> for Brightness {
    fn display(brightness: Self::Result, sender: &RowSenderT) {
        let percentage = percentage(brightness.max as u64, brightness.current as u64);
        DataRow::info("Brightness", &format!("{percentage}%"), sender);
    }
}

impl DisplayModule<RowSenderT> for CPU {
    fn display(cpus: Self::Result, sender: &RowSenderT) {
        for unit in cpus {
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
        }
    }
}

impl DisplayModule<RowSenderT> for Device {
    fn display(device: Self::Result, sender: &RowSenderT) {
        DataRow::info("Device", &device, sender)
    }
}

impl DisplayModule<RowSenderT> for Disks {
    fn display(disks: Self::Result, sender: &RowSenderT) {
        for disk in disks {
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
        }
    }
}

impl DisplayModule<RowSenderT> for GPU {
    fn display(gpus: Self::Result, sender: &RowSenderT) {
        for gpu in gpus {
            DataRow::info(
                "GPU",
                &format!(
                    "{vendor} {model}",
                    vendor = gpu.vendor.to_string(),
                    model = gpu.model
                ),
                sender,
            )
        }
    }
}

impl DisplayModule<RowSenderT> for Memory {
    fn display(memory: Self::Result, sender: &RowSenderT) {
        DataRow::info(
            "Memory",
            &format!(
                "{used}MiB / {total}MiB",
                used = memory.used.mb,
                total = memory.total.mb
            ),
            sender,
        );

        if let Some(swap) = memory.swap {
            DataRow::info(
                "Swap",
                &format!(
                    "{used}MiB / {total}MiB",
                    used = swap.used.mb,
                    total = swap.total.mb
                ),
                sender,
            )
        }
    }
}
