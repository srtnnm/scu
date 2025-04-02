use crate::config::enable_versions;

use super::Detection;

use libscu::software::shell;

pub struct Shell;

impl Detection for Shell {
    type Result = shell::Shell;
    const NAME: &'static str = "shell";

    fn fetch(&self) -> std::io::Result<Self::Result> {
        shell::fetch_info(enable_versions())
    }
}
