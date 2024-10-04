use crate::data::raw_models;

use libscu::hardware::cpu;

pub(super) fn fetch_cpu_info() -> Option<cpu::CPUInfo> {
    match cpu::fetch_info(raw_models()) {
        Ok(cpu_info) => Some(cpu_info),
        Err(err) => {
            eprintln!("failed to get information about cpu: {err:?}");
            None
        }
    }
}
