use crate::config::enable_versions;

use super::Detection;

use libscu::software::terminal;

pub struct Terminal;

impl Detection for Terminal {
    type Result = terminal::TerminalInfo;
    const NAME: &'static str = "terminal";

    fn fetch(&self) -> std::io::Result<Self::Result> {
        terminal::fetch_info(enable_versions())
    }
}
