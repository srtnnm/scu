use crate::info::get_vec;

use super::{DataRow, ModuleTrait};

pub struct GPU;

impl ModuleTrait for GPU {
    const NAME: &'static str = "gpu";

    fn run(info: &crate::info::SystemInformation) -> std::io::Result<usize> {
        let gpus = get_vec("gpus", &info.gpus)?;

        let len = gpus
            .iter()
            .map(|gpu| {
                DataRow::info(
                    "GPU",
                    &format!(
                        "{vendor} {model}",
                        vendor = gpu.vendor.to_string(),
                        model = gpu.model
                    ),
                )
            })
            .max()
            .unwrap_or_default();

        Ok(len)
    }
}
