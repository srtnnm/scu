use super::Detection;

use libscu::{software::graphics, util::data::DesktopEnvironment};

pub(super) fn fetch() -> Option<DesktopEnvironment> {
    graphics::fetch_environment().ok()
}

pub struct DE;

impl Detection for DE {
    type Result = DesktopEnvironment;
    const NAME: &'static str = "de";

    fn fetch(&self) -> std::io::Result<Self::Result> {
        graphics::fetch_environment()
    }
}
