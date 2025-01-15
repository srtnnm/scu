use libscu::{software::graphics, util::data::DesktopEnvironment};

pub(super) fn fetch() -> Option<DesktopEnvironment> {
    graphics::fetch_environment().ok()
}
