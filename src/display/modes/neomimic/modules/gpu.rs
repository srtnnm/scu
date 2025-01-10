use crate::{
    display::modes::neomimic::row::DataRow,
    info::{get_option, get_vec},
};

use super::Module;

pub struct GPU;

impl Module for GPU {
    const NAME: &'static str = "gpu";

    fn get(
        info: &crate::info::SystemInformation,
    ) -> std::io::Result<crate::display::modes::neomimic::row::DataRow> {
        let gpus = get_vec("gpus", &info.gpus)?;

        let first_gpu = get_option("first gpu", &gpus.iter().next())?;

        let first_gpu_str = format!(
            "{vendor} {model}",
            vendor = first_gpu.vendor.to_string(),
            model = first_gpu.model
        );

        Ok(DataRow::info("GPU", first_gpu_str))
    }
}
