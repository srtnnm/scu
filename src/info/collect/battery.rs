#![cfg(any(target_os = "linux"))]

use crate::info::r#struct::Battery;

use libscu::hardware::battery;

pub fn collect() -> Battery {
    let mut result = Battery::default();

    if let Some(first_battery) = battery::fetch_all().first() {
        result.model = first_battery.model.clone();
        result.technology = first_battery.technology.clone();
        result.level = first_battery.capacity.clone();
        result.status = first_battery.status.clone();
    }

    result
}
