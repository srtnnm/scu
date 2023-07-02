use which::which;

pub fn _which(name: &str) -> String {
    which(name)
        .unwrap_or("".into())
        .into_os_string()
        .into_string()
        .unwrap()
}
