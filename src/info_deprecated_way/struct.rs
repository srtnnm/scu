mod battery;
mod disks;
mod graphics;
mod memory;
mod packages;
mod processor;
mod system;

#[cfg(target_os = "linux")]
pub use battery::Battery;
#[cfg(target_os = "linux")]
pub use disks::Disks;
pub use graphics::Graphics;
pub use memory::Memory;
pub use packages::Packages;
pub use processor::Processor;
pub use system::System;
