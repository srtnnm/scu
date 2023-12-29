#![cfg(feature = "drives")]

use std::fs;
use std::path::PathBuf;

use crate::utils;

const SYSFS_BLOCKS: &str = "/sys/block";

#[derive(Debug, PartialEq)]
pub enum DriveTechnology {
    HDD,
    SSD,
}

#[derive(Debug)]
pub struct Drive {
    pub path: String,
    pub model: String,
    pub size: utils::converter::MemorySize,
    pub technology: DriveTechnology,
}

impl Drive {
    pub fn is_ssd(&self) -> bool {
        self.technology == DriveTechnology::SSD
    }
}

fn get_devtype(content: String) -> String {
    for line in content.split('\n') {
        if line.contains("DEVTYPE=") {
            return line.split("DEVTYPE=").nth(1).unwrap().to_string();
        }
    }

    String::from("")
}

pub fn scan_drives() -> Option<Vec<Drive>> {
    let mut result: Vec<Drive> = Vec::new();

    let sysblock = PathBuf::from(SYSFS_BLOCKS);
    if !sysblock.exists() || !sysblock.is_dir() {
        return None;
    }
    let readdir = fs::read_dir(sysblock);
    if readdir.is_err() {
        return None;
    }
    for block_device in readdir.unwrap() {
        let dev = PathBuf::from(block_device.unwrap().path());
        let device = dev.file_name().unwrap().to_str().unwrap().to_string();
        let mut model = device.clone();

        let device_data = dev.join("device"); // format!("/sys/block/{}/device", device);
        let device_uevent = dev.join("uevent"); //format!("/sys/block/{}/uevent", device);

        if device.starts_with("dm")
            || device.starts_with("loop")
            || device.starts_with("sr")
            || device.starts_with("zram")
            || device.contains("boot")
            || get_devtype(
                fs::read_to_string(device_uevent.clone())
                    .unwrap_or_else(|_| panic!("NO {} FILE", device_uevent.to_str().unwrap())),
            ) != *"disk"
        {
            continue;
        }

        let technology: DriveTechnology = match fs::read_to_string(dev.join("queue/rotational")) {
            Ok(content) => {
                if content.trim() == "1" {
                    DriveTechnology::HDD
                } else {
                    DriveTechnology::SSD
                }
            }
            Err(_) => DriveTechnology::HDD,
        };

        for model_name_file in ["model", "name"] {
            let model_file = device_data.join(model_name_file);
            if fs::metadata(model_file.clone()).is_ok() {
                model = fs::read_to_string(model_file)
                    .unwrap()
                    .replace('\n', "")
                    .trim()
                    .to_string();
                break;
            }
        }
        let size = utils::converter::MemorySize::from_blocks(
            fs::read_to_string(dev.join("size"))
                .unwrap()
                .replace('\n', "")
                .parse::<i64>()
                .unwrap(),
        );
        if size.mb == 0 {
            continue;
        }

        result.push(Drive {
            path: format!("/dev/{}", device),
            model: model.to_string(),
            size,
            technology,
        });
    }

    Some(result)
}
