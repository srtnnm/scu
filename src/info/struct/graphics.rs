use crate::data::table::*;
#[cfg(any(target_os = "linux", target_os = "android"))]
use crate::utils::{colorize::colorize_by_num, percentage};

#[cfg(any(target_os = "linux", target_os = "android"))]
use libscu::hardware::display::Brightness;
#[cfg(any(target_os = "linux"))]
use libscu::{hardware::gpu::GPUInfo, software::graphics::DisplayServer};

use libscu::software::graphics::WindowManager;

#[derive(Default)]
pub struct Graphics {
    #[cfg(target_os = "linux")]
    pub gpu_list: Vec<GPUInfo>,
    #[cfg(target_os = "linux")]
    pub display_server: Option<DisplayServer>,
    pub environment: Option<String>,
    pub window_manager: Option<WindowManager>,
    #[cfg(any(target_os = "linux", target_os = "android"))]
    pub display_brightness: Option<Brightness>,
}

impl Graphics {
    pub fn to_print(&self, _disable_color: bool) -> Table {
        let mut result = Table::new("Graphics");

        #[cfg(any(target_os = "linux"))]
        {
            let mut gpu_sub_info: Vec<TableEntry> = Vec::new();
            for (gpu_id, gpu_info) in self.gpu_list.iter().enumerate() {
                if let Some(gpu_temp) = gpu_info.temperature {
                    if gpu_temp > 0.0 {
                        gpu_sub_info.push(TableEntry::new(
                            "Temperature",
                            format!("{}°C", gpu_temp).as_str(),
                        ));
                    };
                }
                if let Some(gpu_driver) = gpu_info.driver.as_ref() {
                    gpu_sub_info.push(TableEntry::new("Driver", &gpu_driver))
                }
                result.add_with_additional(
                    format!(
                        "GPU{}",
                        if self.gpu_list.len() > 1 {
                            format!(" #{}", gpu_id)
                        } else {
                            "".to_string()
                        }
                    )
                    .as_str(),
                    format!("{} {}", gpu_info.vendor, gpu_info.model).as_str(),
                    gpu_sub_info.clone(),
                );
                gpu_sub_info.clear();
            }
            let _ = self.display_server.as_ref().is_some_and(|ds| {
                result.add("Display server", format!("{:?}", ds).as_str());
                true
            });
        }
        if let Some(environment) = self.environment.clone() {
            result.add("Environment", &environment);
        }
        let _ = self.window_manager.as_ref().is_some_and(|wm| {
            result.add(
                "Window manager",
                format!(
                    "{}{}",
                    wm.name,
                    wm.version
                        .as_ref()
                        .map(|v| format!(" v{v}"))
                        .unwrap_or_default()
                )
                .as_str(),
            );
            true
        });
        #[cfg(any(target_os = "linux", target_os = "android"))]
        if let Some(display_brightness) = &self.display_brightness {
            let percentage = percentage(
                display_brightness.max as u64,
                display_brightness.current as u64,
            ) as u16;
            result.add(
                "Brightness",
                if !_disable_color {
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
}
