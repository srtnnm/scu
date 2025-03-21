use super::Detection;

use libscu::software::graphics;

pub struct DisplayServer;

impl Detection for DisplayServer {
    type Result = graphics::DisplayServer;
    const NAME: &'static str = "displayserver";

    fn fetch(&self) -> std::io::Result<Self::Result> {
        graphics::fetch_display_server()
    }
}
