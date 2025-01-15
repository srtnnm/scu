use super::display::cursor_mover;

pub(crate) struct DataRow;

impl DataRow {
    pub fn info(name: &str, value: &str) -> usize {
        cursor_mover();
        println!("{name}: {value}");
        name.len().saturating_add(value.len()).saturating_add(2)
    }
    pub fn nameless(value: &str) -> usize {
        cursor_mover();
        println!("{value}");
        value.len()
    }
    pub fn separator(r#char: char) -> usize {
        cursor_mover();
        let len = super::display::LAST_ROW_LENGTH.load(std::sync::atomic::Ordering::Relaxed);
        println!("{}", r#char.to_string().repeat(len));
        len
    }
}
