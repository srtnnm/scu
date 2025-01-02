use libscu::software::terminal;

pub(super) fn fetch(force_versions: bool) -> Option<terminal::TerminalInfo> {
    terminal::fetch_info(force_versions).ok()
}
