use crate::modules::Module;

pub struct Table {
    pub title: String,
    pub modules: Vec<Module>,
}
impl Table {
    fn new(title: &str, modules: Vec<Module>) -> Self {
        Self {
            title: title.to_string(),
            modules,
        }
    }
}

pub struct TableConfig {
    pub tables: Vec<Table>,
}

impl Default for TableConfig {
    fn default() -> Self {
        use Module::*;
        Self {
            tables: vec![
                Table::new(
                    "System",
                    vec![
                        Hostname, Username, OS, Device, Init, Terminal, Shell, Uptime,
                    ],
                ),
                Table::new("Processor", vec![CPU]),
                Table::new("Memory", vec![Memory]),
                Table::new("Graphics", vec![GPU, DisplayServer, DE, WM, Brightness]),
                Table::new("Packages", vec![Packages]),
                Table::new("Disks", vec![Disks]),
                Table::new("Batteries", vec![Battery]),
            ],
        }
    }
}
