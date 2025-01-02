use crate::data::raw_models;

use libscu::hardware::cpu;
#[cfg(target_os = "linux")]
use libscu::hardware::cpu::Unit;

pub(super) fn fetch_cpu_info() -> Option<cpu::CPUInfo> {
    match cpu::fetch_info(raw_models()) {
        Ok(cpu_info) => Some(cpu_info),
        Err(err) => {
            eprintln!("failed to get information about cpu: {err:?}");
            None
        }
    }
}

#[cfg(target_os = "linux")]
pub(super) fn fetch_multicpu_info() -> Vec<Unit> {
    match cpu::fetch_multicpus(raw_models()) {
        Ok(cpus) => cpus,
        Err(err) => {
            eprintln!("failed to get information about multiple cpus: {err:?}");
            Vec::default()
        }
    }
}
