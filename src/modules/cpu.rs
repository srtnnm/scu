use super::Detection;

use crate::config::{multicpu, raw_models};

use libscu::hardware::cpu;

pub struct CPU;
pub struct MultiCPU;

impl Detection for CPU {
    type Result = Vec<cpu::Unit>;
    const NAME: &'static str = "cpu";

    fn fetch(&self) -> std::io::Result<Self::Result> {
        if multicpu() {
            Ok(Vec::from([cpu::Unit {
                physical_id: 0,
                cpuinfo: cpu::fetch_info(raw_models())?,
            }]))
        } else {
            cpu::fetch_multicpus(raw_models())
        }
    }
}

impl Detection for MultiCPU {
    type Result = Vec<cpu::Unit>;
    const NAME: &'static str = "cpu";

    fn fetch(&self) -> std::io::Result<Self::Result> {
        cpu::fetch_multicpus(raw_models())
    }
}
