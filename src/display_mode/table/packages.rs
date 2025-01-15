use crate::{data::table::Table, modules};

pub(super) fn to_table(info: &modules::SystemInformation) -> Option<Table> {
    if info.packages.is_empty() {
        None
    } else {
        let mut result = Table::new("Packages");

        for manager in info.packages.iter() {
            result.add(
                manager.name,
                manager.number_of_packages.to_string().as_str(),
            )
        }

        Some(result)
    }
}
