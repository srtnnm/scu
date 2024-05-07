use crate::info::r#struct::System;

use libscu::{
    hardware::device,
    software::{hostname, kernel, os, shell, terminal, time, users},
};
#[cfg(not(target_os = "android"))]
use libscu::software::init;

pub fn collect(force_version: bool) -> System {
    let mut result = System::default();

    result.hostname = hostname::fetch(false);
    result.username = users::fetch_current().map(|u| u.name);
    result.os_name = os::fetch_name().map(|on| on.pretty_name);
    result.device_name = device::fetch_model(false);
    result.kernel_version = kernel::fetch_version();
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {result.init_system = init::fetch_info().ok();}
    result.terminal_info = terminal::fetch_info(force_version).ok();
    result.shell = shell::fetch_info(force_version);
    result.uptime = time::fetch_uptime();

    result
}
