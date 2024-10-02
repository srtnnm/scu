#![cfg(any(target_os = "linux"))]

use crate::info::r#struct::Battery;

use libscu::hardware::battery;

pub fn collect() -> Battery {
    let mut result = Battery::default();

    let interfaces = battery::fetch_interfaces();

    if let Some(first_battery) = interfaces
        .first()
        .and_then(|interface| battery::collect_info(&interface))
    {
        result.model = first_battery.model.clone();
        result.technology = Some(first_battery.technology.to_str().to_string());
        result.level = first_battery.capacity as u16;
        result.status = Some(first_battery.status.to_str().to_string());
    }

    result
}
