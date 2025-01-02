pub mod distro_colors;
pub mod table;

use std::sync::atomic::{AtomicBool, Ordering};

static RAW_MODELS: AtomicBool = AtomicBool::new(false);

pub(crate) fn raw_models() -> bool {
    RAW_MODELS.load(Ordering::Relaxed)
}
pub(crate) fn set_raw_models(raw_models: bool) {
    RAW_MODELS.store(raw_models, Ordering::Relaxed);
}
