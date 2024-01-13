#[derive(Clone, Debug)]
pub struct TableEntry {
    pub name: String,
    pub value: String,
    pub additional: Vec<Box<TableEntry>>,
}

impl TableEntry {
    pub fn new(name: &str, value: &str) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
            additional: Vec::new(),
        }
    }
    pub fn new_with_additional(name: &str, value: &str, additional: Vec<TableEntry>) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
            additional: additional
                .into_iter()
                .map(|e| Box::new(e.clone()))
                .collect(),
        }
    }
}

#[derive(Clone)]
pub struct Table {
    pub title: String,
    pub entries: Vec<TableEntry>,
}

impl Table {
    pub fn new() -> Self {
        Self {
            title: String::new(),
            entries: Vec::new(),
        }
    }
    pub fn add(&mut self, entry_name: &str, entry_value: &str) {
        self.entries.push(TableEntry::new(entry_name, entry_value))
    }
    pub fn add_with_additional<'a>(
        &mut self,
        entry_name: &str,
        entry_value: &str,
        additional: Vec<TableEntry>,
    ) {
        self.entries.push(TableEntry::new_with_additional(
            entry_name,
            entry_value,
            additional,
        ))
    }
    pub fn set_name(&mut self, name: &str) {
        self.title = name.to_string()
    }
    pub fn clear(&mut self) {
        self.title.clear();
        self.entries.clear();
    }
}
