use super::regex_find;

pub fn len(str: &str) -> usize {
    str.replace(&regex_find(r"\x1b\[38;2;\d+;\d+;\d+m", str), "")
        .replace(&regex_find(r"\x1b\[48;2;\d+;\d+;\d+m", str), "")
        .replace("\x1B[0m", "")
        .chars()
        .count()
}
