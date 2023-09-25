use crate::utils::process;
use std::fs;
use std::path::{Path, PathBuf};

pub struct InitSystem {
    pub name: String,
    pub count_services: u16,
}

use std::os::unix::fs::PermissionsExt;
fn is_executable(file: PathBuf) -> bool {
    let metadata = match file.metadata() {
        Ok(metadata) => metadata,
        Err(_) => return false,
    };
    let permissions = metadata.permissions();
    metadata.is_file() && permissions.mode() & 0o111 != 0
}

fn is_service(file: PathBuf) -> bool {
    if (is_executable(file.clone())
        || (file.is_file() && file.to_string_lossy().contains(".rc")/* android */))
        && !file.is_symlink()
    {
        return true;
    }

    false
}

pub fn detect() -> Option<InitSystem> {
    let mut result = InitSystem {
        name: "".to_string(),
        count_services: 0,
    };
    let proc_info = process::get_info(1);

    if let Ok(proc_info) = proc_info {
        result.name = String::from(match proc_info.command.trim() {
            "systemd" => "SystemD",
            "openrc-init" | "init-openrc" => "OpenRC",
            "runit" => "Runit",
            "init" => {
                if Path::new("/run/dinit").exists() {
                    "Dinit"
                } else if Path::new("/usr/share/sysvinit/inittab").exists()
                    || Path::new("/etc/inittab").exists()
                {
                    "SysVinit"
                } else if std::fs::read_link(proc_info.cmdline.split('\0').next().unwrap())
                    .unwrap_or(PathBuf::from("".to_string()))
                    .to_str()
                    == Some("openrc-init")
                {
                    "OpenRC"
                } else {
                    "Unknown"
                }
            }
            "s6-svscan" => "S6",
            "upstart" => "Upstart",
            _ => "Unknown",
        });

        for services_dir in ["/etc/init.d", "/etc/init"] {
            if Path::new(services_dir).exists() {
                for file in fs::read_dir(services_dir).unwrap() {
                    let file = file.unwrap().path();
                    if is_service(Path::new(file.as_path()).to_path_buf()) {
                        result.count_services += 1;
                    }
                }
            }
            if result.count_services != 0 {
                break;
            }
        }

        if result.name != "Unknown" {
            Some(result)
        } else {
            None
        }
    } else {
        None
    }
}
