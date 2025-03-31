use super::Detection;

use crate::config::raw_models;

use libscu::hardware::gpu;

pub struct GPU;

impl Detection for GPU {
    type Result = Vec<gpu::GPUInfo>;
    const NAME: &'static str = "gpu";

    fn fetch(&self) -> std::io::Result<Self::Result> {
        gpu::fetch_all(raw_models())
    }
}
