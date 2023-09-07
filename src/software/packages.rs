use crate::utils;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process::Command;

pub struct PackageManager {
    pub manager: &'static str,
    pub count_of_packages: i32,
}

pub fn detect_managers(managers: Vec<&'static str>) -> Vec<&'static str> {
    let mut result: Vec<&'static str> = Vec::new();
    for mgr in managers {
        if utils::fs::which(mgr).is_some() {
            result.push(mgr);
        }
    }

    result
}

pub fn get_info() -> Vec<PackageManager> {
    let mut result: Vec<PackageManager> = Vec::new();
    let list_packages_command = BTreeMap::from([
        ("apt", Vec::from(["list", "--installed"])),
        ("emerge", Vec::from([])),
        ("flatpak", Vec::from(["list"])),
        ("pacman", Vec::from(["-Qq"])),
        ("rpm", Vec::from(["-qa"])),
        ("snap", Vec::from(["list"])),
    ]);

    let managers = detect_managers(list_packages_command.clone().into_keys().collect());
    if managers.is_empty() {
        return result;
    }
    for manager in managers {
        let mut manager_info = PackageManager {
            manager,
            count_of_packages: 0,
        };

        if manager == "emerge" && Path::new("/var/db/pkg").exists() {
            fs::read_dir("/var/db/pkg").unwrap().for_each(|pkg_dir| {
                fs::read_dir(pkg_dir.unwrap().path().as_path())
                    .unwrap()
                    .for_each(|_| manager_info.count_of_packages += 1);
            });
            result.push(manager_info);
            continue;
        }

        if list_packages_command.contains_key(manager) {
            let args: Vec<String> = list_packages_command
                .get(manager)
                .unwrap()
                .iter()
                .map(|s| s.to_string())
                .collect();

            if args.is_empty() {
                continue;
            }

            manager_info.count_of_packages = Command::new(manager)
                .args(args)
                .output()
                .unwrap()
                .stdout
                .iter()
                .filter(|b| **b == b'\n')
                .count() as i32
                - if manager == "snap" { 1 } else { 0 };
        }

        if manager_info.count_of_packages > 0 {
            result.push(manager_info);
        }
    }

    result
}
