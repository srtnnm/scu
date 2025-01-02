pub mod colorize;
pub mod len;

pub fn percentage(max: u64, cur: u64) -> f32 {
    ((cur as f64 / max as f64) * 100.0) as f32
}

pub fn regex_find(re: &str, s: &str) -> String {
    match regex_lite::Regex::new(re).unwrap().find(s) {
        Some(_match) => _match.as_str().to_string(),
        None => "".to_string(),
    }
}
