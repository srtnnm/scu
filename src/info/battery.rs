use crate::data::table::*;
use crate::utils::colorize::colorize_by_num;

use libscu::hardware::battery;

pub fn collect(simplify: bool) -> Table {
    let mut result = Table::new("Battery");

    let batteries = battery::fetch_all();
    if !batteries.is_empty() {
        let bat = batteries.first().unwrap();

        result.add("Model", &bat.model);

        let _ = bat.technology.as_ref().is_some_and(|technology| {
            result.add("Technology", &technology);
            true
        });

        result.add(
            "Capacity",
            format!(
                "{}",
                if !simplify {
                    colorize_by_num(
                        format!("{}%", bat.capacity).as_str(),
                        bat.capacity,
                        100,
                        true,
                    )
                } else {
                    format!("{}%", bat.capacity)
                }
            )
            .as_str(),
        );

        let _ = bat.status.as_ref().is_some_and(|status| {
            result.add("Status", &status);
            true
        });
    }

    result
}
