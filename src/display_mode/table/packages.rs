use crate::{data::table::Table, modules::Packages};

use super::gen_table::GenerateTableEntries;

pub(super) fn to_table() -> Option<Table> {
    let mut result = Table::new("Packages");

    Packages.gen_entries(&mut result).ok()?;

    Some(result)
}
