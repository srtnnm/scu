use regex::Regex;
use std::fs;
use std::path::Path;
use std::process;

pub struct Process {
    pub pid: u32,  // process id
    pub ppid: u32, // parent process id
    pub cmdline: String,
    pub command: String,
}

#[derive(Debug)]
pub enum ProcessError {
    ProcessNotFound,
    CmdlineIsEmpty,
    StatusNotFound,
}

pub fn get_pid() -> u32 {
    process::id()
}

pub fn get_ppid(pid: u32) -> Option<u32> {
    if pid > 1 {
        return Some(get_info(pid).unwrap().ppid);
    }

    None
}

pub fn get_info(pid: u32) -> Result<Process, ProcessError> {
    let mut result = Process {
        pid: pid,
        ppid: 0,
        cmdline: String::from(""),
        command: String::from(""),
    };

    if !fs::metadata(format!("/proc/{}", pid)).is_ok() {
        return Err(ProcessError::ProcessNotFound);
    }

    // parent process id
    let content = match std::fs::read_to_string(format!("/proc/{}/status", pid)) {
        Ok(content) => content,
        Err(_) => {
            return Err(ProcessError::StatusNotFound);
        }
    };
    let status_content = content.split('\n');
    for line in status_content {
        if line.contains("PPid") {
            let _regex = Regex::new(r"\d+").unwrap();
            result.ppid = match _regex.find(line).unwrap().as_str().parse::<i32>() {
                Ok(integer) => integer as u32,
                Err(_) => 0,
            };
        }
    }

    result.cmdline = std::fs::read_to_string(format!("/proc/{}/cmdline", pid)).unwrap();
    if result.cmdline.is_empty() {
        return Err(ProcessError::CmdlineIsEmpty);
    };

    result.command = std::fs::read_to_string(format!("/proc/{}/comm", pid))
        .unwrap()
        .replace('\n', "");

    Ok(result)
}

pub fn list_process() -> Vec<Process> {
    let mut result: Vec<Process> = Vec::new();

    let processes: Vec<u32> = std::fs::read_dir("/proc")
        .unwrap()
        .filter_map(|v| v.unwrap().file_name().to_str().unwrap().parse::<u32>().ok())
        .collect();

    for proc in processes {
        match get_info(proc) {
            Ok(info) => result.push(info),
            Err(_) => {
                continue;
            }
        }
    }

    result
}
