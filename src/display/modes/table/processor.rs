use crate::{
    data::table::{Table, TableEntry},
    info,
    util::{colorize::colorize_by_num, percentage},
};

pub fn to_table(
    info: &info::SystemInformation,
    disable_color: bool,
    multicpu: bool,
) -> Option<Table> {
    if !multicpu {
        let cpu_info = info.cpu.clone()?;

        let mut result = Table::new("Processor");

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
                if !disable_color {
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

        return Some(result);
    } else {
        #[cfg(target_os = "linux")]
        {
            let multiple_cpus = info.multicpu.clone();
            if multiple_cpus.is_empty() {
                return None;
            } else {
                let mut result = Table::new("Processor");
                for cpu in multiple_cpus {
                    let cpu_info = cpu.cpuinfo;

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
                            format!("{} Cores / {} Threads", cpu_info.cores, cpu_info.threads)
                                .as_str(),
                        ));
                    }
                    if let Some(temp) = cpu_info.temperature {
                        subtable_entries.push(TableEntry::new(
                            "Temperature",
                            if !disable_color {
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

                    result.add_with_additional(
                        format!("CPU #{}", cpu.physical_id).as_str(),
                        "",
                        subtable_entries,
                    );
                }
                return Some(result);
            }
        }
    }
    #[allow(unreachable_code)]
    None
}
