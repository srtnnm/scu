use crate::{display::modes::neomimic::row::DataRow, info::get_option};

use super::Module;

pub struct WM;

impl Module for WM {
    const NAME: &'static str = "wm";

    fn get(
        info: &crate::info::SystemInformation,
    ) -> std::io::Result<crate::display::modes::neomimic::row::DataRow> {
        let wm = get_option("window manager", &info.window_manager)?;
        let wm_name = get_option("window manager name", &wm.name)?;

        let mut wm_str = wm_name.to_string();
        if let Some(ref version) = wm.version {
            wm_str.push(' ');
            wm_str.push_str(version);
        }

        Ok(DataRow::info("WM", wm_str))
    }
}
