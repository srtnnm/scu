use libscu::{software::time, types::Time};

pub(super) fn fetch() -> Option<Time> {
    time::fetch_uptime()
}
