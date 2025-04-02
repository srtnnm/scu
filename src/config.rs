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

/*
    TODO
    implement Config::from_string()
*/

use bitflags::bitflags;
use libscu::software::users::fetch_current;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::sync::{LazyLock, OnceLock};
use std::{fs, path::Path};

// bitflags! {
//     #[derive(Clone, Copy, Debug, PartialEq, Eq)]
//     pub struct Table: u8 {
//         const SYSTEM    = 0b0000001;
//         const PROCESSOR = 0b0000010;
//         const MEMORY    = 0b0000100;
//         const GRAPHICS  = 0b0001000;
//         const PACKAGES  = 0b0010000;
//         const DISKS     = 0b0100000;
//         const BATTERY   = 0b1000000;
//     }
// }

// impl Table {
//     pub fn from_str(str: &str) -> Option<Self> {
//         match str {
//             "system" => Some(Self::SYSTEM),
//             "processor" => Some(Self::PROCESSOR),
//             "memory" => Some(Self::MEMORY),
//             "graphics" => Some(Self::GRAPHICS),
//             "packages" => Some(Self::PACKAGES),
//             "disks" => Some(Self::DISKS),
//             "battery" => Some(Self::BATTERY),
//             "full" => Some(Self::all()),
//             _ => None,
//         }
//     }
//     pub fn to_str(&self) -> &'static str {
//         match *self {
//             Self::SYSTEM => "system",
//             Self::PROCESSOR => "processor",
//             Self::MEMORY => "memory",
//             Self::GRAPHICS => "graphics",
//             Self::PACKAGES => "packages",
//             Self::DISKS => "disks",
//             Self::BATTERY => "battery",
//             _ => "",
//         }
//     }
// }

// impl Default for Table {
//     fn default() -> Self {
//         Self::SYSTEM
//             | Self::PROCESSOR
//             | Self::MEMORY
//             | Self::GRAPHICS
//             | Self::PACKAGES
//             | Self::DISKS
//             | Self::BATTERY
//     }
// }

// const DEFAULT_CONFIG_ORDER: [Table; 7] = [
//     Table::SYSTEM,
//     Table::PROCESSOR,
//     Table::MEMORY,
//     Table::GRAPHICS,
//     Table::PACKAGES,
//     Table::DISKS,
//     Table::BATTERY,
// ];
// const CONFIG_DIRECTORY: &str = "$HOME/.config/omnid";
// const CONFIG_FILENAME: &str = "scu";

// fn default_config_to_file() -> String {
//     DEFAULT_CONFIG_ORDER
//         .iter()
//         .map(|table| table.to_str())
//         .filter(|table| !table.is_empty())
//         .fold(String::new(), |a, b| a + b + " ")
//         .trim_end()
//         .replace(" ", ",")
// }

// #[derive(Debug)]
// pub struct Config {
//     pub order: Vec<Table>,
// }

// impl Config {
//     pub fn load() -> Self {
//         if let Some(cfg_path) = Self::init() {
//             if let Ok(cfg_content) = fs::read_to_string(cfg_path) {
//                 let config_order: Vec<String> = cfg_content
//                     .replace(" ", "")
//                     .trim()
//                     .split(",")
//                     .map(|s| s.to_string())
//                     .collect();

//                 let order: Vec<Table> = config_order
//                     .iter()
//                     .map(|table| Table::from_str(&table))
//                     .flatten()
//                     .collect();

//                 if !order.is_empty() {
//                     return Self { order };
//                 }
//             }
//         }
//         Self::default()
//     }
//     fn create_config_directory(path: &Path) -> bool {
//         path.exists()
//             || fs::create_dir_all(&path)
//                 .map_err(|err| {
//                     eprintln!("failed to create directory `{CONFIG_DIRECTORY}` for config: {err:?}")
//                 })
//                 .is_ok()
//     }
//     fn init() -> Option<std::path::PathBuf> {
//         if let Ok(user) = fetch_current() {
//             if let Some(homedir) = user.home_dir {
//                 if homedir.is_empty() {
//                     return None;
//                 }
//                 let full_path =
//                     std::path::PathBuf::from(CONFIG_DIRECTORY.replace("$HOME", &homedir));

//                 if full_path.join(CONFIG_FILENAME).exists()
//                     || (Self::create_config_directory(&full_path)
//                         && fs::write(full_path.join(CONFIG_FILENAME), default_config_to_file())
//                             .is_ok())
//                 {
//                     return Some(full_path.join(CONFIG_FILENAME));
//                 }
//             }
//         }
//         None
//     }
// }

