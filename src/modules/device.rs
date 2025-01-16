use super::Detection;

use crate::data::raw_models;

use libscu::hardware::device;

pub(super) fn fetch() -> Option<String> {
    device::fetch_model(raw_models()).ok()
}

pub struct Device;

impl Detection for Device {
    type Result = String;
    const NAME: &'static str = "device";

    fn fetch() -> std::io::Result<Self::Result> {
        device::fetch_model(raw_models())
    }
}
