use super::Detection;

use libscu::software::shell;

pub(super) fn fetch(force_versions: bool) -> Option<shell::Shell> {
    shell::fetch_info(force_versions).ok()
}

pub struct Shell;

impl Detection for Shell {
    type Result = shell::Shell;
    const NAME: &'static str = "shell";

    fn fetch(&self) -> std::io::Result<Self::Result> {
        shell::fetch_info(false) // TODO
    }
}
