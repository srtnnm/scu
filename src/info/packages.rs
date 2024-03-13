use crate::data::table::*;

use libscu::software::packages;

pub fn collect() -> Table {
    let mut result = Table::new("Packages");

    for manager in packages::fetch_all_managers() {
        result.add(manager.name, &manager.number_of_packages.to_string());
    }

    result
}
