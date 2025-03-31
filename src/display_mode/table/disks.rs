use crate::{data::table::Table, modules::Disks};

use super::gen_table::GenerateTableEntries;

pub(super) fn to_table() -> Option<Table> {
    let mut result = Table::new("Disks");

    Disks.gen_entries(&mut result).ok()?;

    Some(result)
}
