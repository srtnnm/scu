pub fn percentage(max: u64, cur: u64) -> f32 {
    ((cur as f64 / max as f64) * 100.0) as f32
}

pub fn uniqalize(_str: String) -> String {
    let mut buf = Vec::<&str>::new();
    for word in _str.split_whitespace() {
        if !buf.contains(&word) {
            buf.push(word);
        }
    }
    buf.iter().fold(String::new(), |a, b| a + b + " ")
}
