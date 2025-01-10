#[derive(Clone)]
pub(crate) struct DataRow(DataRowType);

#[derive(Clone)]
enum DataRowType {
    Information { name: String, value: String },
    Nameless { value: String },
    Separator(char),
}

impl DataRow {
    pub fn info(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self(DataRowType::Information {
            name: name.into(),
            value: value.into(),
        })
    }
    pub fn nameless(value: impl Into<String>) -> Self {
        Self(DataRowType::Nameless {
            value: value.into(),
        })
    }
    pub fn separator(r#char: char) -> Self {
        Self(DataRowType::Separator(r#char))
    }
}

use std::fmt;
impl fmt::Display for DataRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.0.clone() {
                DataRowType::Information { name, value } => {
                    format!("{name}: {value}")
                }
                DataRowType::Nameless { value } => value,
                DataRowType::Separator(c) => c.to_string(),
            }
        )
    }
}
