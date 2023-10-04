use std::fs;

use crate::utils;

pub struct Drive {
    pub path: String,
    pub model: String,
    pub size: utils::converter::MemorySize,
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

    let block_devices = fs::read_dir("/sys/block");
    if block_devices.is_err() {
        return None;
    }
    let block_devices = block_devices.unwrap();
    for block_device in block_devices {
        let device = block_device.unwrap().file_name().into_string().unwrap();

        let device_data = format!("/sys/block/{}/device", device);
        let device_uevent = format!("/sys/block/{}/uevent", device);

        if device.starts_with("dm")
            || device.starts_with("loop")
            || device.starts_with("sr")
            || device.starts_with("zram")
            || device.contains("boot")
            || get_devtype(
                fs::read_to_string(device_uevent.clone())
                    .unwrap_or_else(|_| panic!("NO {} FILE", device_uevent)),
            ) != *"disk"
        {
            continue;
        }

        let model = if fs::metadata(format!("{}/model", device_data)).is_ok() {
            fs::read_to_string(format!("{}/model", device_data))
                .unwrap()
                .replace('\n', "")
                .trim()
                .to_string()
        } else {
            device.clone()
        };
        let size = utils::converter::memory_size_from_blocks(
            fs::read_to_string(format!("/sys/block/{}/size", device))
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
            model,
            size,
        });
    }

    Some(result)
}
