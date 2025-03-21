use crate::config::force_versions;

use super::Detection;

use libscu::software::graphics;

pub struct WM;

impl Detection for WM {
    type Result = graphics::WindowManager;
    const NAME: &'static str = "wm";

    fn fetch(&self) -> std::io::Result<Self::Result> {
        graphics::fetch_window_manager(force_versions())
    }
}
