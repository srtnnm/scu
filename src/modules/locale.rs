use libscu::software::locale;

pub(super) fn fetch() -> Option<String> {
    locale::fetch().ok()
}
