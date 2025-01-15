use crate::{display::modes::neomimic::row::DataRow, info::get_option};

use super::ModuleTrait;

pub struct OS;

impl ModuleTrait for OS {
    const NAME: &'static str = "os";

    fn run(info: &crate::info::SystemInformation) -> std::io::Result<usize> {
        let os_name = get_option("os release", &info.os_release)?;
        let os: String;
        os = if !os_name.pretty_name.is_empty() {
            os_name.pretty_name
        } else if !os_name.name.is_empty() {
            os_name.name
        } else if !os_name.id.is_empty() {
            os_name.id
        } else {
            "Unknown OS".to_string()
        };
        let kernel_arch = info
            .kernel
            .clone()
            .and_then(|kernel| kernel.arch)
            .unwrap_or_default();

        Ok(DataRow::info("OS", format!("{os} {kernel_arch}").trim()))
    }
}
