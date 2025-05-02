use super::{collect::collect_tables, config::TableConfig, table};

use crate::{config::simplify, util::len};

fn update_max_len(max_len_buf: &mut usize, str: &String, plus_integer: usize) {
    let len = len::len(str).saturating_add(plus_integer);
    if len > *max_len_buf {
        *max_len_buf = len;
    }
}

mod symbols {
    pub const VERTICAL: &str = "│";
    pub const HORIZONTAL: &str = "─";
    pub const TOP_RIGHT: &str = "┐";
    pub const TOP_LEFT: &str = "┌";
    pub const BOTTOM_RIGHT: &str = "┘";
    pub const BOTTOM_LEFT: &str = "└";
    pub const VERTICAL_LEFT: &str = "┤";
    pub const VERTICAL_RIGHT: &str = "├";

    pub const THICK_VERTICAL_RIGHT: &str = "┣";
    pub const THICK_BOTTOM_LEFT: &str = "┗";
}

fn print_formatted_info(tables: &Vec<table::Table>) {
    if tables.is_empty() {
        return;
    }

    let (mut max_entry_name_len, mut max_entry_value_len): (usize, usize) = (0, 0);
    for table in tables {
        for entry in &table.entries {
            update_max_len(&mut max_entry_name_len, &entry.name, 0);
            update_max_len(&mut max_entry_value_len, &entry.value, 0);
            for additional in entry.additional.iter() {
                update_max_len(&mut max_entry_name_len, &additional.name, 1);
                update_max_len(&mut max_entry_value_len, &additional.value, 0);
            }
        }
    }
    let table_width = max_entry_name_len
        .saturating_add(max_entry_value_len)
        .saturating_add(2) // ": ".len()
        .saturating_add(2); // horizontal padding

    for (i, table) in tables.iter().enumerate() {
        if table.is_empty() {
            continue;
        }
        let (left_corner, right_corner) = if i == 0 {
            (symbols::TOP_LEFT, symbols::TOP_RIGHT)
        } else {
            (symbols::VERTICAL_RIGHT, symbols::VERTICAL_LEFT)
        };

        // print table title
        println!(
            "{left_corner}{horizontal} {title} {line}{right_corner}",
            title = table.title,
            horizontal = symbols::HORIZONTAL,
            line = symbols::HORIZONTAL.repeat(
                table_width
                    .saturating_sub(table.title.len())
                    .saturating_sub(3)
            )
        );

        // print entries
        {
            let print_entry = |entry: &table::TableEntry, additional_corner: Option<&str>| {
                println!(
                "{border} {additional_corner}{name}: {name_spacing}{value}{value_spacing} {border}",
                    border = symbols::VERTICAL,
                    additional_corner = additional_corner.unwrap_or_default(),
                    name = entry.name,
                    name_spacing = " ".repeat(
                        max_entry_name_len
                            .saturating_sub(len::len(&entry.name))
                            .saturating_sub(if additional_corner.is_some() { 1 } else { 0 })
                    ),
                    value = entry.value,
                    value_spacing =
                        " ".repeat(max_entry_value_len.saturating_sub(len::len(&entry.value)))
                );
            };
            for entry in &table.entries {
                print_entry(entry, None);

                // print additional entries
                {
                    let number_of_additional_entries = entry.additional.len().saturating_sub(1);
                    for (i, entry) in entry.additional.iter().enumerate() {
                        print_entry(
                            entry,
                            Some(if i == number_of_additional_entries {
                                symbols::THICK_BOTTOM_LEFT
                            } else {
                                symbols::THICK_VERTICAL_RIGHT
                            }),
                        );
                    }
                }
            }
        }
    }

    // print table end line
    println!(
        "{left_corner}{line}{right_corner}",
        left_corner = symbols::BOTTOM_LEFT,
        line = symbols::HORIZONTAL.repeat(table_width),
        right_corner = symbols::BOTTOM_RIGHT
    );
}

fn print_simplified(tables: &Vec<table::Table>) {
    for table in tables {
        println!("- {title}", title = table.title);
        for entry in &table.entries {
            println!("{name}: {value}", name = entry.name, value = entry.value);
            for additional in &entry.additional {
                println!(
                    "  {name}: {value}",
                    name = additional.name,
                    value = additional.value
                )
            }
        }
    }
}

pub(super) fn print(config: &TableConfig) {
    let tables = collect_tables(config);
    if simplify() {
        print_simplified(&tables);
    } else {
        print_formatted_info(&tables);
    }
}
