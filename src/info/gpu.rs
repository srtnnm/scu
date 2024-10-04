use crate::data::raw_models;

use libscu::hardware::gpu;

pub(super)fn fetch_gpus() -> Vec<gpu::GPUInfo> {
    match gpu::fetch_all(raw_models()) {
        Ok(gpus) => gpus,
        Err(err) => {
            eprintln!("failed to get information about GPUs: {err:?}");
            Vec::default()
        }
    }
}