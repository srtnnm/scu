use super::Detection;

use libscu::software::kernel;

pub(super) fn fetch() -> Option<String> {
    kernel::fetch_arch().ok()
}

pub struct Arch;

impl Detection for Arch {
    type Result = String;
    const NAME: &'static str = "arch";

    fn fetch() -> std::io::Result<Self::Result> {
        kernel::fetch_arch()
    }
}
