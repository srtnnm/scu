use super::{DisplayModule, DisplaySenderT};

use crate::{
    config::no_colors,
    data::{distro_colors, table::TableEntry},
    modules::{
        DisplayServer, Hostname, Init, Kernel, Locale, Packages, RootFS, Shell, Terminal, Uptime,
        Username, DE, OS, WM,
    },
    util::colorize::colorize_background,
};

impl DisplayModule<DisplaySenderT> for DE {
    fn display(environment: Self::Result, sender: &DisplaySenderT) {
        sender.send(TableEntry::new("Environment", environment.to_str()));
    }
}

impl DisplayModule<DisplaySenderT> for DisplayServer {
    fn display(display_server: Self::Result, sender: &DisplaySenderT) {
        sender.send(TableEntry::new(
            "Display server",
            format!("{:?}", display_server).as_str(),
        ));
    }
}

impl DisplayModule<DisplaySenderT> for Hostname {
    fn display(hostname: Self::Result, sender: &DisplaySenderT) {
        sender.send(TableEntry::new("Hostname", &hostname));
    }
}

impl DisplayModule<DisplaySenderT> for Init {
    fn display(init_system: Self::Result, sender: &DisplaySenderT) {
        let num_services = if let Some(number_of_services) = init_system.number_of_services {
            Vec::from([TableEntry::new("Services", &number_of_services.to_string())])
        } else {
            Vec::new()
        };
        sender.send(TableEntry::new_with_additional(
            "Init system",
            &init_system.name,
            &num_services,
        ));
    }
}

impl DisplayModule<DisplaySenderT> for Kernel {
    fn display(kernel: Self::Result, sender: &DisplaySenderT) {
        sender.send(TableEntry::new(
            "Kernel",
            &format!("{} {}", kernel.name, kernel.version),
        ))
    }
}

impl DisplayModule<DisplaySenderT> for Locale {
    fn display(locale: Self::Result, sender: &DisplaySenderT) {
        sender.send(TableEntry::new("Locale", &locale))
    }
}

impl DisplayModule<DisplaySenderT> for OS {
    fn display(os_release: Self::Result, sender: &DisplaySenderT) {
        let name = if os_release.pretty_name.is_empty() {
            os_release.name
        } else {
            os_release.pretty_name
        };
        sender.send(TableEntry::new(
            "OS",
            match distro_colors::get_color(&name) {
                Some(clr) if !no_colors() => colorize_background(&name, clr.r, clr.g, clr.b),
                _ => name,
            }
            .as_str(),
        ));
    }
}

impl DisplayModule<DisplaySenderT> for Packages {
    fn display(package_managers: Self::Result, sender: &DisplaySenderT) {
        for manager in package_managers {
            sender.send(TableEntry::new(
                &manager.name,
                &manager.number_of_packages.to_string(),
            ))
        }
    }
}

impl DisplayModule<DisplaySenderT> for RootFS {
    fn display(fstype: Self::Result, sender: &DisplaySenderT) {
        sender.send(TableEntry::new("RootFS filesystem", &fstype))
    }
}

impl DisplayModule<DisplaySenderT> for Shell {
    fn display(shell: Self::Result, sender: &DisplaySenderT) {
        sender.send(TableEntry::new(
            "Shell",
            format!(
                "{}{}",
                shell.name,
                if let Some(ref shell_version) = shell.version {
                    format!(" v{}", shell_version)
                } else {
                    "".to_string()
                }
            )
            .as_str(),
        ));
    }
}

impl DisplayModule<DisplaySenderT> for Terminal {
    fn display(terminal: Self::Result, sender: &DisplaySenderT) {
        sender.send(TableEntry::new(
            "Terminal",
            &format!(
                "{}{}",
                terminal.name,
                if let Some(ref version) = terminal.version {
                    format!(" v{version}")
                } else {
                    "".to_string()
                }
            ),
        ));
    }
}

impl DisplayModule<DisplaySenderT> for Uptime {
    fn display(uptime: Self::Result, sender: &DisplaySenderT) {
        let mut uptime_str = String::new();
        if uptime.days > 0 {
            uptime_str += format!("{}d", uptime.days).as_str();
        }
        uptime_str += format!(
            " {:02}:{:02}:{:02}",
            uptime.hours, uptime.minutes, uptime.seconds
        )
        .as_str();
        sender.send(TableEntry::new("Uptime", uptime_str.trim()));
    }
}

impl DisplayModule<DisplaySenderT> for Username {
    fn display(username: Self::Result, sender: &DisplaySenderT) {
        sender.send(TableEntry::new("Username", &username))
    }
}

impl DisplayModule<DisplaySenderT> for WM {
    fn display(wm: Self::Result, sender: &DisplaySenderT) {
        if let Some(ref name) = wm.name {
            sender.send(TableEntry::new(
                "Window manager",
                format!(
                    "{}{}",
                    name,
                    wm.version
                        .as_ref()
                        .map(|v| format!(" v{v}"))
                        .unwrap_or_default()
                )
                .as_str(),
            ));
        }
    }
}
