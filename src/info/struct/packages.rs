use crate::data::table::Table;

use libscu::software::packages::PackageManager;

#[derive(Default)]
pub struct Packages {
    pub managers: Vec<PackageManager>,
}

impl Packages {
    pub fn to_print(&self) -> Table {
        let mut result = Table::new("Packages");

        for manager in self.managers.iter() {
            result.add(
                manager.name,
                manager.number_of_packages.to_string().as_str(),
            )
        }

        result
    }
}
