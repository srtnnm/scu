use super::Detection;

use libscu::software::init;

pub(super) fn fetch() -> Option<init::InitSystem> {
    init::fetch_info().ok()
}

pub struct Init;

impl Detection for Init {
    type Result = init::InitSystem;
    const NAME: &'static str = "init";

    fn fetch() -> std::io::Result<Self::Result> {
        init::fetch_info()
    }
}
