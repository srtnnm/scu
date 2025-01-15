use crate::display_mode::neomimic::logo::TUX_WIDTH;

fn print_line(colors: std::ops::Range<u8>) {
    print!("\x1b[{}C", TUX_WIDTH + 4);
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
