use crate::utils;
use std::collections::BTreeMap;
use std::process::Command;

pub struct PackageManager {
    pub manager: &'static str,
    pub count_of_packages: i32,
}

pub fn detect_managers(managers: Vec<&'static str>) -> Vec<&'static str> {
    let mut result: Vec<&'static str> = Vec::new();
    for mgr in managers {
        if !utils::fs::_which(mgr).is_empty() {
            result.push(mgr);
        }
    }

    result
}

pub fn get_info() -> Vec<PackageManager> {
    let mut result: Vec<PackageManager> = Vec::new();
    let list_packages_command = BTreeMap::from([
        ("pacman", Vec::from(["-Qq"])),
        ("dpkg", Vec::from(["-f", "'.\n'", "-W"])),
        ("rpm", Vec::from(["-qa"])),
        ("flatpak", Vec::from(["list"])),
    ]);

    let managers = detect_managers(list_packages_command.clone().into_keys().collect());
    if managers.is_empty() {
        return result;
    }
    for manager in managers {
        let mut manager_info = PackageManager {
            manager: manager.clone(),
            count_of_packages: 0,
        };

        if list_packages_command.contains_key(manager) {
            let args: Vec<String> = list_packages_command
                .get(manager)
                .unwrap()
                .iter()
                .map(|s| s.to_string())
                .collect();

            manager_info.count_of_packages = Command::new(manager)
                .args(args)
                .output()
                .unwrap()
                .stdout
                .iter()
                .filter(|b| **b == b'\n')
                .count() as i32;
        }

        result.push(manager_info);
    }

    result
}
