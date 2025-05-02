use super::{super::row::DataRow, RowSenderT};

use crate::modules::{
    Arch, Detection, DisplayModule, DisplayServer, Hostname, Init, Kernel, Locale, Packages,
    RootFS, Shell, Terminal, Uptime, Username, DE, OS, WM,
};

impl DisplayModule<RowSenderT> for Arch {
    fn display(arch: Self::Result, sender: &RowSenderT) {
        DataRow::info("Arch", &arch, sender);
    }
}

impl DisplayModule<RowSenderT> for DE {
    fn display(de: Self::Result, sender: &RowSenderT) {
        DataRow::info("DE", de.to_str(), sender);
    }
}

impl DisplayModule<RowSenderT> for DisplayServer {
    fn display(display_server: Self::Result, sender: &RowSenderT) {
        DataRow::info(
            "DisplayModule<RowSenderT> server",
            &format!("{display_server:?}"),
            sender,
        )
    }
}

impl DisplayModule<RowSenderT> for Hostname {
    fn display(hostname: Self::Result, sender: &RowSenderT) {
        DataRow::info("Hostname", &hostname, sender)
    }
}

impl DisplayModule<RowSenderT> for Init {
    fn display(init: Self::Result, sender: &RowSenderT) {
        let mut value = init.name;
        if let Some(number_of_services) = init.number_of_services {
            value.push_str(&format!(" ({number_of_services} services)"));
        }
        DataRow::info("Init", &value, sender)
    }
}

impl DisplayModule<RowSenderT> for Kernel {
    fn display(kernel: Self::Result, sender: &RowSenderT) {
        DataRow::info(
            "Kernel",
            &format!("{} {}", kernel.name, kernel.version),
            sender,
        )
    }
}

impl DisplayModule<RowSenderT> for Locale {
    fn display(locale: Self::Result, sender: &RowSenderT) {
        DataRow::info("Locale", &locale, sender)
    }
}

impl DisplayModule<RowSenderT> for OS {
    fn display(os_release: Self::Result, sender: &RowSenderT) {
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

        DataRow::info("OS", format!("{os} {arch}").trim(), sender)
    }
}

impl DisplayModule<RowSenderT> for Packages {
    fn display(mut package_managers: Self::Result, sender: &RowSenderT) {
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
        DataRow::info("Packages", &value, sender)
    }
}

impl DisplayModule<RowSenderT> for RootFS {
    fn display(fstype: Self::Result, sender: &RowSenderT) {
        DataRow::info("RootFS filesystem", &fstype, sender)
    }
}

impl DisplayModule<RowSenderT> for Shell {
    fn display(shell: Self::Result, sender: &RowSenderT) {
        let mut shell_str = shell.name;
        if let Some(ref version) = shell.version {
            shell_str.push(' ');
            shell_str.push_str(version);
        }

        DataRow::info("Shell", &shell_str, sender)
    }
}

impl DisplayModule<RowSenderT> for Terminal {
    fn display(terminal: Self::Result, sender: &RowSenderT) {
        let mut terminal_str = terminal.name;
        if let Some(ref version) = terminal.version {
            terminal_str.push(' ');
            terminal_str.push_str(version);
        }

        DataRow::info("Terminal", &terminal_str, sender)
    }
}

impl DisplayModule<RowSenderT> for Uptime {
    fn display(uptime: Self::Result, sender: &RowSenderT) {
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
        DataRow::info("Uptime", &value, sender)
    }
}

impl DisplayModule<RowSenderT> for Username {
    fn display(username: Self::Result, sender: &RowSenderT) {
        DataRow::info("Username", &username, sender)
    }
}

impl DisplayModule<RowSenderT> for WM {
    fn display(wm: Self::Result, sender: &RowSenderT) {
        let Some(window_name) = wm.name else {
            return;
        };

        let mut wm_str = window_name.to_string();
        if let Some(ref version) = wm.version {
            wm_str.push(' ');
            wm_str.push_str(version);
        }

        DataRow::info("WM", &wm_str, sender)
    }
}
