use super::{DataRow, ModuleTrait};

use crate::info::get_vec;

pub struct Battery;

impl ModuleTrait for Battery {
    const NAME: &'static str = "battery";

    fn run(info: &crate::info::SystemInformation) -> std::io::Result<usize> {
        let batteries = get_vec("batteries", &info.batteries)?;

        let len = batteries
            .iter()
            .map(|battery| {
                DataRow::info(
                    &format!("Battery ({})", battery.model),
                    &format!("{}% [{}]", battery.level, battery.status.to_str()),
                )
            })
            .max()
            .unwrap_or_default();

        Ok(len)
    }
}
