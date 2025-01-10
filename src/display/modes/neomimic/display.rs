use super::modules::{Header, Module};
use super::row::DataRow;

pub fn display(info: &crate::info::SystemInformation) {
    let mut rows: Vec<DataRow> = Vec::new();

    rows.push(Header::get(info).unwrap());

    for row in rows {
        println!("{}", row.to_string());
    }
}
