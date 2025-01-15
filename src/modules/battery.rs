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
