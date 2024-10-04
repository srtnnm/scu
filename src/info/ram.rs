use libscu::hardware::ram;

pub(super)fn fetch_ram_info() -> Option<ram::RAMInfo> {
    match ram::fetch_info() {
        Ok(ram_info) => Some(ram_info),
        Err(err) => {
            eprintln!("failed to get information about ram: {err:?}");
            None
        }
    }
}