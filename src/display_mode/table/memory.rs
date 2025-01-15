use crate::{
    data::table::Table,
    info,
    util::{colorize::colorize_by_num, percentage},
};

pub fn to_table(info: &info::SystemInformation, disable_color: bool) -> Option<Table> {
    let ram_info = info.ram.clone()?;

    let mut result = Table::new("Memory");

    let ram_usage_percents = percentage(ram_info.total.mb as u64, ram_info.used.mb as u64);
    result.add(
        "RAM",
        &format!(
            "{}MiB / {}MiB [{}]",
            ram_info.used.mb,
            ram_info.total.mb,
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
        ),
    );
    if let Some(swap_info) = ram_info.swap {
        let swap_usage_percents = percentage(swap_info.total.mb as u64, swap_info.used.mb as u64);
        result.add(
            "Swap",
            &format!(
                "{}MiB / {}MiB [{}]",
                swap_info.used.mb,
                swap_info.total.mb,
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
            ),
        );
    }

    Some(result)
}
