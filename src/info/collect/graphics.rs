use crate::info::r#struct::Graphics;
#[cfg(any(target_os = "linux", target_os = "android"))]

#[cfg(any(target_os = "linux", target_os = "android"))]
use libscu::hardware::{display, gpu};
use libscu::software::graphics;

pub fn collect(force_version: bool) -> Graphics {
    let mut result = Graphics::default();

    #[cfg(target_os = "linux")]
    {
        result.gpu_list = gpu::fetch_all();
    }
    result.environment = graphics::fetch_environment();
    result.window_manager = graphics::fetch_window_manager(force_version);
    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        result.display_brightness = display::fetch_brightness();
    }

    result
}
