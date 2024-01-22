pub mod colorize;
pub mod len;

pub fn percentage(max: u64, cur: u64) -> f32 {
    ((cur as f64 / max as f64) * 100.0) as f32
}

pub fn uniqalize(_str: String) -> String {
    let mut buf = Vec::<&str>::new();
    for word in _str.split_whitespace() {
        if word.parse::<i64>().is_ok() || !buf.contains(&word) {
            buf.push(word);
        }
    }
    buf.iter().fold(String::new(), |a, b| a + b + " ")
}

pub fn regex_find(re: &str, s: &str) -> String {
    match regex_lite::Regex::new(re).unwrap().find(s) {
        Some(_match) => _match.as_str().to_string(),
        None => "".to_string(),
    }
}
