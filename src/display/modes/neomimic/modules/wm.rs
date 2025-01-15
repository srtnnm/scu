use crate::{display::modes::neomimic::row::DataRow, info::get_option};

use super::ModuleTrait;

pub struct WM;

impl ModuleTrait for WM {
    const NAME: &'static str = "wm";

    fn run(info: &crate::info::SystemInformation) -> std::io::Result<usize> {
        let wm = get_option("window manager", &info.window_manager)?;
        let wm_name = get_option("window manager name", &wm.name)?;

        let mut wm_str = wm_name.to_string();
        if let Some(ref version) = wm.version {
            wm_str.push(' ');
            wm_str.push_str(version);
        }

        Ok(DataRow::info("WM", &wm_str))
    }
}
