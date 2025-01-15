use libscu::software::kernel;

#[derive(Clone, Debug, Default)]
pub(crate) struct KernelInfo {
    pub name: Option<String>,
    pub version: Option<String>,
    pub arch: Option<String>,
}

impl KernelInfo {
    fn is_none(&self) -> bool {
        self.name.is_none() && self.version.is_none() && self.arch.is_none()
    }
    pub(super) fn fetch() -> Option<KernelInfo> {
        let info = KernelInfo {
            name: kernel::fetch_name().ok(),
            version: kernel::fetch_version().ok(),
            arch: kernel::fetch_arch().ok(),
        };
        if !info.is_none() {
            Some(info)
        } else {
            None
        }
    }
}
