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

use bitflags::bitflags;
use libscu::software::users::fetch_current;
use std::{fs, path::Path};

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq)]
    struct Table: u8 {
        const SYSTEM    = 0b0000001;
        const PROCESSOR = 0b0000010;
        const MEMORY    = 0b0000100;
        const GRAPHICS  = 0b0001000;
        const PACKAGES  = 0b0010000;
        const DISKS     = 0b0100000;
        const BATTERY   = 0b1000000;
    }
}

impl Table {
    pub fn from_str(str: &str) -> Option<Self> {
        match str {
            "system" => Some(Self::SYSTEM),
            "processor" => Some(Self::PROCESSOR),
            "memory" => Some(Self::MEMORY),
            "graphics" => Some(Self::GRAPHICS),
            "packages" => Some(Self::PACKAGES),
            "disks" => Some(Self::DISKS),
            "battery" => Some(Self::BATTERY),
            "full" => Some(Self::all()),
            _ => None,
        }
    }
    pub fn to_str(&self) -> &'static str {
        match *self {
            Self::SYSTEM => "system",
            Self::PROCESSOR => "processor",
            Self::MEMORY => "memory",
            Self::GRAPHICS => "graphics",
            Self::PACKAGES => "packages",
            Self::DISKS => "disks",
            Self::BATTERY => "battery",
            _ => "",
        }
    }
    pub fn full() -> Self {
        Self::all()
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::SYSTEM
            | Self::PROCESSOR
            | Self::MEMORY
            | Self::GRAPHICS
            | Self::PACKAGES
            | Self::DISKS
            | Self::BATTERY
    }
}

const DEFAULT_CONFIG_ORDER: [Table; 7] = [
    Table::SYSTEM,
    Table::PROCESSOR,
    Table::MEMORY,
    Table::GRAPHICS,
    Table::PACKAGES,
    Table::DISKS,
    Table::BATTERY,
];
const CONFIG_DIRECTORY: &str = "$HOME/.config/omnid";
const CONFIG_FILENAME: &str = "scu";

fn default_config_to_file() -> String {
    DEFAULT_CONFIG_ORDER
        .iter()
        .map(|table| table.to_str())
        .filter(|table| !table.is_empty())
        .fold(String::new(), |a, b| a + b + " ")
        .trim_end()
        .replace(" ", ",")
}

#[derive(Debug)]
pub struct Config {
    pub enabled_tables: Table,
    pub order: Vec<Table>,
}

impl Config {
    pub fn load() -> Self {
        if let Some(cfg_path) = Self::init() {
            if let Ok(cfg_content) = fs::read_to_string(cfg_path) {
                let config_order: Vec<String> = cfg_content
                    .replace(" ", "")
                    .trim()
                    .split(",")
                    .map(|s| s.to_string())
                    .collect();

                let order: Vec<Table> = config_order
                    .iter()
                    .map(|table| Table::from_str(&table))
                    .flatten()
                    .collect();
                let mut config = Table::empty();
                if order.contains(&Table::full()) {
                    config = Table::full();
                } else {
                    for table in order.iter() {
                        config.insert(*table);
                    }
                }

                if !order.is_empty() {
                    return Self {
                        enabled_tables: config,
                        order,
                    };
                }
            }
        }
        Self::default()
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
                        && fs::write(full_path.join(CONFIG_FILENAME), default_config_to_file())
                            .is_ok())
                {
                    return Some(full_path.join(CONFIG_FILENAME));
                }
            }
        }
        None
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            enabled_tables: Table::default(),
            order: DEFAULT_CONFIG_ORDER.into_iter().collect()
        }
    }
}