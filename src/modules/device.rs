use super::Detection;

use crate::config::raw_models;

use libscu::hardware::device;

pub struct Device;

impl Detection for Device {
    type Result = String;
    const NAME: &'static str = "device";

    fn fetch(&self) -> std::io::Result<Self::Result> {
        device::fetch_model(raw_models())
    }
}
