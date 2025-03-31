use super::Detection;

use libscu::{software::graphics, util::data::DesktopEnvironment};

pub struct DE;

impl Detection for DE {
    type Result = DesktopEnvironment;
    const NAME: &'static str = "de";

    fn fetch(&self) -> std::io::Result<Self::Result> {
        graphics::fetch_environment()
    }
}
