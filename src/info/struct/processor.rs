use crate::{
    data::table::Table,
    utils::{colorize::colorize_by_num, percentage},
};

use libscu::types::Frequency;

#[derive(Default)]
pub struct Processor {
    pub model: String,
    pub cores: u8,
    pub threads: u8,
    pub frequency: Frequency,
    pub temperature: Option<f32>,
}

impl Processor {
    pub fn to_print(&self, disable_color: bool) -> Table {
        let mut result = Table::new("Processor");

        result.add("Model", format!("{}", self.model).as_str());
        result.add(
            "Frequency",
            format!("{:.2}GHz", self.frequency.ghz).as_str(),
        );
        if self.cores > 0 {
            result.add(
                "Computing units",
                format!("{} Cores / {} Threads", self.cores, self.threads).as_str(),
            );
        }
        if let Some(temp) = self.temperature {
            result.add(
                "Temperature",
                if !disable_color {
                    colorize_by_num(
                        format!("{:.1}°C", temp).as_str(),
                        percentage(90, temp as u64) as u16,
                        100,
                        false,
                    )
                } else {
                    format!("{:.1}°C", temp)
                }
                .as_str(),
            );
        }

        result
    }
}
