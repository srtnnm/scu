use crate::{
    data::table::Table,
    modules::{
        Device, Hostname, Init, Kernel, Locale, RootFS, Shell, Terminal, Uptime, Username, OS,
    },
};

use super::gen_table::GenerateTableEntries;

pub fn to_table() -> Option<Table> {
    let mut result = Table::new("System");

    Hostname.gen_entries(&mut result).ok()?;
    Username.gen_entries(&mut result).ok()?;
    OS.gen_entries(&mut result).ok()?;
    Device.gen_entries(&mut result).ok()?;
    Kernel.gen_entries(&mut result).ok()?;
    Init.gen_entries(&mut result).ok()?;
    Terminal.gen_entries(&mut result).ok()?;
    Shell.gen_entries(&mut result).ok()?;
    Uptime.gen_entries(&mut result).ok()?;
    RootFS.gen_entries(&mut result).ok()?;
    Locale.gen_entries(&mut result).ok()?;

    Some(result)
}
