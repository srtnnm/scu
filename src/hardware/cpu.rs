use crate::utils;
use regex::Regex;
use std::fs;

pub struct CPUInfo {
    pub model: String,
    pub max_freq: utils::converter::Frequency,
    pub cores: u8,
    pub threads: u8,
}

fn extract_i64(_str: &str) -> i64 {
    Regex::new(r"\d+")
        .unwrap()
        .find(_str)
        .unwrap()
        .as_str()
        .parse::<i64>()
        .unwrap()
}

fn extract_model_name(mut _str: String) -> String {
    for trash in [
        "Intel",
        "AMD",
        "\\((TM|tm)\\)",
        "\\((R|r)\\)",
        "CPU",
        "Processor",
        "[Dual,Quad,Six,Eight]-Core",
        "[[:digit:]]+-Core", // regular expression for "2-Core", "4-Core", etc.
        "Core",
        "Technologies, Inc",
        "\\,",
        "[[:digit:]]? COMPUTE CORES",
        "RADEON R[[:digit:]]",
        "[[:digit:]]\\w\\+[[:digit:]]\\w",
    ] {
        let re = Regex::new(trash).unwrap();
        _str = match re.find(&_str) {
            Some(part) => _str.replace(part.as_str(), ""),
            None => _str,
        };
    }

    if _str.contains('@') {
        _str = _str.split('@').next().unwrap().to_string();
    }

    _str.trim().to_string()
}

fn get_vendor(vendor_id: &str) -> String {
    String::from(match vendor_id {
        "GenuineIntel" => "Intel",
        "AuthenticAMD" => "AMD",
        _ => "",
    })
}

pub fn get_info() -> CPUInfo {
    let mut result = CPUInfo {
        model: String::from(""),
        max_freq: utils::converter::frequency_from_hz(0),
        cores: 0,
        threads: 0,
    };

    // parse /proc/cpuinfo
    let mut vendor = String::new();
    for line in fs::read_to_string("/proc/cpuinfo")
        .expect("NO /proc/cpuinfo FILE")
        .split('\n')
    {
        let mut line_content = line.split(":");
        let variable = line_content.next().unwrap().trim();
        let value = line_content.next().unwrap_or("").trim().to_string();
        if value.is_empty() {
            continue;
        }
        match variable.trim() {
            "model name" | "Hardware" => {
                result.model = extract_model_name(value.trim().to_string());
            }
            "vendor_id" => {
                vendor = get_vendor(value.as_str());
            }
            "cpu cores" => {
                result.cores = value.trim().parse::<u8>().unwrap();
            }
            "processor" => {
                result.threads = value.trim().parse::<u8>().unwrap() + 1_u8;
            }
            _ => {
                continue;
            }
        }
    }
    if !vendor.is_empty() {
        result.model = format!("{} {}", vendor, result.model);
    }

    // cores not presented in /proc/cpuinfo
    if result.cores == 0 && result.threads != 0 {
        result.cores = result.threads;
    }

    // get max_freq
    let max_freq_file_path = "/sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq";
    if fs::metadata(max_freq_file_path).is_ok() {
        result.max_freq = utils::converter::frequency_from_hz(extract_i64(
            fs::read_to_string(max_freq_file_path).unwrap().as_str(),
        ));
    }

    result
}
