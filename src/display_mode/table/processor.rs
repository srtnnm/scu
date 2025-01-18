use super::gen_table::GenerateTableEntries;

use crate::{data::table::Table, modules::CPU};

pub fn to_table(disable_color: bool, multicpu: bool) -> Option<Table> {
    let mut result = Table::new("Processor");

    CPU.gen_entries(&mut result).ok()?;

    Some(result)
}