// impl Default for Config {
//     fn default() -> Self {
//         Self {
//             order: DEFAULT_CONFIG_ORDER.into_iter().collect(),
//         }
//     }
// }

use std::sync::atomic::{AtomicBool, Ordering};

macro_rules! setup_loaders {
    ($($var:ident in_config:$string_repr:tt default_val:$default:tt getter:$fn_name:ident,)*) => {
        $(
            pub(super) static $var: AtomicBool = AtomicBool::new($default);
            pub(crate) fn $fn_name() -> bool {
                $var.load(Ordering::Relaxed)
            }
        )*
        const NUMBER_OF_BOOL_PROPERTIES: usize = {
            let mut count = 0;
            $(
                let _ = $var;
                count += 1;
            )*
            count
        };
        pub static PROPERTY_CONFIG_REPRESENTATION: &[(&str, &'static AtomicBool)] = &[
            $(
                ($string_repr, &$var),
            )*
        ];
        fn find_atomic_by_key(name: &str) -> Option<&'static AtomicBool> {
            for (key,atomic) in PROPERTY_CONFIG_REPRESENTATION {
                if *key==name {return Some(atomic)}
            }
            None
        }
    };
}

setup_loaders!(
    RAW_MODELS     in_config:"raw_models"     default_val:false getter:raw_models,
    SIMPLIFY       in_config:"simplify"       default_val:false getter:simplify,
    NO_COLORS      in_config:"no_colors"      default_val:false getter:no_colors,
    NO_LOGO        in_config:"no_logo"        default_val:false getter:no_logo,
    MULTICPU       in_config:"multicpu"       default_val:false getter:multicpu,
    NEOMIMIC       in_config:"neomimic"       default_val:false getter:neomimic,
    FORCE_VERSIONS in_config:"force_versions" default_val:false getter:force_versions,
);

pub static NEOMIMIC_CONFIG: OnceLock<NeomimicConfig> = OnceLock::new();
pub static TABLE_CONFIG: OnceLock<TableConfig> = OnceLock::new();

pub(crate) enum ConfigData {
    RawModels,
    Simplify,
    Multicpu,
    Neomimic,
    ForceVersions,
}

pub(crate) fn set(data: ConfigData, value: bool) {
    use ConfigData::*;
    match data {
        RawModels => RAW_MODELS.store(value, Ordering::Relaxed),
        Simplify => SIMPLIFY.store(value, Ordering::Relaxed),
        Multicpu => MULTICPU.store(value, Ordering::Relaxed),
        Neomimic => NEOMIMIC.store(value, Ordering::Relaxed),
        ForceVersions => FORCE_VERSIONS.store(value, Ordering::Relaxed),
    }
}

pub struct Config;
impl Config {
    pub fn find_config(name: Option<&str>) -> Option<PathBuf> {
        const CONFIG_DIRECTORY: &str = "$HOME/.config/omnid/scu";
        let mut name = name?.to_string();
        if !name.ends_with(".json") {
            name.push_str(".json");
        }

        let path = Path::new(&name);
        if path.is_file() {
            // if provided path instead of config name
            return Some(path.to_path_buf());
        }

        // get user's HOME directory
        let user = fetch_current().ok()?;
        let home_dir = user.home_dir?;
        if home_dir.is_empty() {
            return None;
        }

        // check if config directory is file (deprecated format)
        let full_path = PathBuf::from(CONFIG_DIRECTORY.replace("$HOME", &home_dir));
        if full_path.is_file() {
            logs::warning!("old config {} is file. it is now deprecated so you need remove that file and create directory with the same name",
                full_path.to_string_lossy());
            return None;
        }

        // create directory for configs
        if !Self::create_config_directory(&full_path) {
            return None;
        }

        if full_path.join(&name).is_file() {
            Some(full_path.join(name))
        } else {
            None
        }
    }
    fn create_config_directory(path: &Path) -> bool {
        path.is_dir()
            || fs::create_dir_all(&path)
                .map_err(|err| {
                    logs::warning!(
                        "failed to create directory `{}` for config: {err}",
                        path.to_string_lossy()
                    )
                })
                .is_ok()
    }
    pub fn parse(path: &Path) -> std::io::Result<()> {
        let read_file = std::fs::read_to_string(path)?;
        let parsed_json = json::parse(&read_file)
            .map_err(|error| Error::new(ErrorKind::InvalidData, error.to_string()))?;

        if let json::JsonValue::Object(global_object) = parsed_json["global"].clone() {
            Self::parse_global(&global_object);
        }
        TABLE_CONFIG
            .set(Self::parse_table_config(&parsed_json["table"]))
            .unwrap();
        let _ = NEOMIMIC_CONFIG.set(Self::parse_neomimic_config(&parsed_json["neomimic"]));

        Ok(())
    }
    fn parse_global(object: &json::object::Object) {
        for (key, value) in object.iter() {
            let Some(atomic_ref) = find_atomic_by_key(key) else {
                logs::warning!("unknown global property: `{key}`");
                continue;
            };
            let Some(value) = value.as_bool() else {
                logs::warning!("global property `{key}` must be boolean");
                continue;
            };

            atomic_ref.store(value, Ordering::Relaxed);
        }
    }
    fn parse_neomimic_config(value: &json::JsonValue) -> NeomimicConfig {
        NeomimicConfig::from_json(value).unwrap_or_default()
    }
    fn parse_table_config(value: &json::JsonValue) -> TableConfig {
        TableConfig::from_json(value).unwrap_or_default()
    }
}

