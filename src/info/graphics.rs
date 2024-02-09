use crate::data::table::*;
use crate::utils::{colorize::colorize_by_num, percentage};

use libscu::{
    hardware::{display, gpu},
    software::graphics,
};

pub fn collect(simplify: bool, force_version: bool) -> Table {
    let mut result = Table::new("Graphics");

    let gpus = gpu::fetch_all();
    if !gpus.is_empty() {
        let count_gpus = gpus.len();
        for entry in gpus.iter().enumerate() {
            let (gpu_id, gpu_info) = (entry.0, entry.1);
            let mut sub_info: Vec<TableEntry> = Vec::new();
            let (_, _) = (
                gpu_info.driver.as_ref().is_some_and(|gpu_driver| {
                    sub_info.push(TableEntry::new("Driver", &gpu_driver));
                    true
                }),
                gpu_info.temperature.is_some_and(|temp| {
                    if temp > 0.0 {
                        sub_info.push(TableEntry::new(
                            "Temperature",
                            format!("{}°C", temp).as_str(),
                        ));
                    };
                    true
                }),
            );
            result.add_with_additional(
                format!(
                    "GPU{}",
                    if count_gpus > 1 {
                        format!(" #{}", gpu_id)
                    } else {
                        "".to_string()
                    }
                )
                .as_str(),
                format!("{} {}", gpu_info.vendor, gpu_info.model).as_str(),
                sub_info,
            );
        }
    }
    if let Some(display_server) = graphics::fetch_display_server() {
        result.add("Display server", format!("{:?}", display_server).as_str());
    }
    if let Some(environment) = graphics::fetch_environment() {
        result.add("Environment", &environment);
    }
    if let Some(wm) = graphics::fetch_window_manager(force_version) {
        result.add(
            "Window manager",
            format!(
                "{}{}",
                wm.name,
                if let Some(wm_version) = wm.version {
                    format!(" v{}", wm_version)
                } else {
                    "".to_string()
                }
            )
            .as_str(),
        );
    }
    if let Some(display_brightness) = display::fetch_brightness() {
        let percentage = percentage(
            display_brightness.max as u64,
            display_brightness.current as u64,
        ) as u16;
        result.add(
            "Brightness",
            if !simplify {
                colorize_by_num(
                    format!("{}%", percentage).as_str(),
                    percentage as u16,
                    40,
                    true,
                )
            } else {
                format!("{}%", percentage)
            }
            .as_str(),
        )
    }

    result
}
