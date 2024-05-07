use crate::{
    data::table::Table,
    utils::{colorize::colorize_by_num, percentage},
};

use libscu::types::Memory as MemorySize;

#[derive(Default)]
pub struct Memory {
    pub ram_total: MemorySize,
    pub ram_used: MemorySize,
    pub swap_enabled: bool,
    pub swap_total: MemorySize,
    pub swap_used: MemorySize,
}

impl Memory {
    pub fn to_print(&self, disable_color: bool) -> Table {
        let mut result = Table::new("Memory");

        let ram_usage_percents = percentage(self.ram_total.mb as u64, self.ram_used.mb as u64);
        result.add(
            "RAM",
            format!(
                "{}MiB / {}MiB [{}]",
                self.ram_used.mb,
                self.ram_total.mb,
                if !disable_color {
                    colorize_by_num(
                        format!("{:.1}%", ram_usage_percents).as_str(),
                        ram_usage_percents as u16,
                        100,
                        false,
                    )
                } else {
                    format!("{:.1}%", ram_usage_percents)
                }
            )
            .as_str(),
        );
        if self.swap_enabled {
            let swap_usage_percents =
                percentage(self.swap_total.mb as u64, self.swap_used.mb as u64);
            result.add(
                "Swap",
                format!(
                    "{}MiB / {}MiB [{}]",
                    self.swap_used.mb,
                    self.swap_total.mb,
                    if !disable_color {
                        colorize_by_num(
                            format!("{:.1}%", swap_usage_percents).as_str(),
                            swap_usage_percents as u16,
                            100,
                            false,
                        )
                    } else {
                        format!("{:.1}%", swap_usage_percents)
                    }
                )
                .as_str(),
            );
        }

        result
    }
}