use crate::{
    display_mode::{
        neomimic::{config::NeomimicConfig, logo::Logo},
        table::config::{TableCategory, TableConfig},
    },
    modules::Module,
};
impl NeomimicConfig {
    fn from_json(json_value: &json::JsonValue) -> Option<Self> {
        let json::JsonValue::Object(neomimic_config) = json_value.clone() else {
            return None;
        };

        Some(Self {
            logo: Self::logo_from_json(&neomimic_config["logo"]),
            modules: Module::from_json_array(&neomimic_config["modules"]),
        })
    }
    fn logo_from_json(value: &json::JsonValue) -> Logo {
        let logo = match value.clone() {
            json::JsonValue::String(logo) => logo,
            json::JsonValue::Null => {
                return Logo::default();
            }
            _ => {
                logs::warning!("neomimic `logo` must be a string value");
                return Logo::default();
            }
        };

        let logo_path = Path::new(&logo);
        let logo_path_exists = logo_path.is_absolute() && logo_path.is_file();

        match logo.as_str() {
            path if logo_path_exists => match Logo::from_path(path) {
                Ok(logo_from_file) => logo_from_file,
                Err(error) => {
                    logs::warning!("failed to read file `{logo}`: {error}");
                    Logo::default()
                }
            },
            "default" | "tux" => Logo::default(),
            _ => {
                logs::warning!("neomimic `logo` unknown name or path: `{logo}`");
                Logo::default()
            }
        }
    }
}
impl TableConfig {
    fn from_json(json_value: &json::JsonValue) -> Option<Self> {
        let table_config = match json_value.clone() {
            json::JsonValue::Object(table_config) => table_config,
            json::JsonValue::Null => {
                return None;
            }
            _ => {
                logs::warning!("`table` property has incorrect value. must be an Object");
                return None;
            }
        };

        let Some(categories) = table_config.get("categories") else {
            logs::warning!("`table` does not contains key `categories`");
            return None;
        };
        let json::JsonValue::Object(categories) = categories else {
            logs::warning!("`table`:`categories` has incorrect value. must be an Object");
            return None;
        };

        let mut table_config_categories: Vec<TableCategory> = Vec::new();

        for (key, value) in categories.iter() {
            if key.is_empty() {
                logs::warning!("category title cannot be empty");
                continue;
            } else if !value.is_array() {
                logs::warning!("category value must be an array of strings");
                continue;
            }

            table_config_categories.push(TableCategory::from_json(key, value));
        }

        Some(Self {
            categories: table_config_categories,
        })
    }
}
impl TableCategory {
    fn from_json(title: &str, value: &json::JsonValue) -> Self {
        let modules = match value {
            json::JsonValue::Array(_) => Module::from_json_array(value),
            _ => Vec::default(),
        };

        Self {
            title: title.to_string(),
            modules,
        }
    }
}

impl Module {
    fn from_json_array(value: &json::JsonValue) -> Vec<Self> {
        let json::JsonValue::Array(array) = value.clone() else {
            return Vec::default();
        };

        array
            .iter()
            .flat_map(Self::from_json_value)
            .collect::<Vec<Self>>()
    }
    fn from_json_value(value: &json::JsonValue) -> Option<Self> {
        let module_name = match value.clone() {
            json::JsonValue::String(module_name) => module_name,
            json::JsonValue::Null => {
                return None;
            }
            _ => {
                logs::warning!("invalid module name value: {value}");
                return None;
            }
        };

        Self::from_str(&module_name)
    }
}
