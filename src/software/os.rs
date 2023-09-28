use crate::utils;
use std::env;
use std::fs;
use std::process::Command;

pub fn get_name() -> String {
    let mut result: String = String::from("Unknown");

    if fs::metadata("/etc/os-release").is_ok() {
        for line in fs::read_to_string("/etc/os-release").unwrap().split('\n') {
            if line.starts_with("NAME=") {
                result = line.split("NAME=").nth(1).unwrap().replace('\"', "");
            }
        }
    } else if fs::metadata("/system/app").is_ok() && fs::metadata("/system/priv-app").is_ok() {
        result = String::from("Android");
        let version = String::from_utf8(
            Command::new("getprop")
                .args(["ro.build.version.release"])
                .output()
                .unwrap()
                .stdout,
        )
        .unwrap();
        result += (String::from(" ") + &version).as_str();
    }

    result
}

pub fn get_uptime() -> Option<utils::converter::Time> {
    let time = fs::read_to_string("/proc/uptime");
    if let Ok(time) = time {
        let time = time.split('.').next().unwrap().parse::<i32>().unwrap();

        return Some(utils::converter::time_from_seconds(time));
    }

    None
}

pub fn get_hostname() -> String {
    if fs::metadata("/etc/hostname").is_ok() {
        return fs::read_to_string("/etc/hostname")
            .unwrap()
            .replace('\n', "");
    } else {
        let hostname = env::var("HOSTNAME");
        if hostname.is_ok() {
            return hostname.unwrap();
        }
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
