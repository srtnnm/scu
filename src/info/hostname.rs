use crate::data::raw_models;

use libscu::software::hostname;

pub(super) fn fetch() -> Option<String> {
    hostname::fetch(raw_models())
}