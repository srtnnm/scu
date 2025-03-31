use super::Detection;

use libscu::software::packages;

pub struct Packages;

impl Detection for Packages {
    type Result = Vec<packages::PackageManager>;
    const NAME: &'static str = "packages";

    fn fetch(&self) -> std::io::Result<Self::Result> {
        packages::fetch_all_managers()
    }
}
