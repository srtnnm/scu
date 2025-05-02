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
    pub fn push_entry(&mut self, entry: TableEntry) {
        self.entries.push(entry)
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
