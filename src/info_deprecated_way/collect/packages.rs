use crate::info::r#struct::Packages;

use libscu::software::packages;

pub fn collect() -> Packages {
    let mut result = Packages::default();

    for pm in packages::fetch_all_managers() {
        result.managers.push(pm);
    }

    result
}
