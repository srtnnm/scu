#[cfg(target_os = "macos")]
use crate::data::raw_models;

use libscu::software::hostname;

pub(super) fn fetch() -> Option<String> {
    hostname::fetch(
        #[cfg(target_os = "macos")]
        raw_models(),
    )
}
