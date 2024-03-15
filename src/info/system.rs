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
    if let Some(osrelease) = os::fetch_name() {
        let pretty_name = osrelease.pretty_name;
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
        result.add_with_additional(
            "Init system",
            &init_system.name,
            if let Some(number_of_services) = init_system.number_of_services {
                Vec::from([TableEntry::new("Services", &number_of_services.to_string())])
            } else {
                Vec::new()
            },
        );
    }
    // result.add("Terminal", &terminal::fetch_info(force_version));
    let terminal_info = terminal::fetch_info(force_version);

    result.add(
        "Terminal",
        &format!(
            "{}{}",
            terminal_info.name,
            if let Some(version) = terminal_info.version {
                format!(" v{version}")
            } else {
                "".to_string()
            }
        ),
    );

    if let Some(shell) = shell::fetch_info(force_version) {
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
