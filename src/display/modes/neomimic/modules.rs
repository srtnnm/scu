mod battery;
mod cpu;
mod de;
mod gpu;
mod header;
mod host;
mod kernel;
mod locale;
mod memory;
mod os;
mod packages;
mod separator;
mod shell;
mod terminal;
mod uptime;
mod wm;

use super::row::DataRow;

use crate::info::SystemInformation;

use std::io;

pub(crate) trait ModuleTrait {
    const NAME: &'static str;

    fn run(info: &SystemInformation) -> io::Result<usize>; // usize is an length
}

pub enum Module {
    Battery,
    CPU,
    DE,
    GPU,
    Header,
    Host,
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
    pub fn run(&self, info: &crate::info::SystemInformation) -> std::io::Result<usize> {
        match self {
            Self::Battery => battery::Battery::run(info),
            Self::CPU => cpu::CPU::run(info),
            Self::DE => de::DE::run(info),
            Self::GPU => gpu::GPU::run(info),
            Self::Header => header::Header::run(info),
            Self::Host => host::Host::run(info),
            Self::Kernel => kernel::Kernel::run(info),
            Self::Locale => locale::Locale::run(info),
            Self::Memory => memory::Memory::run(info),
            Self::OS => os::OS::run(info),
            Self::Packages => packages::Packages::run(info),
            Self::Separator => separator::Separator::run(info),
            Self::Shell => shell::Shell::run(info),
            Self::Terminal => terminal::Terminal::run(info),
            Self::Uptime => uptime::Uptime::run(info),
            Self::WM => wm::WM::run(info),
        }
    }
}

// TODO: show possible errors for debugging
pub fn run_module(module: &Module, info: &crate::info::SystemInformation) -> Option<usize> {
    module.run(info).ok()
}
