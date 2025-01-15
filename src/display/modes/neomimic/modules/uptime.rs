use crate::{display::modes::neomimic::row::DataRow, info::get_option};

use super::ModuleTrait;

pub struct Uptime;

impl ModuleTrait for Uptime {
    const NAME: &'static str = "uptime";

    fn run(info: &crate::info::SystemInformation) -> std::io::Result<usize> {
        let uptime = get_option("uptime", &info.uptime)?;

        let mut value = String::default();
        for (int, name) in [
            (&uptime.days, "days"),
            (&uptime.hours, "hours"),
            (&uptime.minutes, "mins"),
        ] {
            if int > &0 {
                value.push_str(&format!("{int} {name}, "));
            }
        }
        value.pop();
        value.pop();
        Ok(DataRow::info("Uptime", &value))
    }
}
