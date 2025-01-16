use super::Detection;

use libscu::software::graphics;

pub(super) fn fetch() -> Option<graphics::DisplayServer> {
    graphics::fetch_display_server().ok()
}

pub struct DisplayServer;

impl Detection for DisplayServer {
    type Result = graphics::DisplayServer;
    const NAME: &'static str = "displayserver";

    fn fetch() -> std::io::Result<Self::Result> {
        graphics::fetch_display_server()
    }
}
