use crate::data::table::*;
use crate::utils::{colorize::colorize_by_num, percentage};

use libscu::hardware::ram;

pub fn collect(simplify: bool) -> Table {
    let mut result = Table::new("Memory");

    let mem_info = ram::fetch_info();
    let ram_usage_percents = percentage(mem_info.total.mb as u64, mem_info.used.mb as u64);
    result.add(
        "RAM",
        format!(
            "{}MiB / {}MiB [{}]",
            mem_info.used.mb,
            mem_info.total.mb,
            if !simplify {
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
    if let Some(swap_info) = mem_info.swap {
        let swap_usage_percents = percentage(swap_info.total.mb as u64, swap_info.used.mb as u64);
        result.add(
            "Swap",
            format!(
                "{}MiB / {}MiB [{}]",
                swap_info.used.mb,
                swap_info.total.mb,
                if !simplify {
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
