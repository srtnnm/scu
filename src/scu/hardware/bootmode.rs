#![cfg(feature="bootmode")]

#[derive(Clone,Debug)]
pub enum BootMode {
    BIOS,
    UEFI
}

pub fn get() -> BootMode {
    match std::fs::metadata("/sys/firmware/efi/fw_platform_size") {
        Ok(_) => BootMode::UEFI,
        Err(_) => BootMode::BIOS
    }
}