use super::gen_table::GenerateTableEntries;

use crate::{
    data::table::Table,
    modules::{Brightness, DisplayServer, DE, GPU, WM},
};

pub fn to_table() -> Option<Table> {
    let mut result = Table::new("Graphics");

    GPU.gen_entries(&mut result).ok()?;
    DisplayServer.gen_entries(&mut result).ok()?;
    DE.gen_entries(&mut result).ok()?;
    WM.gen_entries(&mut result).ok()?;
    Brightness.gen_entries(&mut result).ok()?;

    Some(result)
}
