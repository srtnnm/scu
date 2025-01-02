use crate::{
    data::table::{Table, TableEntry},
    info,
    util::{colorize::colorize_by_num, percentage},
};

pub fn to_table(info: &info::SystemInformation, _disable_color: bool) -> Option<Table> {
    let mut result = Table::new("Graphics");

    #[cfg(any(target_os = "linux"))]
    {
        let mut gpu_sub_info: Vec<TableEntry> = Vec::new();
        for (gpu_id, gpu_info) in info.gpus.iter().enumerate() {
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
                    if info.gpus.len() > 1 {
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
        let _ = info.display_server.as_ref().is_some_and(|ds| {
            result.add("Display server", format!("{:?}", ds).as_str());
            true
        });
    }
    if let Some(environment) = info.desktop_environment.clone() {
        result.add("Environment", environment.to_str());
    }
    if let Some(wm) = &info.window_manager {
        if let Some(name) = wm.name.clone() {
            result.add(
                "Window manager",
                format!(
                    "{}{}",
                    name,
                    wm.version
                        .as_ref()
                        .map(|v| format!(" v{v}"))
                        .unwrap_or_default()
                )
                .as_str(),
            );
        }
    }
    #[cfg(any(target_os = "linux", target_os = "android"))]
    if let Some(display_brightness) = &info.display_brightness {
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

    Some(result)
}
