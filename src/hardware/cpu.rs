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
    let re = Regex::new(r"\d+").unwrap();
    re.find(_str).unwrap().as_str().parse::<i64>().unwrap()
}

fn extract_model_name(mut _str: String) -> String {
    for trash in [
        "Intel",
        "Core",
        "AMD",
        "\\((TM|tm)\\)",
        "\\((R|r)\\)",
        "CPU",
        "Processor",
        "[Dual,Quad,Six,Eight]-Core",
        "[[:digit:]]+-Core", // regular expression for "2-Core", "4-Core", etc.
        "Technologies, Inc",
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
    let mut model: String = String::from("");
    let mut vendor: String = String::from("");
    let mut max_freq = utils::converter::frequency_from_hz(0);
    let mut cores: u8 = 0;
    let mut threads: u8 = 0;

    // parse /proc/cpuinfo
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
                model = extract_model_name(value.trim().to_string());
            }
            "vendor_id" => {
                vendor = get_vendor(value.as_str());
            }
            "cpu cores" => {
                cores = value.trim().parse::<u8>().unwrap();
            }
            "processor" => {
                threads = value.trim().parse::<u8>().unwrap() + 1_u8;
            }
            _ => {
                continue;
            }
        }
    }

    // get max_freq
    let max_freq_file_path = "/sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq";
    if fs::metadata(max_freq_file_path).is_ok() {
        max_freq = utils::converter::frequency_from_hz(extract_i64(
            fs::read_to_string(max_freq_file_path).unwrap().as_str(),
        ));
    }

    model = if !vendor.is_empty() {
        vendor + " "
    } else {
        "".to_string()
    } + model.as_str();

    CPUInfo {
        model,
        max_freq,
        cores,
        threads,
    }
}
