use crate::{
    data::{distro_colors, table::*},
    utils::colorize,
};

use libscu::{
    hardware::device,
    software::{hostname, init_system, kernel, os, shell, terminal, uptime, whoami},
};

pub fn collect(simplify: bool, force_version: bool) -> Table {
    let mut result = Table::new("System");
    if let Some(hostname) = hostname::fetch() {
        result.add("Hostname", &hostname)
    }
    if let Some(username) = whoami::fetch_name() {
        result.add("Username", &username)
    }
    {
        let pretty_name = os::fetch_name().pretty_name;
        result.add(
            "Distro",
            (if simplify {
                pretty_name
            } else {
                match distro_colors::get_color(&pretty_name) {
                    Some(clr) => colorize::colorize_background(&pretty_name, clr.r, clr.g, clr.b),
                    None => pretty_name,
                }
            })
            .as_str(),
        );
    }
    if let Some(device_name) = device::fetch_model() {
        result.add("Device", &device_name);
    }
    result.add("Kernel", &kernel::fetch_version());

    if let Some(init_system) = init_system::fetch_info() {
        if init_system.name.is_some() {
            result.add_with_additional(
                "Init system",
                &init_system.name.unwrap_or("".to_string()),
                if init_system.count_services.is_some() {
                    Vec::from([TableEntry::new(
                        "Services",
                        &init_system.count_services.unwrap_or(0).to_string(),
                    )])
                } else {
                    Vec::new()
                },
            );
        }
    }
    result.add("Terminal", &terminal::fetch_name(force_version));
    if let Some(shell) = shell::fetch_name(force_version) {
        result.add(
            "Shell",
            format!(
                "{}{}",
                shell.name,
                if let Some(shell_version) = shell.version {
                    format!(" v{}", shell_version)
                } else {
                    "".to_string()
                }
            )
            .as_str(),
        );
    }
    if let Some(mut uptime) = uptime::fetch() {
        let mut uptime_str = String::new();
        if uptime.hours >= 24 {
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
        result.add("Uptime", uptime_str.trim());
    };

    result
}