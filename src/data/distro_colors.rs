#[derive(Clone, Copy)]
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

pub static COLORS: [(&str, Color); 11] = [
    ("Android", Color::new(61, 220, 132)),
    ("Arch", Color::new(23, 147, 209)),
    ("Artix", Color::new(23, 147, 209)),
    ("Debian", Color::new(206, 0, 86)),
    ("EndeavourOS", Color::new(127, 63, 191)),
    ("Fedora", Color::new(60, 110, 180)),
    ("Gentoo", Color::new(84, 72, 122)),
    ("Linux Mint", Color::new(146, 182, 98)),
    ("Manjaro", Color::new(52, 190, 91)),
    ("NixOS", Color::new(126, 186, 228)),
    ("Ubuntu", Color::new(233, 84, 32)),
];

pub fn get_color(distro_name: &str) -> Option<Color> {
    COLORS
        .iter()
        .find(|(distro, _)| distro_name.contains(distro))
        .map(|(_, color)| *color)
}
