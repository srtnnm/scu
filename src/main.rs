#![allow(
dead_code,
unused_imports
)]

mod software;
mod hardware;

mod utils;

use whoami;

fn print_system_info() {
  let distro_name    = software::os::get_name();
  let uptime           = software::os::get_uptime();
  let hostname       = software::os::get_hostname();
  let shell          = software::os::get_shell();
  let kernel_info    = software::kernel::get_info();
  let init_system    = software::init_system::detect();
  let terminal       = software::terminal::get_name();

  println!( "[SYSTEM]" );
  println!( "Hostname:   {}", hostname );
  println!( "Username:   {}", whoami::username() );
  println!( "Distro:     {}", distro_name );
  println!( "Kernel:     {}", kernel_info.full_version );
  println!( "Kernel version: {}", kernel_info.version );
  println!( "Init system: {}", init_system );
  println!( "Terminal:   {}", terminal );
  println!( "Shell:      {}", shell );
  println!( "Uptime:     {}H {}M {}S", uptime.hours, uptime.minutes, uptime.seconds );
}

fn print_pkgs_info() {
  let pkg_info = software::packages::get_info();
  if !pkg_info.is_empty() {
    println!( "[PACKAGES]" );
    for manager in pkg_info {
      println!( "({}): {}", manager.manager, manager.count_of_packages );
    }
  }
}

fn print_cpu_info() {
  let cpu_info = hardware::cpu::get_info();
  println!( "[CPU]" );
  println!( "Vendor:   {}", cpu_info.vendor );
  println!( "Model:    {}", cpu_info.model );
  println!( "Max freq: {}GHz", cpu_info.max_freq.ghz );
  println!( "Cores:    {}", cpu_info.cores );
  println!( "Threads:  {}", cpu_info.threads );
}

fn print_memory_info() {
  let mem_info = hardware::ram::get_info();
  println!( "[MEMORY]" );
  println!( "Total:      {}MiB", mem_info.total.mb );
  println!( "Used:       {}MiB", mem_info.used.mb  );
  if mem_info.swap_enabled {
    println!( "Swap total: {}MiB", mem_info.swap_total.mb );
    println!( "Swap used:  {}MiB", mem_info.swap_used.mb );
  }
}

fn print_drives_info() {
  let drives = hardware::drive::scan_drives();
  if !drives.is_empty() {
    println!( "[DRIVES]" );
    for drive in drives {
      println!( "{}: {}MiB", drive.model, drive.size.mb );
    }
  }
}

fn print_full_info() {
  print_system_info();
  print_pkgs_info();
  print_cpu_info();
  print_memory_info();
  print_drives_info();
}

fn main() {
  print_full_info();
}
