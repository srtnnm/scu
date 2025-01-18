#![allow(unused_imports)]

macro_rules! export_modules {
    ( $( $x:ident ),* ) => {
        $(
            mod $x;
            pub use $x::*;
        )*
    };
}

export_modules!(
    arch,
    battery,
    brightness,
    cpu,
    de,
    device,
    disks,
    display_server,
    gpu,
    hostname,
    init,
    kernel,
    locale,
    os,
    packages,
    ram,
    rootfs,
    shell,
    terminal,
    uptime,
    username,
    wm
);

use crate::{
    args::Args,
    config::{Config, Table},
};

use libscu::{
    hardware::{
        battery as libscu_battery, cpu as libscu_cpu, disk as libscu_disk,
        display as libscu_display, gpu as libscu_gpu, ram as libscu_ram,
    },
    software::{
        graphics, init as libscu_init, os as libscu_os, packages as libscu_packages,
        shell as libscu_shell, terminal as libscu_terminal,
    },
    types::Time,
    util::data::DesktopEnvironment,
};

// #[derive(Debug, Default)]
// pub(crate) struct SystemInformation {
//     pub arch: Option<String>,
//     pub batteries: Vec<libscu_battery::BatteryInfo>,
//     pub cpu: Option<libscu_cpu::CPUInfo>,
//     pub multicpu: Vec<libscu_cpu::Unit>,
//     pub desktop_environment: Option<DesktopEnvironment>,
//     pub device_name: Option<String>,
//     pub disks: Vec<libscu_disk::Disk>,
//     pub display_brightness: Option<libscu_display::Brightness>,
//     pub display_server: Option<graphics::DisplayServer>,
//     pub gpus: Vec<libscu_gpu::GPUInfo>,
//     pub hostname: Option<String>,
//     pub init_system: Option<libscu_init::InitSystem>,
//     pub kernel: Option<kernel::KernelInfo>,
//     pub locale: Option<String>,
//     pub os_release: Option<libscu_os::OSRelease>,
//     pub packages: Vec<libscu_packages::PackageManager>,
//     pub ram: Option<libscu_ram::RAMInfo>,
//     pub rootfs_fstype: Option<String>,
//     pub shell: Option<libscu_shell::Shell>,
//     pub terminal: Option<libscu_terminal::TerminalInfo>,
//     pub uptime: Option<Time>,
//     pub username: Option<String>,
//     pub window_manager: Option<graphics::WindowManager>,
// }

pub fn get_option<T>(variable_name: &str, variable: &Option<T>) -> std::io::Result<T>
where
    T: Clone,
{
    let Some(var) = variable.clone() else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("failed to determine {variable_name}"),
        ));
    };
    Ok(var)
}

pub fn get_vec<T>(data_kind: &str, variable: &Vec<T>) -> std::io::Result<Vec<T>>
where
    T: Clone,
{
    let var = variable.clone();
    if var.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("no {data_kind} found"),
        ));
    } else {
        Ok(var)
    }
}

pub trait Detection {
    type Result;
    const NAME: &'static str;

    fn fetch(&self) -> std::io::Result<Self::Result>;
}

#[derive(Debug, PartialEq)]
pub enum Module {
    Arch,
    Battery,
    Brightness,
    CPU,
    DE,
    Device,
    Disks,
    DisplayServer,
    GPU,
    Header,
    Hostname,
    Init,
    Kernel,
    Locale,
    Memory,
    OS,
    Packages,
    RootFS,
    Separator,
    Shell,
    Terminal,
    Uptime,
    Username,
    WM,
}

const MODULE_STRING_REPRESENTATION: [(Module, &str); 24] = [
    (Module::Arch, "arch"),
    (Module::Battery, "battery"),
    (Module::Brightness, "brightness"),
    (Module::CPU, "cpu"),
    (Module::DE, "de"),
    (Module::Device, "device"),
    (Module::Disks, "disks"),
    (Module::DisplayServer, "display_server"),
    (Module::GPU, "gpu"),
    (Module::Header, "header"),
    (Module::Hostname, "hostname"),
    (Module::Init, "init"),
    (Module::Kernel, "kernel"),
    (Module::Locale, "locale"),
    (Module::Memory, "memory"),
    (Module::OS, "os"),
    (Module::Packages, "packages"),
    (Module::RootFS, "rootfs"),
    (Module::Separator, "separator"),
    (Module::Shell, "shell"),
    (Module::Terminal, "terminal"),
    (Module::Uptime, "uptime"),
    (Module::Username, "username"),
    (Module::WM, "wm"),
];

impl Module {
    pub fn from_str(name: &str) -> Option<Self> {
        for (module, string_representation) in MODULE_STRING_REPRESENTATION {
            if string_representation.starts_with(name) {
                return Some(module);
            }
        }
        None
    }
    pub fn to_str(&self) -> &'static str {
        for (module, string_representation) in MODULE_STRING_REPRESENTATION.iter() {
            if module == self {
                return string_representation;
            }
        }
        panic!("string representation for {self:?} not found in MODULE_STRING_REPRESENT")
    }
}
