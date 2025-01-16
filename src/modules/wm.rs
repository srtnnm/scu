use super::Detection;

use libscu::software::graphics;

pub(super) fn fetch(force_versions: bool) -> Option<graphics::WindowManager> {
    graphics::fetch_window_manager(force_versions).ok()
}

pub struct WM;

impl Detection for WM {
    type Result = graphics::WindowManager;
    const NAME: &'static str = "wm";

    fn fetch() -> std::io::Result<Self::Result> {
        graphics::fetch_window_manager(false) // TODO
    }
}
