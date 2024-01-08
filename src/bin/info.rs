use std::collections::BTreeMap;

use scu::{utils::converter, *};

use crate::config::Config;
use crate::data::{ascii_art, distro_colors, table::*};
use crate::utils;
use crate::utils::{colorize::*, len};

fn drive_size_to_string(size: converter::MemorySize) -> String {
    let mut _size: f64 = 0_f64;
    let mut suffix = "";
    if size.gb == 0 {
        _size = size.mb as f64;
        suffix = "MiB";
    } else if size.gb < 1024 {
        _size = size.gb as f64;
        suffix = "GiB";
    } else if size.gb > 1024 {
        _size = size.gb as f64 / 1024_f64;
        suffix = "TiB";
    }

    format!("{:.1}{}", _size, suffix)
}

fn collect_info(cfg: Vec<String>, simplify_output: bool, force_version: bool) -> BTreeMap<String, Table> {
    let mut result: BTreeMap<String, Table> = BTreeMap::new();
    let mut buf = Table::new();

    // System
    if cfg.contains(&"system".to_string()) {
        let device_name = device::get_device_model();
        let distro_name = os::get_name().pretty_name;
        let uptime = os::get_uptime();
        let hostname = os::get_hostname();
        let username = whoami::username().unwrap();
        let shell = os::get_shell();
        let kernel_version = kernel::get_version();
        let init_system = init_system::detect();
        let terminal = terminal::get_name();

        buf.add("Hostname", &hostname);
        buf.add("Username", &username);
        buf.add("Distro", &distro_name);
        if let Some(device_name) = device_name {
            buf.add("Device", &device_name);
        }
        buf.add("Kernel", &kernel_version);

        if let Some(init_system) = init_system {
            buf.add_with_additional(
                "Init system",
                &init_system.name,
                Vec::from([TableEntry::new(
                    "Services",
                    &init_system.count_services.to_string(),
                )]),
            );
        }
        buf.add("Terminal", &terminal);
        buf.add("Shell", &shell);
        if let Some(mut uptime) = uptime {
            let mut uptime_str = String::new();
            if uptime.hours > 24 {
                uptime_str += format!("{}d", uptime.hours / 24).as_str();
            }
            uptime.hours %= 24;
            uptime_str += format!(
                " {}:{}:{}",
                format!(
                    "{}{}",
                    if uptime.hours < 10 { "0" } else { "" },
                    uptime.hours
                ),
                format!(
                    "{}{}",
                    if uptime.minutes < 10 { "0" } else { "" },
                    uptime.minutes
                ),
                format!(
                    "{}{}",
                    if uptime.seconds < 10 { "0" } else { "" },
                    uptime.seconds
                )
            )
            .as_str();
            buf.add("Uptime", uptime_str.trim());
        }

        buf.set_name("System");
        result.insert(buf.title.to_ascii_lowercase(), buf.clone());
        buf.clear();
    }

    // Packages
    if cfg.contains(&"packages".to_string()) {
        let pkg_info = packages::get_info();
        if !pkg_info.is_empty() {
            for manager in pkg_info {
                buf.add(manager.manager, &manager.count_of_packages.to_string());
            }
            buf.set_name("Packages");
            result.insert(buf.title.to_ascii_lowercase(), buf.clone());
            buf.clear();
        }
    }

    // Processor
    if cfg.contains(&"processor".to_string()) {
        let cpu_info = cpu::get_info();
        buf.add(
            "Model",
            format!("{} {}", cpu_info.vendor, cpu_info.model).as_str(),
        );
        buf.add(
            "Frequency",
            format!("{:.2}GHz", cpu_info.frequency.ghz).as_str(),
        );
        if cpu_info.cores > 0 {
            buf.add(
                "Computing units",
                format!("{} Cores / {} Threads", cpu_info.cores, cpu_info.threads).as_str(),
            );
        }
        if cpu_info.temperature > 0.0 {
            buf.add(
                "Temperature",
                if !simplify_output {
                    colorize_by_num(
                        format!("{:.1}°C", cpu_info.temperature).as_str(),
                        utils::percentage(90, cpu_info.temperature as u64) as u16,
                        false,
                    )
                } else {
                    format!("{:.1}°C", cpu_info.temperature)
                }
                .as_str(),
            );
        }

        buf.set_name("Processor");
        result.insert(buf.title.to_ascii_lowercase(), buf.clone());
        buf.clear();
    }

    // Memory
    if cfg.contains(&"memory".to_string()) {
        let mem_info = ram::get_info();
        let (ram_usage_percents, swap_usage_percents) = (
            utils::percentage(mem_info.total.mb as u64, mem_info.used.mb as u64),
            utils::percentage(mem_info.swap_total.mb as u64, mem_info.swap_used.mb as u64),
        );
        buf.add(
            "RAM",
            format!(
                "{}MiB / {}MiB [{}]",
                mem_info.used.mb,
                mem_info.total.mb,
                if !simplify_output {
                    colorize_by_num(
                        format!("{:.1}%", ram_usage_percents).as_str(),
                        ram_usage_percents as u16,
                        false,
                    )
                } else {
                    format!("{:.1}%", ram_usage_percents)
                }
            )
            .as_str(),
        );
        if mem_info.swap_enabled {
            buf.add(
                "Swap",
                format!(
                    "{}MiB / {}MiB [{}]",
                    mem_info.swap_used.mb,
                    mem_info.swap_total.mb,
                    if !simplify_output {
                        colorize_by_num(
                            format!("{:.1}%", swap_usage_percents).as_str(),
                            swap_usage_percents as u16,
                            false,
                        )
                    } else {
                        format!("{:.1}%", swap_usage_percents)
                    }
                )
                .as_str(),
            );
        }

        buf.set_name("Memory");
        result.insert(buf.title.to_ascii_lowercase(), buf.clone());
        buf.clear();
    }

    // Battery
    if cfg.contains(&"battery".to_string()) {
        let battery = battery::get_battery_info();
        if let Some(battery) = battery {
            buf.add("Model", &battery.model);
            buf.add("Technology", &battery.technology);
            buf.add("Capacity", format!("{}%", battery.capacity).as_str());
            buf.add("Status", &battery.status);

            buf.set_name("Battery");
            result.insert(buf.title.to_ascii_lowercase(), buf.clone());
            buf.clear();
        }
    }

    // Drives
    if cfg.contains(&"drives".to_string()) {
        let drives = drive::scan_drives();
        if let Some(drives) = drives {
            if !drives.is_empty() {
                for drive in drives {
                    buf.add(
                        format!("[{:?}] {}", drive.technology, drive.model).as_str(),
                        drive_size_to_string(drive.size).as_str(),
                    );
                }
                buf.set_name("Drives");
                result.insert(buf.title.to_ascii_lowercase(), buf.clone());
                buf.clear();
            }
        }
    }

    // Graphics
    if cfg.contains(&"graphics".to_string()) {
        let gpus = gpu::get_info();
        if let Some(gpus) = gpus {
            let count_gpus = gpus.len();
            for entry in gpus.iter().enumerate() {
                let (gpu_id, gpu_info) = (entry.0, entry.1);
                let mut sub_info: Vec<TableEntry> = Vec::new();
                if gpu_info.driver != "Unknown" {
                    sub_info.push(TableEntry::new("Driver", &gpu_info.driver));
                }
                if gpu_info.temperature > 0.0 {
                    sub_info.push(TableEntry::new(
                        "Temperature",
                        format!("{}°C", gpu_info.temperature).as_str(),
                    ));
                }
                buf.add_with_additional(
                    format!(
                        "GPU{}",
                        if count_gpus > 1 {
                            format!(" #{}", gpu_id)
                        } else {
                            String::from("")
                        }
                    )
                    .as_str(),
                    format!("{} {}", gpu_info.vendor, gpu_info.model,).as_str(),
                    sub_info,
                );
            }
        }
        let session_type = graphics::get_session_type();
        if let Some(session_type) = session_type {
            buf.add("Session type", &session_type);
        }
        let de = graphics::detect_de();
        if let Some(de) = de {
            buf.add("Environment", &de);
        }
        let wm = graphics::detect_wm(force_version);
        if let Some(wm) = wm {
            buf.add("Window manager", &wm);
        }

        buf.set_name("Graphics");
        result.insert(buf.title.to_ascii_lowercase(), buf.clone());
        buf.clear();
    }

    result
}

