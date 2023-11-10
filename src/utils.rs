pub mod ascii_art;
pub mod converter;
pub mod distro_colors;
pub mod fs;
pub mod libc;
pub mod process;
pub mod string;
pub mod whoami;

pub fn get_version(program: &str) -> Option<String> {
    let arg = match program {
        "dwm" => "-v",
        _ => "--version",
    };
    let output = match std::process::Command::new(program).arg(arg).output() {
        Ok(output) => String::from_utf8(if ["dwm", "ksh"].contains(&program) {
            output.stderr
        } else {
            output.stdout
        })
        .unwrap_or("".to_string()),
        Err(_) => String::from(""),
    };

    return match regex::Regex::new(r"(\d+\.?)+")
        .unwrap()
        .find(output.as_str())
    {
        Some(_str) => Some(_str.as_str().to_string()),
        None => None,
    };
}

pub fn percentage(max: u64, cur: u64) -> f32 {
    ((cur as f64 / max as f64) * 100.0) as f32
}
