mod battery;
mod cpu;
mod desktop_environment;
mod device_name;
mod disks;
mod display_brightness;
mod display_server;
mod gpu;
mod hostname;
mod init_system;
mod kernel;
mod os_release;
mod packages;
mod ram;
mod shell;
mod terminal;
mod uptime;
mod username;
mod window_manager;
mod rootfs;

use crate::{
    args::Args,
    config::{Config, Table},
};

#[cfg(target_os = "linux")]
use libscu::{
    hardware::{battery::BatteryInfo, disk::Disk, gpu::GPUInfo, cpu::Unit},
    software::{graphics::DisplayServer, init::InitSystem},
};
use libscu::{
    hardware::{
        cpu::CPUInfo,
        display::Brightness,
        ram::RAMInfo,
    },
    software::{
        graphics::WindowManager, os::OSRelease, packages::PackageManager, shell::Shell,
        terminal::TerminalInfo,
    },
    types::Time,
    util::data::DesktopEnvironment,
};

#[derive(Debug, Default)]
pub(crate) struct SystemInformation {
    #[cfg(target_os = "linux")]
    pub batteries: Vec<BatteryInfo>,
    pub cpu: Option<CPUInfo>,
    #[cfg(target_os = "linux")]
    pub multicpu: Vec<Unit>,
    pub desktop_environment: Option<DesktopEnvironment>,
    pub device_name: Option<String>,
    #[cfg(target_os = "linux")]
    pub disks: Vec<Disk>,
    pub display_brightness: Option<Brightness>,
    #[cfg(target_os = "linux")]
    pub display_server: Option<DisplayServer>,
    #[cfg(target_os = "linux")]
    pub gpus: Vec<GPUInfo>,
    pub hostname: Option<String>,
    #[cfg(target_os = "linux")]
    pub init_system: Option<InitSystem>,
    pub kernel: Option<kernel::KernelInfo>,
    pub os_release: Option<OSRelease>,
    pub packages: Vec<PackageManager>,
    pub ram: Option<RAMInfo>,
    #[cfg(any(target_os = "linux", target_os = "android"))]
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
                    self.cpu = cpu::fetch_cpu_info();
                    #[cfg(target_os = "linux")]
                    {
                        self.multicpu = cpu::fetch_multicpu_info();
                    }
                }
                Table::GRAPHICS => {
                    self.desktop_environment = desktop_environment::fetch();
                    self.display_brightness = display_brightness::fetch();
                    #[cfg(target_os = "linux")]
                    {
                        self.display_server = display_server::fetch();
                        self.gpus = gpu::fetch_gpus();
                    }
                    self.window_manager = window_manager::fetch(args.force_versions);
                }
                Table::MEMORY => self.ram = ram::fetch_ram_info(),
                Table::SYSTEM => {
                    self.device_name = device_name::fetch();
                    self.hostname = hostname::fetch();
                    #[cfg(target_os = "linux")]
                    {
                        self.init_system = init_system::fetch();
                    }
                    self.kernel = kernel::KernelInfo::fetch();
                    self.os_release = os_release::fetch();
                    self.shell = shell::fetch(args.force_versions);
                    self.terminal = terminal::fetch(args.force_versions);
                    self.uptime = uptime::fetch();
                    self.username = username::fetch();
                    #[cfg(any(target_os = "linux", target_os = "android"))]
                    {
                        self.rootfs_fstype = rootfs::get_rootfs_fstype();
                    }
                }
                #[cfg(target_os = "linux")]
                Table::BATTERY => self.batteries = battery::fetch_batteries_info(),
                #[cfg(target_os = "linux")]
                Table::DISKS => self.disks = disks::fetch_disks(),
                Table::PACKAGES => self.packages = packages::fetch_package_managers(),
                _ => {}
            }
        }
    }
}
