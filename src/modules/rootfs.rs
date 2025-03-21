use super::Detection;

use libscu::software::mounts;

pub struct RootFS;

impl Detection for RootFS {
    type Result = String;
    const NAME: &'static str = "rootfs";

    fn fetch(&self) -> std::io::Result<Self::Result> {
        mounts::get_mountpoint_fstype("/")
    }
}
