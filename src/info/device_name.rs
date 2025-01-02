use crate::data::raw_models;

use libscu::hardware::device;

pub(super) fn fetch() -> Option<String> {
    device::fetch_model(raw_models())
}
