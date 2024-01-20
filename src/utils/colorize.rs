pub fn text(str: &str, r: u16, g: u16, b: u16) -> String {
    format!("\x1b[38;2;{r};{g};{b}m{str}\x1B[0m")
}

pub fn colorize_by_num(string: &str, num_cur: u16, num_max: u16, more_is_better: bool) -> String {
    let mut gradient: [(u16, u16, u16); 5] = [
        (0, 255, 0),    // green
        (178, 214, 63), // idk
        (255, 255, 0),  // yellow
        (255, 165, 0),  // orange
        (255, 0, 0),    // red
    ];
    if more_is_better {
        gradient.reverse()
    }
    let elem = (crate::utils::percentage(num_max as u64, num_cur as u64) as u16 / 20) as usize;
    let (r, g, b) = gradient[if elem > 4 { 4 } else { elem }];

    text(string, r, g, b)
}

pub fn colorize_background(str: &str, r: u16, g: u16, b: u16) -> String {
    let mut result = format!("\x1b[48;2;{r};{g};{b}m{str}\x1B[0m");
    if (r + g + b) / 3 > 123 {
        result = text(&result, 0, 0, 0);
    } else {
        // avoid terminal's text color
        result = text(&result, 255, 255, 255);
    }
    result
}
