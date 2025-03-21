use super::Detection;

use libscu::software::init;

pub struct Init;

impl Detection for Init {
    type Result = init::InitSystem;
    const NAME: &'static str = "init";

    fn fetch(&self) -> std::io::Result<Self::Result> {
        init::fetch_info()
    }
}
