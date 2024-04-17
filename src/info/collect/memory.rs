use crate::info::r#struct::Memory;

use libscu::hardware::ram;

pub fn collect() -> Memory {
    let mut result = Memory::default();

    let mem = ram::fetch_info();
    (result.ram_total, result.ram_used) = (mem.total, mem.used);

    if let Some(swap) = mem.swap {
        result.swap_enabled = true;
        (result.swap_total, result.swap_used) = (swap.total, swap.used);
    }

    result
}
