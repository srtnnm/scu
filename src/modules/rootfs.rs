use libscu::software::mounts;

pub(crate) fn get_rootfs_fstype() -> Option<String> {
    // ADD DEBUGGING FUNCTIONAL
    mounts::get_mountpoint_fstype("/").ok()
}
