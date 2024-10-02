/*
all tables:
    system
    processor
    memory
    graphics
    packages
    disks
    battery
*/

use libscu::software::users::fetch_current;
use std::{fs, path::Path};

const ALL_TABLES: [&str; 7] = [
    "system",
    "processor",
    "memory",
    "graphics",
    "packages",
    "disks",
    "battery",
];
static DEFAULT_CONFIG: &str = "system,processor,graphics,memory,packages,disks,battery";
const CONFIG_DIRECTORY: &str = "$HOME/.config/omnid";
const CONFIG_FILENAME: &str = "scu";

pub struct Config {
    pub order: Vec<String>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            order: Self::load(),
        }
    }
    fn load() -> Vec<String> {
        let default = DEFAULT_CONFIG
            .split(",")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        if let Some(cfg_path) = Self::init() {
            if let Ok(cfg_content) = fs::read_to_string(cfg_path) {
                let order: Vec<String> = cfg_content
                    .replace(" ", "")
                    .trim()
                    .split(",")
                    .filter(|info_table| ALL_TABLES.contains(info_table))
                    .map(|s| s.to_string())
                    .collect();

                if !order.is_empty() {
                    return order;
                }
            }
        }
        default
    }
    fn create_config_directory(path: &Path) -> bool {
        path.exists()
            || fs::create_dir_all(&path)
                .map_err(|err| {
                    eprintln!("failed to create directory `{CONFIG_DIRECTORY}` for config: {err:?}")
                })
                .is_ok()
    }
    fn init() -> Option<std::path::PathBuf> {
        if let Some(user) = fetch_current() {
            if let Some(homedir) = user.home_dir {
                if homedir.is_empty() {
                    return None;
                }
                let full_path =
                    std::path::PathBuf::from(CONFIG_DIRECTORY.replace("$HOME", &homedir));

                if full_path.join(CONFIG_FILENAME).exists()
                    || (Self::create_config_directory(&full_path)
                        && fs::write(full_path.join(CONFIG_FILENAME), DEFAULT_CONFIG).is_ok())
                {
                    return Some(full_path);
                }
            }
        }
        None
    }
}
