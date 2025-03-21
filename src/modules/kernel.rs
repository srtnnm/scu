use super::Detection;

use libscu::software::kernel;

#[derive(Clone, Debug, Default)]
pub struct KernelInfo {
    pub name: String,
    pub version: String,
}

pub struct Kernel;

impl Detection for Kernel {
    type Result = KernelInfo;
    const NAME: &'static str = "kernel";

    fn fetch(&self) -> std::io::Result<Self::Result> {
        Ok(KernelInfo {
            name: kernel::fetch_name()?,
            version: kernel::fetch_version()?,
        })
    }
}
