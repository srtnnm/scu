use super::Detection;

use libscu::software::terminal;

pub(super) fn fetch(force_versions: bool) -> Option<terminal::TerminalInfo> {
    terminal::fetch_info(force_versions).ok()
}
pub struct Terminal;

impl Detection for Terminal {
    type Result = terminal::TerminalInfo;
    const NAME: &'static str = "terminal";

    fn fetch() -> std::io::Result<Self::Result> {
        terminal::fetch_info(false) // TODO
    }
}
