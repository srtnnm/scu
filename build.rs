use std::process::{Command, Stdio};

fn get_metadata() -> Option<String> {
    let output = Command::new("cargo")
        .args(["metadata", "--format-version=1", "--no-deps"])
        .stdout(Stdio::piped())
        .output()
        .ok()?;

    Some(
        String::from_utf8(output.stdout)
        .ok()?
        .split("\"dependencies\":[{\"name\":\"libscu\",\"source\":\"registry+https://github.com/rust-lang/crates.io-index\",\"req\":\"^")
        .nth(1)
        .map(|s|s.to_string())?
        .split("\"")
        .next()
        .map(|s|s.to_string())?
    )
}

fn main() {
    if let Some(v) = get_metadata() {
        println!("cargo::rustc-env=LIBSCU_VERSION={v}")
    }
}
