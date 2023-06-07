use std::path::Path;

pub fn detect() -> String {
    let comm = std::fs::read_to_string("/proc/1/comm").unwrap_or("Unknown".into());

    (match comm.trim() {
        "systemd" => "Systemd",
        "openrc-init" | "init-openrc" => "OpenRC",
        "runit" => "Runit",
        "init" => {
            if Path::new("/run/dinit").exists() {
                "Dinit"
            } else if Path::new("/usr/share/sysvinit/inittab").exists() {
                "SysVinit"
            } else {
                "Unknown"
            }
        }
        "s6-svscan" => "s6",
        "upstart" => "Upstart",
        s => s,
    })
    .into()
}