fn formatted_info(cfg: Config, simplify_output: bool, force_version: bool) -> Vec<(String, Vec<String>)> {
    let tables = collect_info(cfg.order.clone(), simplify_output, force_version);
    let mut result: Vec<(String, Vec<String>)> = Vec::new();

    let max_param_len = len::param_max_len(tables.clone().into_iter().map(|elm| elm.1).collect());

    for table in cfg.order {
        if !tables.contains_key(&table) {
            continue;
        }
        let table = tables.get(&table).unwrap().clone();
        let mut table_buf = Vec::<String>::new();
        if table.entries.is_empty() {
            continue
        }
        for entry in table.entries {
            let param_len = len::len(&entry.name);
            let line_val = utils::uniqalize(entry.value);
            if &line_val != "Unknown" && &line_val != "0" {
                table_buf.push(format!(
                    "{}:{}{}",
                    &entry.name,
                    " ".repeat(if !simplify_output {
                        max_param_len - param_len + 2
                    } else {
                        1
                    }),
                    line_val
                ));
            }
            for additional in entry.additional.clone().into_iter().enumerate() {
                let (i, additional) = additional;
                let add_line_val = utils::uniqalize(additional.value);
                if &add_line_val != "Unknown" && &add_line_val != "0" {
                    table_buf.push(format!(
                        "{}{}:{}{}",
                        if simplify_output {
                            "  "
                        } else {
                            if i == entry.additional.len() - 1 {
                                "┗"
                            } else {
                                "┣"
                            }
                        },
                        &additional.name,
                        " ".repeat(if !simplify_output {
                            max_param_len - len::len(&additional.name) + 1
                        } else {
                            1
                        }),
                        add_line_val
                    ));
                }
            }
        }
        result.push((table.title, table_buf));
    }

    result
}

