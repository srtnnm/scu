fn print_line(colors: std::ops::Range<u8>) {
    for i in colors {
        print!("\x1b[48;5;{i}m{text}", text = "   ")
    }
    println!("\x1b[0m");
}

pub fn print() {
    print_line(0..8);
    print_line(8..16);
}
