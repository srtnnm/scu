use libscu::software::os;

pub(super) fn fetch() -> Option<os::OSRelease> {
    os::fetch_release().ok()
}
