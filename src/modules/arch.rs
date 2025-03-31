use super::Detection;

use libscu::software::kernel;

pub struct Arch;

impl Detection for Arch {
    type Result = String;
    const NAME: &'static str = "arch";

    fn fetch(&self) -> std::io::Result<Self::Result> {
        kernel::fetch_arch()
    }
}
