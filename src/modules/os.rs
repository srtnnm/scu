use super::Detection;

use libscu::software::os;

pub struct OS;

impl Detection for OS {
    type Result = os::OSRelease;
    const NAME: &'static str = "os";

    fn fetch(&self) -> std::io::Result<Self::Result> {
        os::fetch_release()
    }
}