fn add_logo(output_text: &mut Vec<String>, max_len: usize, override_whale: bool) {
    let mut distro_name = os::get_name().name;
    if distro_name.is_empty() {
        distro_name = "Linux".to_string();
    } else {
        for trash in ["/", "GNU", "Linux"] {
            distro_name = distro_name.replace(trash, "");
        }
    }
    let logo_lines: Vec<String> = if override_whale {
        distro_name = "Whale".to_string();
        ascii_art::WHALE
            .split("\0")
            .map(|l| l.to_string())
            .collect()
    } else {
        ascii_art::generate(&distro_name)
    };
    let logo_max_len = len::max_len(logo_lines.clone());
    if terminal::get_size().unwrap().width > max_len + logo_max_len + 5_usize
        && output_text.len() >= logo_lines.len() + 3
    {
        let _logo_box_height = logo_lines.len() + 2;
        for line in 0..output_text.len() {
            if line == 0 || line == _logo_box_height - 1 {
                let info_box_frame_element = output_text[line].pop().unwrap_or('\0');
                output_text[line].push_str(match info_box_frame_element {
                    '┐' => "┬",
                    '│' => "├",
                    '┤' => "┼",
                    '┘' => "┴",
                    _ => "|",
                });
                output_text[line].push_str(&format!(
                    "{}{}",
                    "─".repeat(logo_max_len + 2),
                    match line {
                        0 => "┐",
                        _logo_box_height => "┘",
                    }
                ));
            }
            if line > 0 && line - 1 < logo_lines.len() {
                let color = distro_colors::get_color(&distro_name);
                let colorized_line = match color {
                    Some(color) => {
                        colorize_background(&logo_lines[line - 1], color.r, color.g, color.b)
                    }
                    _ => logo_lines[line - 1].to_string(),
                };
                output_text[line].push_str(&format!(
                    " {}{} │",
                    colorized_line,
                    " ".repeat(logo_max_len - len::len(&logo_lines[line - 1].to_string()))
                ));
            }
        }
    }
}

pub fn print_info(cfg: Config, override_whale: bool, simplify_output: bool, force_version: bool) {
    let info = formatted_info(cfg, simplify_output, force_version);

    if !simplify_output {
        let max_len = info
            .clone()
            .into_iter()
            .map(|e| len::max_len(e.1))
            .max()
            .unwrap();
        let mut to_display: Vec<String> = Vec::new();
        for table in info.clone().into_iter().enumerate() {
            to_display.push(format!(
                "{}─┤ {} ├{}{}",
                if table.0 == 0 { "┌" } else { "├" },
                table.1 .0,
                "─".repeat(max_len - len::len(&table.1 .0) - 4),
                if table.0 == 0 { "┐" } else { "┤" }
            ));
            for entry in table.1 .1 {
                to_display.push(format!(
                    "│ {}{}│",
                    entry,
                    " ".repeat(max_len - len::len(entry.as_str()))
                ));
            }
        }
        to_display.push(format!("└{}┘", "─".repeat(max_len + 1)));

        add_logo(&mut to_display, max_len, override_whale);

        // Display info
        to_display.into_iter().for_each(|l| println!("{l}"));
    } else {
        for table in info {
            println!("- {}", table.0);
            for entry in table.1 {
                println!("{}", entry);
            }
        }
    }
}
