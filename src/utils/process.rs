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
        pid,
        ppid: 0,
        cmdline: String::from(""),
        command: String::from(""),
    };

    if !Path::new("/proc").exists() || !Path::new(format!("/proc/{}", pid).as_str()).exists() {
        return Err(ProcessError::ProcessNotFound);
    }

    // parent process id
    let content = match fs::read_to_string(format!("/proc/{}/status", pid)) {
        Ok(content) => content,
        Err(_) => {
            return Err(ProcessError::StatusNotFound);
        }
    };
    let status_content = content.split('\n');
    for line in status_content {
        if line.contains(':') {
            let value = line.split(':').nth(1).unwrap().trim().to_string();
            match line
                .split(':')
                .next()
                .unwrap()
                .to_ascii_lowercase()
                .as_str()
            {
                "name" => {
                    result.command = value;
                }
                "ppid" => {
                    result.ppid = value.parse::<u32>().unwrap();
                }
                _ => {
                    continue;
                }
            }
        }
    }

    result.cmdline = std::fs::read_to_string(format!("/proc/{}/cmdline", pid)).unwrap();
    if result.cmdline.is_empty() {
        return Err(ProcessError::CmdlineIsEmpty);
    };

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
