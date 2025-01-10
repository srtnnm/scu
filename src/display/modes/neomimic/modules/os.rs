use crate::display::modes::neomimic::row::DataRow;

use super::Module;

pub struct OS;

impl Module for OS {
    const NAME: &'static str = "os";

    fn get(
        info: &crate::info::SystemInformation,
    ) -> std::io::Result<crate::display::modes::neomimic::row::DataRow> {
        let os_name = info
            .os_release
            .clone()
            .and_then(|osrelease| Some(osrelease.pretty_name))
            .unwrap_or_default();
        let kernel_arch = info
            .kernel
            .clone()
            .and_then(|kernel| kernel.arch)
            .unwrap_or_default();

        Ok(DataRow::info(
            "OS",
            format!("{os_name} {kernel_arch}").trim(),
        ))
    }
}
