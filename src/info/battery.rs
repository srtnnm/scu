#![cfg(any(target_os = "linux"))]

use crate::data::table::*;
use crate::utils::colorize::colorize_by_num;

use libscu::hardware::battery;

pub fn collect(simplify: bool) -> Table {
    let mut result = Table::new("Battery");

    if let Some(first_battery) = battery::fetch_all().first() {
        result.add("Model", &first_battery.model);

        if let Some(technology) = first_battery.technology.clone() {
            result.add("Technology", &technology);
        }

        result.add(
            "Capacity",
            format!(
                "{}",
                if !simplify {
                    colorize_by_num(
                        format!("{}%", first_battery.capacity).as_str(),
                        first_battery.capacity,
                        100,
                        true,
                    )
                } else {
                    format!("{}%", first_battery.capacity)
                }
            )
            .as_str(),
        );

        if let Some(battery_status) = first_battery.status.clone() {
            result.add("Status", &battery_status);
        }
    }

    result
}
