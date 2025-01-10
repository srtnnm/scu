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

use super::row::DataRow;

use crate::info::SystemInformation;

use std::io;

pub(crate) trait Module {
    const NAME: &'static str;

    fn get(info: &SystemInformation) -> io::Result<DataRow>;
}
