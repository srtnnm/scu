pub struct Color {
    pub r: u16,
    pub g: u16,
    pub b: u16,
}

impl Color {
    pub const fn new(r: u16, g: u16, b: u16) -> Color {
        Color { r, g, b }
    }
}

pub static COLORS: [(&str, (u16, u16, u16)); 11] = [
    ("Android", (61, 220, 132)),
    ("Arch", (23, 147, 209)),
    ("Artix", (23, 147, 209)),
    ("Debian", (206, 0, 86)),
    ("EndeavourOS", (127, 63, 191)),
    ("Fedora", (60, 110, 180)),
    ("Gentoo", (84, 72, 122)),
    ("Linux Mint", (146, 182, 98)),
    ("Manjaro", (52, 190, 91)),
    ("NixOS", (126, 186, 228)),
    ("Ubuntu", (233, 84, 32)),
];

pub fn get_color(distro_name: &str) -> Option<Color> {
    for (distro, (r, g, b)) in &COLORS {
        if distro_name.contains(distro) {
            return Some(Color::new(*r, *g, *b));
        }
    }
    None
}
