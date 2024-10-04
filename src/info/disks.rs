use libscu::hardware::disk;

pub(super) fn fetch_disks() -> Vec<disk::Disk> {
    let disks = disk::fetch_devices().unwrap_or_default();

    disks
        .iter()
        .map(|device| disk::fetch_disk_info(&device))
        .flatten()
        .collect::<Vec<disk::Disk>>()
}
