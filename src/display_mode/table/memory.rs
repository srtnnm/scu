use crate::{data::table::Table, modules::Memory};

use super::gen_table::GenerateTableEntries;

pub fn to_table(disable_color: bool) -> Option<Table> {
    let mut result = Table::new("Memory");

    Memory.gen_entries(&mut result).ok()?;

    Some(result)
}
