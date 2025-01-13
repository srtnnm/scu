mod cpu;
mod de;
mod gpu;
mod header;
mod host;
mod kernel;
mod memory;
mod os;
mod packages;
mod shell;
mod terminal;
mod uptime;
mod wm;

use super::row::DataRow;

use crate::info::SystemInformation;

use std::io;

pub(crate) trait ModuleTrait {
    const NAME: &'static str;

    fn get(info: &SystemInformation) -> io::Result<DataRow>;
}

pub enum Module {
    CPU,
    DE,
    GPU,
    Header,
    Host,
    Kernel,
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
    pub fn run(&self, info: &crate::info::SystemInformation) -> std::io::Result<DataRow> {
        match self {
            Self::CPU => cpu::CPU::get(info),
            Self::DE => de::DE::get(info),
            Self::GPU => gpu::GPU::get(info),
            Self::Header => header::Header::get(info),
            Self::Host => host::Host::get(info),
            Self::Kernel => kernel::Kernel::get(info),
            Self::Memory => memory::Memory::get(info),
            Self::OS => os::OS::get(info),
            Self::Packages => packages::Packages::get(info),
            Self::Separator => Ok(DataRow::separator('-')),
            Self::Shell => shell::Shell::get(info),
            Self::Terminal => terminal::Terminal::get(info),
            Self::Uptime => uptime::Uptime::get(info),
            Self::WM => wm::WM::get(info),
        }
    }
}

// TODO: show possible errors for debugging
pub fn run_module(module: &Module, info: &crate::info::SystemInformation) -> Option<DataRow> {
    module.run(info).ok()
}
