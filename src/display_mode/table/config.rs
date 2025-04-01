use crate::modules::Module;

pub struct Category {
    pub title: String,
    pub modules: Vec<Module>,
}
impl Category {
    fn new(title: &str, modules: Vec<Module>) -> Self {
        Self {
            title: title.to_string(),
            modules,
        }
    }
}

pub struct TableConfig {
    pub categories: Vec<Category>,
}

impl Default for TableConfig {
    fn default() -> Self {
        use Module::*;
        Self {
            categories: vec![
                Category::new(
                    "System",
                    vec![
                        Hostname, Username, OS, Device, Init, Terminal, Shell, Uptime,
                    ],
                ),
                Category::new("Processor", vec![CPU]),
                Category::new("Memory", vec![Memory]),
                Category::new("Graphics", vec![GPU, DisplayServer, DE, WM, Brightness]),
                Category::new("Packages", vec![Packages]),
                Category::new("Disks", vec![Disks]),
                Category::new("Batteries", vec![Battery]),
            ],
        }
    }
}
