#![cfg(target_os = "linux")]

use crate::data::table::Table;

#[derive(Default)]
pub struct Battery {
    pub model: String,
    pub technology: Option<String>,
    pub level: u16,
    pub status: Option<String>,
}

impl Battery {
    pub fn to_print(&self) -> Table {
        let mut result = Table::new("Battery");

        result.add("Model", &self.model);
        if let Some(technology) = self.technology.clone() {
            result.add("Technology", &technology)
        }
        result.add("Level", format!("{}%", self.level).as_str());
        if let Some(status) = self.status.clone() {
            result.add("Status", &status);
        }

        result
    }
}