use super::Detection;

use libscu::software::mounts;

pub(crate) fn get_rootfs_fstype() -> Option<String> {
    // ADD DEBUGGING FUNCTIONAL
    mounts::get_mountpoint_fstype("/").ok()
}

pub struct RootFS;

impl Detection for RootFS {
    type Result = String;
    const NAME: &'static str = "rootfs";

    fn fetch(&self) -> std::io::Result<Self::Result> {
        mounts::get_mountpoint_fstype("/")
    }
}
