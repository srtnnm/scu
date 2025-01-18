// TODO: PLACE INTO ARGUMENT PARSING
fn disable_colors() -> bool {
    false
}

use super::GenerateTableEntries;

use crate::{
    data::{distro_colors, table::TableEntry},
    modules::{
        DisplayServer, Hostname, Init, Kernel, Locale, Packages, RootFS, Shell, Terminal, Uptime,
        Username, DE, OS, WM,
    },
    util::colorize::colorize_background,
};

impl GenerateTableEntries for DE {
    fn display(environment: Self::Result, table: &mut crate::data::table::Table) {
        table.add("Environment", environment.to_str());
    }
}

impl GenerateTableEntries for DisplayServer {
    fn display(display_server: Self::Result, table: &mut crate::data::table::Table) {
        table.add("Display server", format!("{:?}", display_server).as_str());
    }
}

impl GenerateTableEntries for Hostname {
    fn display(hostname: Self::Result, table: &mut crate::data::table::Table) {
        table.add("Hostname", &hostname);
    }
}

impl GenerateTableEntries for Init {
    fn display(init_system: Self::Result, table: &mut crate::data::table::Table) {
        let num_services = if let Some(number_of_services) = init_system.number_of_services {
            Vec::from([TableEntry::new("Services", &number_of_services.to_string())])
        } else {
            Vec::new()
        };
        table.add_with_additional("Init system", &init_system.name, &num_services);
    }
}

impl GenerateTableEntries for Kernel {
    fn display(kernel: Self::Result, table: &mut crate::data::table::Table) {
        table.add("Kernel", &format!("{} {}", kernel.name, kernel.version))
    }
}

impl GenerateTableEntries for Locale {
    fn display(locale: Self::Result, table: &mut crate::data::table::Table) {
        table.add("Locale", &locale)
    }
}

impl GenerateTableEntries for OS {
    fn display(os_release: Self::Result, table: &mut crate::data::table::Table) {
        let name = if os_release.pretty_name.is_empty() {
            os_release.name
        } else {
            os_release.pretty_name
        };
        table.add(
            "OS",
            (if !disable_colors() {
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
}

impl GenerateTableEntries for Packages {
    fn display(package_managers: Self::Result, table: &mut crate::data::table::Table) {
        for manager in package_managers {
            table.add(&manager.name, &manager.number_of_packages.to_string())
        }
    }
}

impl GenerateTableEntries for RootFS {
    fn display(fstype: Self::Result, table: &mut crate::data::table::Table) {
        table.add("RootFS filesystem", &fstype)
    }
}

impl GenerateTableEntries for Shell {
    fn display(shell: Self::Result, table: &mut crate::data::table::Table) {
        table.add(
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
        );
    }
}

impl GenerateTableEntries for Terminal {
    fn display(terminal: Self::Result, table: &mut crate::data::table::Table) {
        table.add(
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
        );
    }
}

impl GenerateTableEntries for Uptime {
    fn display(uptime: Self::Result, table: &mut crate::data::table::Table) {
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
        table.add("Uptime", uptime_str.trim());
    }
}

impl GenerateTableEntries for Username {
    fn display(username: Self::Result, table: &mut crate::data::table::Table) {
        table.add("Username", &username)
    }
}

impl GenerateTableEntries for WM {
    fn display(wm: Self::Result, table: &mut crate::data::table::Table) {
        if let Some(ref name) = wm.name {
            table.add(
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
            );
        }
    }
}
