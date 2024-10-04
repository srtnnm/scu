#![cfg(target_os = "linux")]

use crate::{
    data::table::{Table, TableEntry},
    info,
};

pub fn to_table(info: &info::SystemInformation) -> Option<Table> {
    if info.batteries.is_empty() {
        return None;
    }

    let mut result = Table::new("Batteries");

    for battery in &info.batteries {
        let additional = [
            TableEntry::new("Level", &format!("{}%", battery.level)),
            TableEntry::new("Status", &battery.status.to_str()),
            TableEntry::new("Technology", &battery.technology.to_string()),
        ]
        .to_vec();
        result.add_with_additional("Battery", &battery.model, additional);
    }

    Some(result)
}
