use super::Detection;

use libscu::hardware::ram;

pub struct Memory;

impl Detection for Memory {
    type Result = ram::RAMInfo;
    const NAME: &'static str = "memory";

    fn fetch(&self) -> std::io::Result<Self::Result> {
        ram::fetch_info()
    }
}
