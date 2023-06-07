use regex::Regex;
use std::fs;

pub struct Kernel {
    pub full_version: String,
    pub version: String,
}

pub fn get_info() -> Kernel {
    let version_re = Regex::new(r"(\d\.?)+").unwrap();
    if !fs::metadata("/proc/version").is_ok() {
        return Kernel {
            full_version: String::from("Unknown"),
            version: String::from("Unknown"),
        };
    }
    let content = String::from(fs::read_to_string("/proc/version").unwrap().as_str());
    Kernel {
        full_version: content.split(' ').nth(2).unwrap().to_string(),
        version: String::from(version_re.find(content.as_str()).unwrap().as_str()),
    }
}
