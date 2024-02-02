use crate::data::table::*;
use crate::utils::colorize::colorize_by_num;

use libscu::hardware::battery;

pub fn collect(simplify: bool) -> Table {
    let mut result = Table::new("Battery");

    let batteries = battery::fetch_batteries();
    if !batteries.is_empty() {
        let bat = batteries.first().unwrap();
        let (_, _, _, _) = (
            bat.model.as_ref().is_some_and(|model| {
                result.add("Model", &model);
                true
            }),
            bat.technology.as_ref().is_some_and(|technology| {
                result.add("Technology", &technology);
                true
            }),
            bat.capacity.is_some_and(|capacity| {
                result.add(
                    "Capacity",
                    format!(
                        "{}",
                        if !simplify {
                            colorize_by_num(format!("{}%", capacity).as_str(), capacity, 100, true)
                        } else {
                            format!("{}%", capacity)
                        }
                    )
                    .as_str(),
                );
                true
            }),
            bat.status.as_ref().is_some_and(|status| {
                result.add("Status", &status);
                true
            }),
        );
    }

    result
}
