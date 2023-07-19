#![allow(dead_code, unused_imports, unused_must_use)]

mod hardware;
mod software;

mod utils;

use std::collections::BTreeMap;

fn get_info() -> BTreeMap<String, Vec<String>> {
    let mut result: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut buf: Vec<String> = Vec::new();

    let distro_name = software::os::get_name();
    let uptime = software::os::get_uptime();
    let hostname = software::os::get_hostname();
    let shell = software::os::get_shell();
    let kernel_info = software::kernel::get_info();
    let init_system = software::init_system::detect();
    let terminal = software::terminal::get_name();

    buf.push(format!("Hostname:   {}", hostname));
    buf.push(format!("Username:   {}", whoami::username()));
    buf.push(format!("Distro:     {}", distro_name));
    buf.push(format!("Kernel:     {}", kernel_info.full_version));
    buf.push(format!("Kernel version: {}", kernel_info.version));
    buf.push(format!("Init system: {}", init_system));
    buf.push(format!("Terminal:   {}", terminal));
    buf.push(format!("Shell:      {}", shell));
    buf.push(format!(
        "Uptime:     {}H {}M {}S",
        uptime.hours, uptime.minutes, uptime.seconds
    ));

    result.insert("System".to_string(), buf.clone());
    buf.clear();

    let pkg_info = software::packages::get_info();
    if !pkg_info.is_empty() {
        for manager in pkg_info {
            buf.push(format!(
                "({}): {}",
                manager.manager, manager.count_of_packages
            ));
        }
    }
    result.insert("Packages".to_string(), buf.clone());
    buf.clear();

    let cpu_info = hardware::cpu::get_info();
    buf.push(format!("Vendor:   {}", cpu_info.vendor));
    buf.push(format!("Model:    {}", cpu_info.model));
    buf.push(format!("Max freq: {}GHz", cpu_info.max_freq.ghz));
    buf.push(format!("Cores:    {}", cpu_info.cores));
    buf.push(format!("Threads:  {}", cpu_info.threads));

    result.insert("CPU".to_string(), buf.clone());
    buf.clear();

    let mem_info = hardware::ram::get_info();
    buf.push(format!("Total:      {}MiB", mem_info.total.mb));
    buf.push(format!("Used:       {}MiB", mem_info.used.mb));
    if mem_info.swap_enabled {
        buf.push(format!("Swap total: {}MiB", mem_info.swap_total.mb));
        buf.push(format!("Swap used:  {}MiB", mem_info.swap_used.mb));
    }

    result.insert("Memory".to_string(), buf.clone());
    buf.clear();

    let drives = hardware::drive::scan_drives();
    if !drives.is_empty() {
        for drive in drives {
            buf.push(format!("{}: {}MiB", drive.model, drive.size.mb));
        }
        result.insert("Drives".to_string(), buf.clone());
        buf.clear()
    }

    result
}

fn get_max_len(map: BTreeMap<String, Vec<String>>) -> usize {
    let mut result: usize = 0;
    if map.is_empty() {
        return result;
    }

    for key in map.keys() {
        for line in map.get(key).unwrap() {
            if line.len() > result {
                result = line.len();
            }
        }
    }

    result
}

fn print_info() {
    let info = get_info();
    let mut keys: Vec<String> = info.clone().keys().map(|s| s.to_string()).collect();
    keys.reverse();

    let max_len = get_max_len(info.clone());
    for category in &keys {
        println!(
            "{}-[{}]{}-{}",
            if Some(category) == keys.first() {
                "/"
            } else {
                "|"
            },
            category,
            "-".repeat(max_len - category.len() - 2),
            if Some(category) == keys.first() {
                "\\"
            } else {
                "|"
            }
        );
        for info_line in info.get(category.as_str()).unwrap() {
            println!("| {}{} |", info_line, " ".repeat(max_len - info_line.len()));
        }
        if Some(category) != keys.last() {
            println!("| {} |", " ".repeat(max_len));
        }
    }
    println!("\\_{}_/", "_".repeat(max_len))
}

fn main() {
    print_info();
}
