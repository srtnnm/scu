use super::{super::row::DataRow, Display};

use crate::modules::{
    get_option, Arch, Detection, DisplayServer, Hostname, Init, Kernel, Locale, Packages, RootFS,
    Shell, Terminal, Uptime, Username, DE, OS, WM,
};

impl Display for Arch {
    fn display(arch: Self::Result) -> std::io::Result<usize> {
        Ok(DataRow::info("Arch", &arch))
    }
}

impl Display for DE {
    fn display(de: Self::Result) -> std::io::Result<usize> {
        Ok(DataRow::info("DE", de.to_str()))
    }
}

impl Display for DisplayServer {
    fn display(display_server: Self::Result) -> std::io::Result<usize> {
        Ok(DataRow::info(
            "Display server",
            &format!("{display_server:?}"),
        ))
    }
}

impl Display for Hostname {
    fn display(hostname: Self::Result) -> std::io::Result<usize> {
        Ok(DataRow::info("Hostname", &hostname))
    }
}

impl Display for Init {
    fn display(init: Self::Result) -> std::io::Result<usize> {
        let mut value = init.name;
        if let Some(number_of_services) = init.number_of_services {
            value.push_str(&format!(" ({number_of_services} services)"));
        }
        Ok(DataRow::info("Init", &value))
    }
}

impl Display for Kernel {
    fn display(kernel: Self::Result) -> std::io::Result<usize> {
        Ok(DataRow::info(
            "Kernel",
            &format!("{} {}", kernel.name, kernel.version),
        ))
    }
}

impl Display for Locale {
    fn display(locale: Self::Result) -> std::io::Result<usize> {
        Ok(DataRow::info("Locale", &locale))
    }
}

impl Display for OS {
    fn display(os_release: Self::Result) -> std::io::Result<usize> {
        let os = if !os_release.pretty_name.is_empty() {
            os_release.pretty_name
        } else if !os_release.name.is_empty() {
            os_release.name
        } else if !os_release.id.is_empty() {
            os_release.id
        } else {
            "Unknown OS".to_string()
        };
        let arch = Arch.fetch().unwrap_or_default();

        Ok(DataRow::info("OS", format!("{os} {arch}").trim()))
    }
}

impl Display for Packages {
    fn display(mut package_managers: Self::Result) -> std::io::Result<usize> {
        package_managers.sort_by(|pm1, pm2| pm2.number_of_packages.cmp(&pm1.number_of_packages));

        let mut value = String::default();
        for pm in package_managers {
            value.push_str(&format!(
                "{packages} ({manager}), ",
                packages = pm.number_of_packages,
                manager = pm.name
            ));
        }
        value.pop();
        value.pop();
        Ok(DataRow::info("Packages", &value))
    }
}

impl Display for RootFS {
    fn display(fstype: Self::Result) -> std::io::Result<usize> {
        Ok(DataRow::info("RootFS filesystem", &fstype))
    }
}

impl Display for Shell {
    fn display(shell: Self::Result) -> std::io::Result<usize> {
        let mut shell_str = shell.name;
        if let Some(ref version) = shell.version {
            shell_str.push(' ');
            shell_str.push_str(version);
        }

        Ok(DataRow::info("Shell", &shell_str))
    }
}

impl Display for Terminal {
    fn display(terminal: Self::Result) -> std::io::Result<usize> {
        let mut terminal_str = terminal.name;
        if let Some(ref version) = terminal.version {
            terminal_str.push(' ');
            terminal_str.push_str(version);
        }

        Ok(DataRow::info("Terminal", &terminal_str))
    }
}

impl Display for Uptime {
    fn display(uptime: Self::Result) -> std::io::Result<usize> {
        let mut value = String::default();
        for (int, name) in [
            (&uptime.days, "days"),
            (&uptime.hours, "hours"),
            (&uptime.minutes, "mins"),
        ] {
            if int > &0 {
                value.push_str(&format!("{int} {name}, "));
            }
        }
        value.pop();
        value.pop();
        Ok(DataRow::info("Uptime", &value))
    }
}

impl Display for Username {
    fn display(username: Self::Result) -> std::io::Result<usize> {
        Ok(DataRow::info("Username", &username))
    }
}

impl Display for WM {
    fn display(wm: Self::Result) -> std::io::Result<usize> {
        let mut wm_str = get_option("window manager name", &wm.name)?.to_string();
        if let Some(ref version) = wm.version {
            wm_str.push(' ');
            wm_str.push_str(version);
        }

        Ok(DataRow::info("WM", &wm_str))
    }
}
