use crate::modules::Module;

#[derive(Debug)]
pub struct TableCategory {
    pub title: String,
    pub modules: Vec<Module>,
}
impl TableCategory {
    fn new(title: &str, modules: Vec<Module>) -> Self {
        Self {
            title: title.to_string(),
            modules,
        }
    }
}

#[derive(Debug)]
pub struct TableConfig {
    pub categories: Vec<TableCategory>,
}

impl Default for TableConfig {
    fn default() -> Self {
        use Module::*;
        Self {
            categories: vec![
                TableCategory::new(
                    "System",
                    vec![
                        Hostname, Username, OS, Device, Init, Terminal, Shell, Uptime,
                    ],
                ),
                TableCategory::new("Processor", vec![CPU]),
                TableCategory::new("Memory", vec![Memory]),
                TableCategory::new("Graphics", vec![GPU, DisplayServer, DE, WM, Brightness]),
                TableCategory::new("Packages", vec![Packages]),
                TableCategory::new("Disks", vec![Disks]),
                TableCategory::new("Batteries", vec![Battery]),
            ],
        }
    }
}
