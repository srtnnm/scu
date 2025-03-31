use super::Detection;

use libscu::hardware::disk;

pub struct Disks;

impl Detection for Disks {
    type Result = Vec<disk::Disk>;
    const NAME: &'static str = "disks";

    fn fetch(&self) -> std::io::Result<Self::Result> {
        let disks = disk::fetch_devices()?;

        Ok(disks.iter().flat_map(disk::fetch_disk_info).collect())
    }
}
