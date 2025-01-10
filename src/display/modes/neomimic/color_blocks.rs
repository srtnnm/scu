fn colored(text: &str, color: u32) -> String {
    format!("\x1b[{color}m{text}")
}

pub fn print() {
    let mut s = String::default();
    for i in 30..38 {
        s.push_str(&colored("██", i))
    }
    println!("{s}\x1b[0m");
    let mut s = String::default();
    for i in 90..98 {
        s.push_str(&colored("███", i))
    }
    println!("{s}\x1b[0m")
}
