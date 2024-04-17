use crate::info::r#struct::Packages;

use libscu::software::packages;

pub fn collect() -> Packages {
    let mut result = Packages::default();

    result.managers = packages::fetch_all_managers();

    result
}
