use super::{DisplayModule, DisplaySenderT, TableEntry};

use crate::{
    config::{multicpu, no_colors},
    modules::{Battery, Brightness, Device, Disks, Memory, CPU, GPU},
    util::{colorize::colorize_by_num, percentage},
};

impl DisplayModule<DisplaySenderT> for Battery {
    fn display(batteries: Self::Result, sender: &DisplaySenderT) {
        for battery in batteries {
            sender.send(TableEntry::new_with_additional(
                "Battery",
                &battery.model,
                &[
                    TableEntry::new("Level", &format!("{}%", battery.level)),
                    TableEntry::new("Status", &battery.status.to_str()),
                    TableEntry::new("Technology", &battery.technology.to_string()),
                ],
            ))
        }
    }
}

impl DisplayModule<DisplaySenderT> for Brightness {
    fn display(brightness: Self::Result, sender: &DisplaySenderT) {
        let percentage = percentage(brightness.max as u64, brightness.current as u64) as u16;
        sender.send(TableEntry::new(
            "Brightness",
            &if !no_colors() {
                colorize_by_num(
                    format!("{}%", percentage).as_str(),
                    percentage as u16,
                    40,
                    true,
                )
            } else {
                format!("{}%", percentage)
            },
        ));
    }
}

impl DisplayModule<DisplaySenderT> for CPU {
    fn display(units: Self::Result, sender: &DisplaySenderT) {
        for unit in units {
            let cpu_info = unit.cpuinfo;

            let mut subtable_entries: Vec<TableEntry> = Vec::new();

            subtable_entries.push(TableEntry::new(
                "Model",
                format!("{} {}", cpu_info.vendor, cpu_info.model).as_str(),
            ));
            subtable_entries.push(TableEntry::new(
                "Frequency",
                format!("{:.2}GHz", cpu_info.frequency.ghz).as_str(),
            ));
            if cpu_info.cores > 0 {
                subtable_entries.push(TableEntry::new(
                    "Computing units",
                    format!("{} Cores / {} Threads", cpu_info.cores, cpu_info.threads).as_str(),
                ));
            }
            if let Some(temp) = cpu_info.temperature {
                subtable_entries.push(TableEntry::new(
                    "Temperature",
                    if !no_colors() {
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
                ));
            }

            sender.send(TableEntry::new_with_additional(
                if multicpu() {
                    format!("CPU #{}", unit.physical_id)
                } else {
                    "CPU".to_string()
                }
                .as_str(),
                "",
                &subtable_entries,
            ));
        }
    }
}

impl DisplayModule<DisplaySenderT> for Device {
    fn display(device: Self::Result, sender: &DisplaySenderT) {
        sender.send(TableEntry::new("Device", &device))
    }
}

fn size_to_string(size: &libscu::types::Memory) -> String {
    if size.gb == 0_f64 {
        format!("{:.1}MiB", size.mb as f64)
    } else if size.gb < 1024_f64 {
        format!("{:.1}GiB", size.gb as f64)
    } else {
        // So size.gb is more then 1024GiB (TiB)
        format!("{:.1}TiB", size.gb as f64 / 1024_f64)
    }
}
impl DisplayModule<DisplaySenderT> for Disks {
    fn display(disks: Self::Result, sender: &DisplaySenderT) {
        for disk in disks.iter() {
            sender.send(TableEntry::new(
                &disk.model.clone().unwrap_or("unknown model".to_string()),
                format!("{} [{:?}]", size_to_string(&disk.size), disk.technology).as_str(),
            ))
        }
    }
}

impl DisplayModule<DisplaySenderT> for GPU {
    fn display(gpus: Self::Result, sender: &DisplaySenderT) {
        let mut gpu_sub_info: Vec<TableEntry> = Vec::new();
        for (gpu_id, gpu_info) in gpus.iter().enumerate() {
            if let Some(gpu_temp) = gpu_info.temperature {
                if gpu_temp > 0.0 {
                    gpu_sub_info.push(TableEntry::new(
                        "Temperature",
                        format!("{}°C", gpu_temp).as_str(),
                    ));
                };
            }
            if let Some(gpu_driver) = gpu_info.driver.as_ref() {
                gpu_sub_info.push(TableEntry::new("Driver", &gpu_driver))
            }
            sender.send(TableEntry::new_with_additional(
                format!(
                    "GPU{}",
                    if gpus.len() > 1 {
                        format!(" #{}", gpu_id)
                    } else {
                        "".to_string()
                    }
                )
                .as_str(),
                format!("{} {}", gpu_info.vendor, gpu_info.model).as_str(),
                &gpu_sub_info,
            ));
            gpu_sub_info.clear();
        }
    }
}

impl DisplayModule<DisplaySenderT> for Memory {
    fn display(ram_info: Self::Result, sender: &DisplaySenderT) {
        let ram_usage_percents = percentage(ram_info.total.mb as u64, ram_info.used.mb as u64);
        sender.send(TableEntry::new(
            "RAM",
            &format!(
                "{}MiB / {}MiB [{}]",
                ram_info.used.mb,
                ram_info.total.mb,
                if !no_colors() {
                    colorize_by_num(
                        format!("{:.1}%", ram_usage_percents).as_str(),
                        ram_usage_percents as u16,
                        100,
                        false,
                    )
                } else {
                    format!("{:.1}%", ram_usage_percents)
                }
            ),
        ));
        if let Some(swap_info) = ram_info.swap {
            let swap_usage_percents =
                percentage(swap_info.total.mb as u64, swap_info.used.mb as u64);
            sender.send(TableEntry::new(
                "Swap",
                &format!(
                    "{}MiB / {}MiB [{}]",
                    swap_info.used.mb,
                    swap_info.total.mb,
                    if !no_colors() {
                        colorize_by_num(
                            format!("{:.1}%", swap_usage_percents).as_str(),
                            swap_usage_percents as u16,
                            100,
                            false,
                        )
                    } else {
                        format!("{:.1}%", swap_usage_percents)
                    }
                ),
            ));
        };
    }
}
