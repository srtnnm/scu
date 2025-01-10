use crate::{display::modes::neomimic::row::DataRow, info::get_option};

use super::Module;

pub struct Terminal;

impl Module for Terminal {
    const NAME: &'static str = "terminal";

    fn get(
        info: &crate::info::SystemInformation,
    ) -> std::io::Result<crate::display::modes::neomimic::row::DataRow> {
        let terminal = get_option("terminal", &info.terminal)?;

        let mut terminal_str = terminal.name;
        if let Some(ref version) = terminal.version {
            terminal_str.push(' ');
            terminal_str.push_str(version);
        }

        Ok(DataRow::info("Terminal", terminal_str))
    }
}
