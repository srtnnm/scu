mod battery;
mod drives;
mod graphics;
mod memory;
mod packages;
mod processor;
mod system;

use std::collections::BTreeMap;

use crate::config::Config;
use crate::data::table::*;
use crate::utils::{self, len};

fn collect_info(
    cfg: Vec<String>,
    simplify_output: bool,
    force_version: bool,
) -> BTreeMap<String, Table> {
    let mut result: BTreeMap<String, Table> = BTreeMap::new();

    if cfg.contains(&"system".to_string()) {
        result.insert(
            "system".to_string(),
            system::collect(simplify_output, force_version),
        );
    }

    if cfg.contains(&"packages".to_string()) {
        result.insert("packages".to_string(), packages::collect());
    }

    if cfg.contains(&"processor".to_string()) {
        result.insert("processor".to_string(), processor::collect(simplify_output));
    }

    if cfg.contains(&"memory".to_string()) {
        result.insert("memory".to_string(), memory::collect(simplify_output));
    }

    if cfg.contains(&"battery".to_string()) {
        result.insert("battery".to_string(), battery::collect(simplify_output));
    }

    if cfg.contains(&"drives".to_string()) {
        result.insert("drives".to_string(), drives::collect());
    }

    if cfg.contains(&"graphics".to_string()) {
        result.insert(
            "graphics".to_string(),
            graphics::collect(simplify_output, force_version),
        );
    }

    result
}

fn formatted_info(
    cfg: Config,
    simplify_output: bool,
    force_version: bool,
) -> Vec<(String, Vec<String>)> {
    let tables = collect_info(cfg.order.clone(), simplify_output, force_version);
    let mut result: Vec<(String, Vec<String>)> = Vec::new();

    let max_param_len = len::param_max_len(tables.clone().into_iter().map(|elm| elm.1).collect());

    for table in cfg.order {
        if !tables.contains_key(&table) {
            continue;
        }
        let table = tables.get(&table).unwrap().clone();
        let mut table_buf = Vec::<String>::new();
        if table.is_empty() {
            continue;
        }
        for entry in table.entries {
            let line_val = utils::uniqalize(entry.value);
            if !line_val.contains("Unknown") && line_val != "0" {
                table_buf.push(format!(
                    "{}:{}{line_val}",
                    &entry.name,
                    " ".repeat(if !simplify_output {
                        max_param_len - len::len(&entry.name) + 2
                    } else {
                        1
                    }),
                ));
            }
            for additional in entry.additional.clone().into_iter().enumerate() {
                let (i, additional) = additional;
                let add_line_val = utils::uniqalize(additional.value);
                if !add_line_val.contains("Unknown") && add_line_val != "0" {
                    table_buf.push(format!(
                        "{}{}:{}{}",
                        if simplify_output {
                            "  "
                        } else {
                            if i == entry.additional.len() - 1 {
                                "┗"
                            } else {
                                "┣"
                            }
                        },
                        &additional.name,
                        " ".repeat(if !simplify_output {
                            max_param_len - len::len(&additional.name) + 1
                        } else {
                            1
                        }),
                        add_line_val
                    ));
                }
            }
        }
        result.push((table.title, table_buf));
    }

    result
}

pub fn print_info(cfg: Config, simplify_output: bool, force_version: bool) {
    let info = formatted_info(cfg, simplify_output, force_version);

    if !simplify_output {
        let max_len = info
            .clone()
            .into_iter()
            .map(|e| len::max_len(e.1))
            .max()
            .unwrap();
        let mut to_display: Vec<String> = Vec::new();
        for table in info.clone().into_iter().enumerate() {
            to_display.push(format!(
                "{}─┤ {} ├{}{}",
                if table.0 == 0 { "┌" } else { "├" },
                table.1 .0,
                "─".repeat(max_len - len::len(&table.1 .0) - 4),
                if table.0 == 0 { "┐" } else { "┤" }
            ));
            for entry in table.1 .1 {
                to_display.push(format!(
                    "│ {}{}│",
                    entry,
                    " ".repeat(max_len - len::len(entry.as_str())),
                ));
            }
        }
        to_display.push(format!("└{}┘", "─".repeat(max_len + 1)));

        // Display info
        to_display.into_iter().for_each(|l| println!("{l}"));
    } else {
        for table in info {
            println!("- {}", table.0);
            for entry in table.1 {
                println!("{}", entry);
            }
        }
    }
}
