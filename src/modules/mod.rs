macro_rules! export_modules {
    ( $( $x:ident ),* ) => {
        $(
            mod $x;
            pub use self::$x::*;
        )*
    };
}

export_modules!(
    arch,
    battery,
    brightness,
    cpu,
    de,
    device_name,
    disks,
    display_server,
    gpu,
    hostname,
    init_system,
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
        battery::BatteryInfo, cpu::CPUInfo, cpu::Unit, disk::Disk, display::Brightness,
        gpu::GPUInfo, ram::RAMInfo,
    },
    software::{
        graphics::DisplayServer, graphics::WindowManager, init::InitSystem, os::OSRelease,
        packages::PackageManager, shell::Shell, terminal::TerminalInfo,
    },
    types::Time,
    util::data::DesktopEnvironment,
};

#[derive(Debug, Default)]
pub(crate) struct SystemInformation {
    pub arch: Option<String>,
    pub batteries: Vec<BatteryInfo>,
    pub cpu: Option<CPUInfo>,
    pub multicpu: Vec<Unit>,
    pub desktop_environment: Option<DesktopEnvironment>,
    pub device_name: Option<String>,
    pub disks: Vec<Disk>,
    pub display_brightness: Option<Brightness>,
    pub display_server: Option<DisplayServer>,
    pub gpus: Vec<GPUInfo>,
    pub hostname: Option<String>,
    pub init_system: Option<InitSystem>,
    pub kernel: Option<kernel::KernelInfo>,
    pub locale: Option<String>,
    pub os_release: Option<OSRelease>,
    pub packages: Vec<PackageManager>,
    pub ram: Option<RAMInfo>,
    pub rootfs_fstype: Option<String>,
    pub shell: Option<Shell>,
    pub terminal: Option<TerminalInfo>,
    pub uptime: Option<Time>,
    pub username: Option<String>,
    pub window_manager: Option<WindowManager>,
}

impl SystemInformation {
    pub(crate) fn new() -> Self {
        Self::default()
    }
    pub(crate) fn fetch(&mut self, config: &Config, args: &Args) {
        for table in &config.order {
            match *table {
                Table::PROCESSOR => {
                    if args.multicpu {
                        self.multicpu = cpu::fetch_multicpu_info();
                    } else {
                        self.cpu = cpu::fetch_cpu_info();
                    }
                }
                Table::GRAPHICS => {
                    self.desktop_environment = de::fetch();
                    self.display_brightness = brightness::fetch();
                    self.display_server = display_server::fetch();
                    self.gpus = gpu::fetch_gpus();
                    self.window_manager = wm::fetch(args.force_versions);
                }
                Table::MEMORY => self.ram = ram::fetch_ram_info(),
                Table::SYSTEM => {
                    self.arch = arch::fetch();
                    self.device_name = device_name::fetch();
                    self.hostname = hostname::fetch();
                    self.init_system = init_system::fetch();
                    self.kernel = kernel::KernelInfo::fetch();
                    self.os_release = os::fetch();
                    self.shell = shell::fetch(args.force_versions);
                    self.terminal = terminal::fetch(args.force_versions);
                    self.uptime = uptime::fetch();
                    self.username = username::fetch();
                    self.rootfs_fstype = rootfs::get_rootfs_fstype();
                    self.locale = locale::fetch();
                }
                Table::BATTERY => self.batteries = battery::fetch_batteries_info(),
                Table::DISKS => self.disks = disks::fetch_disks(),
                Table::PACKAGES => self.packages = packages::fetch_package_managers(),
                _ => {}
            }
        }
    }
}

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

    fn fetch() -> std::io::Result<Self::Result>;
}

/*
macro_rules! expose_submodules {
    ( $( $x:ident ),* ) => {
        $(
            mod $x;
            pub use self::$x::*;
        )*
    };
}
*/
