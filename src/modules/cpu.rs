use super::Detection;

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

pub(super) fn fetch_multicpu_info() -> Vec<cpu::Unit> {
    match cpu::fetch_multicpus(raw_models()) {
        Ok(cpus) => cpus,
        Err(err) => {
            eprintln!("failed to get information about multiple cpus: {err:?}");
            Vec::default()
        }
    }
}

pub struct CPU;
pub struct MultiCPU;

impl Detection for CPU {
    type Result = cpu::CPUInfo;
    const NAME: &'static str = "cpu";

    fn fetch() -> std::io::Result<Self::Result> {
        cpu::fetch_info(raw_models())
    }
}

impl Detection for MultiCPU {
    type Result = Vec<cpu::Unit>;
    const NAME: &'static str = "cpu";

    fn fetch() -> std::io::Result<Self::Result> {
        cpu::fetch_multicpus(raw_models())
    }
}
