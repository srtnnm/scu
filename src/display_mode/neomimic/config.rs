use crate::modules::Module;

use super::logo::Logo;

pub struct NeomimicConfig {
    pub logo: Logo,
    pub modules: Vec<Module>,
}

impl Default for NeomimicConfig {
    fn default() -> Self {
        use Module::*;

        Self {
            logo: Logo::default(),
            modules: vec![
                Header, Separator, OS, Device, Kernel, Uptime, Packages, Shell, WM, Terminal, CPU,
                GPU, Memory,
            ],
        }
    }
}
