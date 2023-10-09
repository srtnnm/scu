use crate::utils;
use regex::Regex;
use std::fs;

pub struct CPUInfo {
    pub model: String,
    pub freq: utils::converter::Frequency,
    pub cores: u8,
    pub threads: u8,
    pub temperature: f32,
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
        "with",
        "Radeon",
        "Vega",
        "Mobile",
        "Series",
        "Graphics",
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
        model: String::from("Unknown"),
        freq: utils::converter::frequency_from_hz(0),
        cores: 0,
        threads: 0,
        temperature: 0.0,
    };

    // parse /proc/cpuinfo
    let mut vendor = String::new();
    for line in fs::read_to_string("/proc/cpuinfo")
        .expect("NO /proc/cpuinfo FILE")
        .split('\n')
    {
        let mut line_content = line.split(':');
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
            "cpu" => {
                if value.contains("POWER9") {
                    result.model = "POWER9".to_string();
                    vendor = "IBM".to_string();
                }
            }
            /*"processor" => {
                result.threads = value.trim().parse::<u8>().unwrap() + 1_u8;
            } maybe no more needed*/
            _ => {
                continue;
            }
        }
    }
    if !vendor.is_empty() {
        result.model = format!("{} {}", vendor, result.model).trim().to_string();
    }

    // get threads (all units)
    if fs::metadata("/sys/bus/cpu/devices").is_ok() {
        result.threads = fs::read_dir("/sys/bus/cpu/devices").unwrap().count() as u8;
    }

    // cores not presented in /proc/cpuinfo
    if result.cores == 0 && result.threads != 0 {
        result.cores = result.threads;
    }

    // get max_freq
    let cpu_freq_files_path = "/sys/devices/system/cpu/cpu0/cpufreq/";
    for file in ["bios_limit", "scaling_max_freq", "cpuinfo_max_freq"] {
        let file_path = format!("{}{}", cpu_freq_files_path, file);
        if fs::metadata(file_path.clone()).is_ok() {
            result.freq = utils::converter::frequency_from_hz(extract_i64(
                fs::read_to_string(file_path.clone()).unwrap().as_str(),
            ));
            break;
        }
    }

    // get temperature
    // sensor drivers
    // k10temp  - AMD
    // coretemp - Intel
    if fs::metadata("/sys/class/hwmon").is_ok() {
        for hwmon in fs::read_dir("/sys/class/hwmon").unwrap() {
            let hwmon = format!(
                "/sys/class/hwmon/{}",
                hwmon.unwrap().file_name().to_str().unwrap()
            );
            if fs::metadata(format!("{}/name", hwmon)).is_err()
                || fs::metadata(format!("{}/temp1_input", hwmon)).is_err()
            {
                continue;
            }
            if ["k10temp", "coretemp"].contains(
                &fs::read_to_string(format!("{}/name", hwmon))
                    .unwrap()
                    .as_str()
                    .trim(),
            ) {
                result.temperature = fs::read_to_string(format!("{}/temp1_input", hwmon))
                    .unwrap()
                    .trim()
                    .parse::<u32>()
                    .unwrap() as f32
                    / 1000.0;
                break;
            }
        }
    }

    result
}
