use crate::{display::modes::neomimic::row::DataRow, info::get_option};

use super::ModuleTrait;

pub struct DE;

impl ModuleTrait for DE {
    const NAME: &'static str = "de";

    fn get(
        info: &crate::info::SystemInformation,
    ) -> std::io::Result<crate::display::modes::neomimic::row::DataRow> {
        let de = get_option("desktop environment", &info.desktop_environment)?;

        Ok(DataRow::info("DE", de.to_str()))
    }
}
