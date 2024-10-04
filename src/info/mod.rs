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

use crate::config::{Config, Table};

use libscu::{
    hardware::{
        battery::BatteryInfo, cpu::CPUInfo, disk::Disk, display::Brightness, gpu::GPUInfo,
        ram::RAMInfo,
    },
    software::{
        graphics::{DisplayServer, WindowManager},
        init::InitSystem,
        os::OSRelease,
        packages::PackageManager,
        shell::Shell,
        terminal::TerminalInfo,
    },
    types::Time,
    util::data::DesktopEnvironment,
};

#[derive(Debug)]
pub(crate) struct SystemInformation {
    pub batteries: Vec<BatteryInfo>,
    pub cpu: Option<CPUInfo>,
    pub desktop_environment: Option<DesktopEnvironment>,
    pub device_name: Option<String>,
    pub disks: Vec<Disk>,
    pub display_brightness: Option<Brightness>,
    pub display_server: Option<DisplayServer>,
    pub gpus: Vec<GPUInfo>,
    pub hostname: Option<String>,
    pub init_system: Option<InitSystem>,
    pub kernel: Option<kernel::KernelInfo>,
    pub os_release: Option<OSRelease>,
    pub packages: Vec<PackageManager>,
    pub ram: Option<RAMInfo>,
    pub shell: Option<Shell>,
    pub terminal: Option<TerminalInfo>,
    pub uptime: Option<Time>,
    pub username: Option<String>,
    pub window_manager: Option<WindowManager>,
}

impl Default for SystemInformation {
    fn default() -> Self {
        Self {
            batteries: Vec::default(),
            cpu: None,
            desktop_environment: None,
            device_name: None,
            disks: Vec::default(),
            display_brightness: None,
            display_server: None,
            gpus: Vec::default(),
            hostname: None,
            init_system: None,
            kernel: None,
            os_release: None,
            packages: Vec::default(),
            ram: None,
            shell: None,
            terminal: None,
            uptime: None,
            username: None,
            window_manager: None,
        }
    }
}

impl SystemInformation {
    pub(crate) fn new() -> Self {
        Self::default()
    }
    pub(crate) fn fetch(&mut self, config: &Config, force_versions: bool) {
        for table in &config.order {
            match *table {
                Table::PROCESSOR => self.cpu = cpu::fetch_cpu_info(),
                Table::GRAPHICS => {
                    self.desktop_environment = desktop_environment::fetch();
                    self.display_brightness = display_brightness::fetch();
                    self.display_server = display_server::fetch();
                    self.gpus = gpu::fetch_gpus();
                    self.window_manager = window_manager::fetch(force_versions);
                }
                Table::MEMORY => self.ram = ram::fetch_ram_info(),
                Table::SYSTEM => {
                    self.device_name = device_name::fetch();
                    self.hostname = hostname::fetch();
                    self.init_system = init_system::fetch();
                    self.kernel = kernel::KernelInfo::fetch();
                    self.os_release = os_release::fetch();
                    self.shell = shell::fetch(force_versions);
                    self.terminal = terminal::fetch(force_versions);
                    self.uptime = uptime::fetch();
                    self.username = username::fetch();
                }
                Table::BATTERY => self.batteries = battery::fetch_batteries_info(),
                Table::DISKS => self.disks = disks::fetch_disks(),
                Table::PACKAGES => self.packages = packages::fetch_package_managers(),
                _ => {}
            }
        }
    }
}
