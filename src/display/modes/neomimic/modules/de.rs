use crate::{display::modes::neomimic::row::DataRow, info::get_option};

use super::ModuleTrait;

pub struct DE;

impl ModuleTrait for DE {
    const NAME: &'static str = "de";

    fn run(info: &crate::info::SystemInformation) -> std::io::Result<usize> {
        let de = get_option("desktop environment", &info.desktop_environment)?;

        Ok(DataRow::info("DE", de.to_str()))
    }
}
