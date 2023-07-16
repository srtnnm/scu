use crate::utils;
use regex::Regex;
use std::fs;

pub struct CPUInfo {
    pub model: String,
    pub vendor: String,
    pub max_freq: utils::converter::Frequency,
    pub cores: i8,
    pub threads: i8,
}

// additional info are in /sys/devices/system/cpu
// max frequency -> /sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq

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

pub fn get_info() -> CPUInfo {
    let mut model: String = String::from("");
    let mut vendor: String = String::from("");
    let mut max_freq = utils::converter::frequency_from_hz(0);
    let mut cores: i8 = 0;
    let mut threads: i8 = 0;

    // parse /proc/cpuinfo
    for line in fs::read_to_string("/proc/cpuinfo")
        .expect("NO /proc/cpuinfo FILE")
        .split('\n')
    {
        if line.contains("model name\t:") && model.is_empty() {
            model = String::from(line.split(": ").nth(1).unwrap());
        } else if line.contains("vendor_id\t:") && vendor.is_empty() {
            vendor = String::from(match line.split(": ").nth(1).unwrap() {
                "GenuineIntel" => "Intel",
                "AuthenticAMD" => "AMD",
                _ => "Unknown",
            });
        } else if line.contains("cpu cores\t:") && cores == 0 {
            cores = line.split(": ").nth(1).unwrap().parse::<i8>().unwrap();
        } else if line.contains("processor\t:") {
            threads = line.split(": ").nth(1).unwrap().parse::<i8>().unwrap() + 1_i8;
        } else {
            continue;
        }
    }

    // get max_freq
    let max_freq_file_path = "/sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq";
    if fs::metadata(max_freq_file_path).is_ok() {
        max_freq = utils::converter::frequency_from_hz(extract_i64(
            fs::read_to_string(max_freq_file_path).unwrap().as_str(),
        ));
    }

    model = extract_model_name(model);

    CPUInfo {
        model,
        vendor,
        max_freq,
        cores,
        threads,
    }
}
