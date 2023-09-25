use std::fs;
use std::path::Path;

pub enum BatteryStatus {
    Charging,
    Discharging
}

pub struct BatteryInfo {
    pub model: String,
    pub technology: String,
    pub capacity: u16
}

pub fn get_battery_info() -> Option<BatteryInfo> {
    let mut result: BatteryInfo = BatteryInfo {
        model: String::from("Unknown"),
        technology: String::from("Unknown"),
        capacity: 0_u16,
    };

    let mut battery_dir = String::from("");

    if Path::new("/sys/class/power_supply").exists() {
        let power_supply = fs::read_dir("/sys/class/power_supply");
        if let Ok(power_supply) = power_supply {
            for dir in power_supply {
                let dir = dir.unwrap().file_name().into_string().unwrap();
                if dir.starts_with("BAT") && dir.len() == 4 {
                    battery_dir = format!("/sys/class/power_supply/{}", dir);
                    break;
                }
            }
        }
    }
    if battery_dir.is_empty() { return None; }

    if Path::new(&battery_dir).exists() {
        result.model = match fs::read_to_string(format!("{}/model_name", battery_dir)) {
            Ok(content) => content.trim().to_string(),
            Err(_) => "Unknown".to_string(),
        };
        result.technology = match fs::read_to_string(format!("{}/technology", battery_dir)) {
            Ok(content) => content.trim().to_string(),
            Err(_) => "Unknown".to_string(),
        };
        result.capacity = match fs::read_to_string(format!("{}/capacity", battery_dir)) {
            Ok(content) => content.trim().to_string().parse::<u16>().unwrap_or(0_u16),
            Err(_) => 0_u16,
        };
    }

    if result.model == "Unknown" && result.technology == "Unknown" && result.capacity == 0_u16 {
        return None;
    }

    Some(result)
}
