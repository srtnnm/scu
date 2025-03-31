#![allow(dead_code)]

#[derive(Clone, Debug)]
pub struct TableEntry {
    pub name: String,
    pub value: String,
    pub additional: Vec<TableEntry>,
}

impl TableEntry {
    pub fn new(name: &str, value: &str) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
            additional: Vec::new(),
        }
    }
    pub fn new_with_additional(name: &str, value: &str, additional: &[TableEntry]) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
            additional: additional.into_iter().map(Clone::clone).collect(),
        }
    }
}

#[derive(Clone)]
pub struct Table {
    pub title: String,
    pub entries: Vec<TableEntry>,
}

impl Table {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            entries: Vec::new(),
        }
    }
    pub fn add(&mut self, entry_name: &str, entry_value: &str) {
        self.entries.push(TableEntry::new(entry_name, entry_value))
    }
    pub fn add_with_additional(
        &mut self,
        entry_name: &str,
        entry_value: &str,
        additional: &[TableEntry],
    ) {
        self.entries.push(TableEntry::new_with_additional(
            entry_name,
            entry_value,
            additional,
        ))
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
