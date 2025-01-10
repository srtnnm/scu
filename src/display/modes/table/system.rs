use crate::{
    data::{
        distro_colors,
        table::{Table, TableEntry},
    },
    info,
    util::colorize::colorize_background,
};

pub fn to_table(info: &info::SystemInformation, disable_color: bool) -> Option<Table> {
    let mut result = Table::new("System");

    if let Some(hostname) = &info.hostname {
        result.add("Hostname", hostname)
    }

    if let Some(user) = &info.username {
        result.add("Username", user)
    }

    if let Some(osrelease) = info.os_release.clone() {
        let name = if osrelease.pretty_name.is_empty() {
            osrelease.name
        } else {
            osrelease.pretty_name
        };
        result.add(
            "Distro",
            (if disable_color {
                name
            } else {
                match distro_colors::get_color(&name) {
                    Some(clr) => colorize_background(&name, clr.r, clr.g, clr.b),
                    None => name,
                }
            })
            .as_str(),
        );
    }

    if let Some(device_name) = info.device_name.clone() {
        result.add("Device", &device_name);
    }

    if let Some(kernel_version) = info
        .kernel
        .clone()
        .and_then(|kernel_info| kernel_info.version)
    {
        result.add("Kernel", &kernel_version);
    }

    if let Some(init_system) = info.init_system.as_ref() {
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

    if let Some(terminal_info) = info.terminal.as_ref() {
        result.add(
            "Terminal",
            &format!(
                "{}{}",
                terminal_info.name,
                if let Some(version) = terminal_info.version.as_deref() {
                    format!(" v{version}")
                } else {
                    "".to_string()
                }
            ),
        );
    }

    if let Some(shell) = info.shell.as_ref() {
        result.add(
            "Shell",
            format!(
                "{}{}",
                shell.name,
                if let Some(shell_version) = shell.version.as_deref() {
                    format!(" v{}", shell_version)
                } else {
                    "".to_string()
                }
            )
            .as_str(),
        );
    }
    if let Some(uptime) = info.uptime.as_ref() {
        let mut uptime_str = String::new();
        if uptime.days > 0 {
            uptime_str += format!("{}d", uptime.days).as_str();
        }
        uptime_str += format!(
            " {}:{}:{}",
            format!(
                "{}{}",
                if (uptime.hours % 24) < 10 { "0" } else { "" },
                uptime.hours % 24
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
    if let Some(ref rootfs_fstype) = info.rootfs_fstype {
        result.add("RootFS filesystem", &rootfs_fstype);
    }

    Some(result)
}
