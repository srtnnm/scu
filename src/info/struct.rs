mod battery;
mod drives;
mod graphics;
mod memory;
mod packages;
mod processor;
mod system;

#[cfg(target_os = "linux")]
pub use battery::Battery;
#[cfg(target_os = "linux")]
pub use drives::Disks;
pub use graphics::Graphics;
pub use memory::Memory;
pub use packages::Packages;
pub use processor::Processor;
pub use system::System;

