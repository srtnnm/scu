use libscu::software::shell;

pub(super) fn fetch(force_versions: bool) -> Option<shell::Shell> {
    shell::fetch_info(force_versions)
}
