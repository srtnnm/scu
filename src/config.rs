/*
all tables:
    system
    processor
    memory
    graphics
    packages
    drives
    battery
*/

use libscu::software::users::fetch_current;
use std::fs;

const ALL_TABLES: [&str; 7] = [
    "system",
    "processor",
    "memory",
    "graphics",
    "packages",
    "drives",
    "battery",
];
static DEFAULT_CONFIG: &str = "system,processor,graphics,memory,packages,drives,battery";
const CONFIG_PATH: &str = "$HOME/.config/scu";

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
                    .filter(|info_table| !info_table.is_empty() && ALL_TABLES.contains(info_table))
                    .map(|s| s.to_string())
                    .collect();

                if !order.is_empty() {
                    return order;
                }
            }
        }
        default
    }
    fn init() -> Option<std::path::PathBuf> {
        if let Some(user) = fetch_current() {
            if let Some(homedir) = user.home_dir {
                if homedir.is_empty() {
                    return None;
                }
                let full_path = std::path::PathBuf::from(
                    CONFIG_PATH.replace("$HOME", &homedir),
                );
                if full_path.exists()
                    || (fs::create_dir_all(full_path.parent().unwrap()).is_ok()
                        && fs::write(full_path.clone(), DEFAULT_CONFIG).is_ok())
                {
                    return Some(full_path);
                }
            }
        }
        None
    }
}
