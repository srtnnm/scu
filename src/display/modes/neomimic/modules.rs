mod header;
pub use header::Header;
mod os;
pub use os::OS;
mod host;
pub use host::Host;
mod kernel;
pub use kernel::Kernel;
mod uptime;
pub use uptime::Uptime;
mod packages;
pub use packages::Packages;
mod shell;
pub use shell::Shell;
mod de;
pub use de::DE;
mod wm;
pub use wm::WM;
mod terminal;
pub use terminal::Terminal;
mod cpu;
pub use cpu::CPU;
mod gpu;
pub use gpu::GPU;
mod memory;
pub use memory::Memory;

use super::row::DataRow;

use crate::info::SystemInformation;

use std::io;

pub(crate) trait Module {
    const NAME: &'static str;

    fn get(info: &SystemInformation) -> io::Result<DataRow>;
}
