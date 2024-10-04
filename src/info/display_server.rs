#![cfg(target_os = "linux")]
use libscu::software::graphics;

pub(super) fn fetch() -> Option<graphics::DisplayServer> {
    graphics::fetch_display_server()
}
