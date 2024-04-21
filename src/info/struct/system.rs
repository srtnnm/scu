use crate::{
    data::{
        distro_colors,
        table::{Table, TableEntry},
    },
    utils::colorize::colorize_background,
};

use libscu::{
    software::{shell::Shell, terminal::TerminalInfo},
    utils::converter::Time,
};
#[cfg(not(target_os = "android"))]
use libscu::software::init::InitSystem;

#[derive(Default)]
pub struct System {
    pub hostname: Option<String>,
    pub username: Option<String>,
    pub os_name: Option<String>,
    pub device_name: Option<String>,
    pub kernel_version: Option<String>,
    #[cfg(not(target_os = "android"))]
    pub init_system: Option<InitSystem>,
    pub terminal_info: Option<TerminalInfo>,
    pub shell: Option<Shell>,
    pub uptime: Option<Time>,
}

impl System {
    pub fn to_print(&self, disable_color: bool) -> Table {
        let mut result = Table::new("System");

        if let Some(hostname) = self.hostname.as_ref() {
            result.add("Hostname", &hostname)
        }

        if let Some(user) = self.username.as_ref() {
            result.add("Username", &user)
        }

        if let Some(osrelease) = self.os_name.clone() {
            result.add(
                "Distro",
                (if disable_color {
                    osrelease
                } else {
                    match distro_colors::get_color(&osrelease) {
                        Some(clr) => colorize_background(&osrelease, clr.r, clr.g, clr.b),
                        None => osrelease,
                    }
                })
                .as_str(),
            );
        }

        if let Some(device_name) = self.device_name.clone() {
            result.add("Device", &device_name);
        }

        if let Some(kernel_version) = self.kernel_version.clone() {
            result.add("Kernel", &kernel_version);
        }

        #[cfg(not(target_os = "android"))]
        if let Some(init_system) = self.init_system.as_ref() {
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

        if let Some(terminal_info) = self.terminal_info.as_ref() {
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

        if let Some(shell) = self.shell.as_ref() {
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
        if let Some(uptime) = self.uptime.as_ref() {
            let mut uptime_str = String::new();
            if uptime.hours >= 24 {
                uptime_str += format!("{}d", uptime.hours / 24).as_str();
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

        result
    }
}
