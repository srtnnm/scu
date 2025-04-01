use crate::display_mode::neomimic::logo::logo_width;

fn print_line(colors: std::ops::Range<u8>) {
    print!("\x1b[{}C", logo_width() + 4);
    for i in colors {
        print!("\x1b[48;5;{i}m{text}", text = "   ")
    }
    println!("\x1b[0m");
}

pub fn print() {
    println!();
    print_line(0..8);
    print_line(8..16);
}
