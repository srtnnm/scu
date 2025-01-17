use super::Detection;

use libscu::software::packages;

pub(super) fn fetch_package_managers() -> Vec<packages::PackageManager> {
    packages::fetch_all_managers().unwrap_or_default()
}

pub struct Packages;

impl Detection for Packages {
    type Result = Vec<packages::PackageManager>;
    const NAME: &'static str = "packages";

    fn fetch(&self) -> std::io::Result<Self::Result> {
        packages::fetch_all_managers()
    }
}
