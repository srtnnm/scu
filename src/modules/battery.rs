use super::Detection;

use libscu::hardware::battery;

pub(super) fn fetch_batteries_info() -> Vec<battery::BatteryInfo> {
    let Ok(battery_interfaces) = battery::fetch_interfaces() else {
        return Vec::default();
    };

    battery_interfaces
        .iter()
        .map(|interface| battery::collect_info(interface))
        .flatten()
        .collect::<Vec<battery::BatteryInfo>>()
}

pub struct Battery;

impl Detection for Battery {
    type Result = Vec<battery::BatteryInfo>;
    const NAME: &'static str = "battery";

    fn fetch() -> std::io::Result<Self::Result> {
        let battery_interfaces = battery::fetch_interfaces()?;

        Ok(battery_interfaces
            .iter()
            .map(|interface| battery::collect_info(interface))
            .flatten()
            .collect())
    }
}
