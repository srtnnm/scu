use super::Detection;

use libscu::{software::time, types::Time};

pub(super) fn fetch() -> Option<Time> {
    time::fetch_uptime().ok()
}

pub struct Uptime;

impl Detection for Uptime {
    type Result = Time;
    const NAME: &'static str = "uptime";

    fn fetch(&self) -> std::io::Result<Self::Result> {
        time::fetch_uptime()
    }
}
