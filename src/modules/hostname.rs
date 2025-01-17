use super::Detection;

#[cfg(target_os = "macos")]
use crate::data::raw_models;

use libscu::software::hostname;

pub(super) fn fetch() -> Option<String> {
    hostname::fetch(
        #[cfg(target_os = "macos")]
        raw_models(),
    )
    .ok()
}

pub struct Hostname;

impl Detection for Hostname {
    type Result = String;
    const NAME: &'static str = "hostname";

    fn fetch(&self) -> std::io::Result<Self::Result> {
        hostname::fetch(
            #[cfg(target_os = "macos")]
            raw_models(),
        )
    }
}
