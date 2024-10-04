use super::regex_find;

use crate::data::table::*;

pub fn len(str: &str) -> usize {
    //                        \x1b\[38;2;{r};{g};{b}m
    str.replace(&regex_find(r"\x1b\[38;2;\d+;\d+;\d+m", str), "")
        .replace(&regex_find(r"\x1b\[48;2;\d+;\d+;\d+m", str), "")
        .replace("\x1B[0m", "")
        .chars()
        .count()
}

pub fn max_len(strs: Vec<String>) -> usize {
    strs.into_iter().map(|s| len(&s)).max().unwrap_or(0)
}

pub fn param_max_len(tables: Vec<Table>) -> usize {
    let mut result: usize = 0;
    if tables.is_empty() {
        return result;
    }

    for table in tables {
        for entry in table.entries {
            let _len = len(&entry.name);
            if _len > result {
                result = _len;
            }
            for additional_entry in entry.additional {
                let _len = len(&additional_entry.name) + 1;
                if _len > result {
                    result = _len;
                }
            }
        }
    }

    result
}
