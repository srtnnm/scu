use crate::info::r#struct::Graphics;

use libscu::{
    hardware::{display, gpu},
    software::graphics,
};

pub fn collect(force_version: bool) -> Graphics {
    let mut result = Graphics::default();

    result.gpu_list = gpu::fetch_all();
    result.display_server = graphics::fetch_display_server();
    result.environment = graphics::fetch_environment();
    result.window_manager = graphics::fetch_window_manager(force_version);
    result.display_brightness = display::fetch_brightness();

    result
}
