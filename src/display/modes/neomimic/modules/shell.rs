use crate::{display::modes::neomimic::row::DataRow, info::get_option};

use super::ModuleTrait;

pub struct Shell;

impl ModuleTrait for Shell {
    const NAME: &'static str = "shell";

    fn run(info: &crate::info::SystemInformation) -> std::io::Result<usize> {
        let shell = get_option("shell", &info.shell)?;

        let mut shell_str = shell.name;
        if let Some(ref version) = shell.version {
            shell_str.push(' ');
            shell_str.push_str(version);
        }

        Ok(DataRow::info("Shell", &shell_str))
    }
}
