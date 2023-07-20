use crate::utils;
use std::env;
use std::fs;

pub fn get_name() -> String {
    let mut result: String = String::from("Unknown");

    if fs::metadata("/etc/os-release").is_err() {
        return result;
    }

    for line in fs::read_to_string("/etc/os-release").unwrap().split('\n') {
        if line.contains("NAME") {
            result = line.split("NAME=").nth(1).unwrap().replace('\"', "");
        }
    }

    result
}

pub fn get_uptime() -> utils::converter::Time {
    let time: i32 = fs::read_to_string("/proc/uptime")
        .unwrap()
        .split('.')
        .next()
        .unwrap()
        .parse::<i32>()
        .unwrap();

    utils::converter::time_from_seconds(time)
}

pub fn get_hostname() -> String {
    if fs::metadata("/etc/hostname").is_ok() {
        return fs::read_to_string("/etc/hostname")
            .unwrap()
            .replace('\n', "");
    }

    String::from("Unknown")
}

pub fn get_shell() -> String {
    let mut result = String::from("");
    let mut ppid = utils::process::get_ppid(utils::process::get_pid()).unwrap();
    while ppid > 1 {
        let command = utils::process::get_info(ppid).unwrap().command;
        if ["bash", "fish", "tcsh", "zsh", "dash"].contains(&command.as_str()) {
            result = command;
            break;
        }
        ppid = utils::process::get_ppid(ppid).unwrap();
    }
    result
}
