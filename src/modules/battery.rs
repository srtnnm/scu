use super::Detection;

use libscu::hardware::battery;

pub struct Battery;

impl Detection for Battery {
    type Result = Vec<battery::BatteryInfo>;
    const NAME: &'static str = "battery";

    fn fetch(&self) -> std::io::Result<Self::Result> {
        let battery_interfaces = battery::fetch_interfaces()?;

        Ok(battery_interfaces
            .iter()
            .map(|interface| battery::collect_info(interface))
            .flatten()
            .collect())
    }
}
