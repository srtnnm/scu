#![cfg(feature = "extract_version")]

pub fn extract_version(program: &str) -> Option<String> {
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
