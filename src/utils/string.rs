pub fn remove_multiple_spaces(_str: String) -> String {
    _str.split(' ')
        .filter(|s| !s.is_empty())
        .fold(String::new(), |a, b| a + b + " ")
        .trim()
        .to_string()
}
