use libscu::software::graphics;

pub(super) fn fetch(force_versions: bool) -> Option<graphics::WindowManager> {
    graphics::fetch_window_manager(force_versions)
}