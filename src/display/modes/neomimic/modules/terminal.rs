use super::{DataRow, ModuleTrait};

use crate::info::get_option;

pub struct Terminal;

impl ModuleTrait for Terminal {
    const NAME: &'static str = "terminal";

    fn run(info: &crate::info::SystemInformation) -> std::io::Result<usize> {
        let terminal = get_option("terminal", &info.terminal)?;

        let mut terminal_str = terminal.name;
        if let Some(ref version) = terminal.version {
            terminal_str.push(' ');
            terminal_str.push_str(version);
        }

        Ok(DataRow::info("Terminal", &terminal_str))
    }
}
