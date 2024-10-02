#![cfg(target_os = "linux")]

use crate::info::r#struct::Battery;

use libscu::hardware::battery;

pub fn collect() -> Option<Battery> {
    let interfaces = battery::fetch_interfaces();

    if let Some(first_battery) = interfaces
        .first()
        .and_then(|interface| battery::collect_info(interface))
    {
        let result = Battery {
            model: first_battery.model.trim().to_string(),
            technology: Some(first_battery.technology.to_str().to_string()),
            level: first_battery.capacity as u16,
            status: Some(first_battery.status.to_str().to_string()),
        };

        if result.model.is_empty() && result.level == 0 {
            return None;
        }

        Some(result)
    } else {
        None
    }
}
