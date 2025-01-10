use libscu::hardware::battery;

pub(super) fn fetch_batteries_info() -> Vec<battery::BatteryInfo> {
    let battery_interfaces = battery::fetch_interfaces();

    battery_interfaces
        .iter()
        .map(|interface| battery::collect_info(interface))
        .flatten()
        .collect::<Vec<battery::BatteryInfo>>()
}
