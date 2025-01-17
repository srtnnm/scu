use super::Detection;

use crate::data::raw_models;

use libscu::hardware::gpu;

pub(super) fn fetch_gpus() -> Vec<gpu::GPUInfo> {
    match gpu::fetch_all(raw_models()) {
        Ok(gpus) => gpus,
        Err(err) => {
            eprintln!("failed to get information about GPUs: {err:?}");
            Vec::default()
        }
    }
}

pub struct GPU;

impl Detection for GPU {
    type Result = Vec<gpu::GPUInfo>;
    const NAME: &'static str = "gpu";

    fn fetch(&self) -> std::io::Result<Self::Result> {
        Ok(gpu::fetch_all(raw_models())?)
    }
}
