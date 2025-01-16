use libscu::software::kernel;

pub(super) fn fetch() -> Option<String> {
    kernel::fetch_arch().ok()
}
