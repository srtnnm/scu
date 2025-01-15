use super::{DataRow, ModuleTrait};

use crate::modules::get_option;

pub struct DE;

impl ModuleTrait for DE {
    const NAME: &'static str = "de";

    fn run(info: &crate::modules::SystemInformation) -> std::io::Result<usize> {
        let de = get_option("desktop environment", &info.desktop_environment)?;

        Ok(DataRow::info("DE", de.to_str()))
    }
}
