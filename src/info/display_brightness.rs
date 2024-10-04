use libscu::hardware::display;

pub(super) fn fetch() -> Option<display::Brightness> {
    display::fetch_brightness()
}