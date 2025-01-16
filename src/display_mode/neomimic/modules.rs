use super::display::{Display, Header, Separator};

use crate::modules::*;

pub enum Module {
    Battery,
    CPU,
    DE,
    GPU,
    Header,
    Device,
    Init,
    Kernel,
    Locale,
    Memory,
    OS,
    Packages,
    Separator,
    Shell,
    Terminal,
    Uptime,
    WM,
}

impl Module {
    pub fn run(&self) -> std::io::Result<usize> {
        match self {
            Self::Battery => Battery::run(),
            Self::CPU => CPU::run(),
            Self::DE => DE::run(),
            Self::GPU => GPU::run(),
            Self::Header => Header::run(),
            Self::Device => Device::run(),
            // Self::Init => Init::run(),
            Self::Kernel => Kernel::run(),
            Self::Locale => Locale::run(),
            Self::Memory => Memory::run(),
            Self::OS => OS::run(),
            Self::Packages => Packages::run(),
            Self::Separator => Separator::run(),
            Self::Shell => Shell::run(),
            Self::Terminal => Terminal::run(),
            Self::Uptime => Uptime::run(),
            Self::WM => WM::run(),
            _ => Ok(0), //todo Separator, Header, Init
        }
    }
}

// TODO: show possible errors for debugging
pub fn run_module(module: &Module) -> Option<usize> {
    module.run().ok()
}
