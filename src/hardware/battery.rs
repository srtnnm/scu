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

    if Path::new("/sys/class/power_supply/BAT0").exists() {
        result.model = match fs::read_to_string("/sys/class/power_supply/BAT0/model_name") {
            Ok(content) => content.trim().to_string(),
            Err(_) => "Unknown".to_string(),
        };
        result.technology = match fs::read_to_string("/sys/class/power_supply/BAT0/technology") {
            Ok(content) => content.trim().to_string(),
            Err(_) => "Unknown".to_string(),
        };
        result.capacity = match fs::read_to_string("/sys/class/power_supply/BAT0/capacity") {
            Ok(content) => match content.trim().to_string().parse::<u16>() {
                Ok(integer) => integer,
                Err(_) => 0_u16,
            },
            Err(_) => 0_u16,
        };
    }

    if result.model == "Unknown" && result.technology == "Unknown" && result.capacity == 0_u16 {
        return None;
    }

    Some(result)
}
