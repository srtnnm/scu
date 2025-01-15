use libscu::software::init;

pub(super) fn fetch() -> Option<init::InitSystem> {
    init::fetch_info().ok()
}
