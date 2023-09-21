#![allow(dead_code, unused_must_use)]
pub mod pci_ids;
mod hardware;
mod software;

mod utils;

use std::collections::BTreeMap;
use std::fmt::Write;

fn get_len(str: &String) -> usize {
    let mut result: usize = 0;
    str.chars().for_each(|_| result += 1);
    result
}

fn get_closest_tb(gb: i64) -> i64 {
    let mut result: i64 = 0;
    let mut delta: i64 = gb;

    for size in [1, 2, 4, 8, 16, 32] {
        let _delta = (gb - (size * 1024)).abs();
        if delta > _delta {
            delta = _delta;
            result = size;
        }
    }

    result
}

fn drive_size_to_string(size: utils::converter::MemorySize) -> String {
    let mut _size: i64 = 0;
    let mut suffix = "";
    if size.gb == 0 {
        suffix = "MiB";
    } else if size.gb < 1024 {
        _size = size.gb;
        suffix = "GiB";
    } else if size.gb > 1024 {
        _size = get_closest_tb(size.gb);
        suffix = "TiB";
    }

    format!("{}{}", _size, suffix)
}

fn get_info() -> BTreeMap<String, Vec<String>> {
    let mut result: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut buf = String::new();

    // System
    let distro_name = software::os::get_name();
    let uptime = software::os::get_uptime();
    let hostname = software::os::get_hostname();
    let username = whoami::username();
    let shell = software::os::get_shell();
    let kernel_version = software::kernel::get_version();
    let init_system = software::init_system::detect();
    let terminal = software::terminal::get_name();

    write!(buf, "Hostname: {hostname}\0");
    write!(buf, "Username: {username}\0");
    write!(buf, "Distro: {distro_name}\0");
    buf.push_str(format!("Kernel: {}\0", kernel_version).as_str());

    if init_system.is_some() {
        let init_system = init_system.unwrap();
        buf.push_str(
            format!(
                "Init system: {}\0┗Services: {}\0",
                init_system.name, init_system.count_services
            )
            .as_str(),
        );
    }
    write!(buf, "Terminal: {terminal}\0");
    write!(buf, "Shell: {shell}\0");
    if uptime.is_some() {
        let mut uptime = uptime.unwrap();
        buf.push_str("Uptime: ");
        if uptime.hours > 24 {
            buf.push_str(format!("{}d", uptime.hours / 24).as_str());
        }
        uptime.hours = uptime.hours % 24;
        buf.push_str(
            format!(
                " {}:{}:{}\0",
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
            .as_str(),
        );
    }

    result.insert(
        "System".to_string(),
        buf.split("\0").map(|s| s.to_string()).collect(),
    );
    buf.clear();

    // Packages
    let pkg_info = software::packages::get_info();
    if !pkg_info.is_empty() {
        for manager in pkg_info {
            buf.push_str(format!("{}: {}\0", manager.manager, manager.count_of_packages).as_str());
        }
        result.insert(
            "Packages".to_string(),
            buf.split("\0").map(|s| s.to_string()).collect(),
        );
        buf.clear();
    }

    // Processor
    let cpu_info = hardware::cpu::get_info();
    buf.push_str(format!("Model: {}\0", cpu_info.model).as_str());
    buf.push_str(format!("Max freq: {}GHz\0", cpu_info.max_freq.ghz).as_str());
    if cpu_info.cores > 0 {
        buf.push_str(format!("Cores: {}\0", cpu_info.cores).as_str());
    }
    buf.push_str(format!("Threads: {}\0", cpu_info.threads).as_str());

    result.insert(
        "Processor".to_string(),
        buf.split("\0").map(|s| s.to_string()).collect(),
    );
    buf.clear();

    // Memory
    let mem_info = hardware::ram::get_info();
    buf.push_str(format!("RAM: {}MiB / {}MiB\0", mem_info.used.mb, mem_info.total.mb).as_str());
    if mem_info.swap_enabled {
        buf.push_str(format!("Swap: {}MiB / {}MiB\0", mem_info.swap_used.mb, mem_info.swap_total.mb).as_str());
    }

    result.insert(
        "Memory".to_string(),
        buf.split("\0").map(|s| s.to_string()).collect(),
    );
    buf.clear();

    // Drives
    let drives = hardware::drive::scan_drives();
    if drives.is_some() {
        let drives = drives.unwrap();
        if !drives.is_empty() {
            for drive in drives {
                // buf.push_str(format!("{}: {}GiB\0", drive.model, drive.size.gb).as_str());
                buf.push_str(
                    format!("{}: {}\0", drive.model, drive_size_to_string(drive.size)).as_str(),
                );
            }
            result.insert(
                "Drives".to_string(),
                buf.split("\0").map(|s| s.to_string()).collect(),
            );
            buf.clear();
        }
    }

    // Graphics
    let gpus = hardware::gpu::get_info();
    if gpus.is_some() {
        let gpus = gpus.unwrap();
        let count_gpus = gpus.len();
        for entry in gpus {
            let gpu_id = entry.0;
            let gpu_name = entry.1;
            buf.push_str(
                format!(
                    "GPU{}: {}\0",
                    if count_gpus > 1 {
                        format!(" #{}", gpu_id)
                    } else {
                        String::from("")
                    },
                    gpu_name
                )
                .as_str(),
            );
        }
    }
    let session_type = software::graphics::get_session_type();
    if session_type.is_some() {
        let session_type = session_type.unwrap();
        write!(buf, "Session type: {session_type}\0");
    }
    let de = software::graphics::detect_de();
    if de.is_some() {
        let de = de.unwrap();
        write!(buf, "Environment: {de}\0");
    }
    let wm = software::graphics::detect_wm();
    if wm.is_some() {
        let wm = wm.unwrap();
        write!(buf, "Window manager: {wm}\0");
    }
    if !buf.is_empty() {
        result.insert(
            "Graphics".to_string(),
            buf.split("\0").map(|s| s.to_string()).collect(),
        );
    }
    buf.clear();

    result
}

fn get_map_max_len(map: BTreeMap<String, Vec<String>>) -> usize {
    let mut result: usize = 0;
    if map.is_empty() {
        return result;
    }

    for key in map.keys() {
        for line in map.get(key).unwrap() {
            let _len = get_len(line);
            if _len > result {
                result = _len;
            }
        }
    }

    result
}

fn get_param_max_len(map: BTreeMap<String, Vec<String>>) -> usize {
    let mut result: usize = 0;
    if map.is_empty() {
        return result;
    }

    for key in map.keys() {
        for line in map.get(key).unwrap() {
            let _len = get_len(&String::from(line.split(": ").next().unwrap()));
            if _len > result {
                result = _len;
            }
        }
    }

    result
}

fn format_info(map: BTreeMap<String, Vec<String>>) -> BTreeMap<String, Vec<String>> {
    let mut result: BTreeMap<String, Vec<String>> = BTreeMap::new();

    let max_param_len = get_param_max_len(map.clone());

    for category in map.keys() {
        let mut buf: Vec<String> = Vec::new();
        map.get(category.as_str())
            .unwrap()
            .iter()
            .for_each(|info_line| {
                if !info_line.is_empty() {
                    let mut line = info_line.split(": ");
                    let line_param = line.next().unwrap();
                    let param_len = get_len(&line_param.to_string());
                    let line_val = line.next().unwrap().trim().to_string();
                    buf.push(format!(
                        "{}:{}{}",
                        line_param,
                        " ".repeat(max_param_len + 2 - param_len),
                        line_val
                    ));
                }
            });
        if !buf.is_empty() {
            result.insert(category.to_string(), buf);
        }
    }

    result
}

fn print_info() {
    let info = format_info(get_info());

    let max_len = get_map_max_len(info.clone());
    for category in info.keys().rev() {
        println!(
            "{}─┤ {} ├{}{}",
            if Some(category) == info.keys().rev().next() {
                "┌"
            } else {
                "├"
            },
            category,
            "─".repeat(max_len - get_len(category) - 3),
            if Some(category) == info.keys().rev().next() {
                "┐"
            } else {
                "┤"
            }
        );
        info.get(category.as_str())
            .unwrap()
            .iter()
            .for_each(|line| println!("│ {}{}│", line, " ".repeat(max_len - line.len() + 1)))
    }
    println!("└{}┘", "─".repeat(max_len + 2))
}

fn main() {
    print_info();
}
