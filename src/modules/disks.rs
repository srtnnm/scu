use super::Detection;

use libscu::hardware::disk;

pub(super) fn fetch_disks() -> Vec<disk::Disk> {
    let disks = disk::fetch_devices().unwrap_or_default();

    disks
        .iter()
        .map(|device| disk::fetch_disk_info(&device))
        .flatten()
        .collect::<Vec<disk::Disk>>()
}

pub struct Disks;

impl Detection for Disks {
    type Result = Vec<disk::Disk>;
    const NAME: &'static str = "disks";

    fn fetch() -> std::io::Result<Self::Result> {
        let disks = disk::fetch_devices()?;

        Ok(disks
            .iter()
            .map(|device| disk::fetch_disk_info(&device))
            .flatten()
            .collect())
    }
}
