#![cfg(feature = "ram")]

use regex::Regex;
use std::fs;

use crate::utils;

pub struct RAMInfo {
    pub total: utils::converter::MemorySize,
    pub used: utils::converter::MemorySize,

    pub swap_enabled: bool,
    pub swap_total: utils::converter::MemorySize,
    pub swap_used: utils::converter::MemorySize,
}

fn extract_i64(_str: &str) -> i64 {
    let re = Regex::new(r"\d+").unwrap();
    re.find(_str).unwrap().as_str().parse::<i64>().unwrap()
}

pub fn get_info() -> RAMInfo {
    // RAM
    let mut total: i64 = 0;
    let mut used: i64 = 0;

    // swap
    let mut swap_total: i64 = 0;
    let mut swap_free: i64 = 0;

    for line in fs::read_to_string("/proc/meminfo")
        .expect("NO /proc/meminfo FILE")
        .split('\n')
    {
        let mut line = line.split(':');
        let line_var = line.next().unwrap().trim();
        let line_val = line.next().unwrap_or("");
        if line_val.is_empty() {
            continue;
        }

        match line_var {
            "MemTotal" => {
                total = extract_i64(line_val);
                used = total;
            }
            "Shmem" => {
                used += extract_i64(line_val);
            }
            "MemFree" | "Buffers" | "Cached" | "SReclaimable" => {
                used -= extract_i64(line_val);
            }
            "SwapTotal" => {
                swap_total = extract_i64(line_val);
            }
            "SwapFree" => {
                swap_free = extract_i64(line_val);
            }
            _ => {
                continue;
            }
        }
    }

    RAMInfo {
        total: utils::converter::memory_size_from_kb(total),
        used: utils::converter::memory_size_from_kb(used),
        swap_enabled: swap_total != 0,
        swap_total: utils::converter::memory_size_from_kb(swap_total),
        swap_used: utils::converter::memory_size_from_kb(swap_total - swap_free),
    }
}
