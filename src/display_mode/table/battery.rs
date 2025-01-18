use super::gen_table::GenerateTableEntries;

use crate::{data::table::Table, modules::Battery};

pub fn to_table() -> Option<Table> {
    let mut result = Table::new("Batteries");

    Battery.gen_entries(&mut result).ok()?;

    Some(result)
}
