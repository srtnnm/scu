use libscu::software::packages;

pub(super) fn fetch_package_managers() -> Vec<packages::PackageManager> {
    packages::fetch_all_managers().unwrap_or_default()
}
