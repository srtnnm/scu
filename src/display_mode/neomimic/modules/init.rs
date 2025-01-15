use super::{DataRow, ModuleTrait};

use crate::modules::get_option;

pub struct Init;

impl ModuleTrait for Init {
    const NAME: &'static str = "init";

    fn run(info: &crate::modules::SystemInformation) -> std::io::Result<usize> {
        let init = get_option("init", &info.init_system)?;

        Ok(DataRow::info(
            "Init",
            &format!(
                "{} ({} services)",
                init.name,
                init.number_of_services.unwrap_or_default()
            ),
        ))
    }
}
