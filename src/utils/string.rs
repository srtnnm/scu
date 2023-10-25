pub fn remove_multiple_spaces(_str: String) -> String {
    _str.trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}
