pub struct Color {
    pub r: u16,
    pub g: u16,
    pub b: u16,
}

impl Color {
    pub const fn new(r: u16, g: u16, b: u16) -> Color {
        Color {
            r:r,
            g:g,
            b:b
        }
    }
}

pub static COLORS: [(&str, (u16,u16,u16)); 2] = [
    ("Android", (61,220,132)),
    ("Gentoo", (102,2,60)),
];

pub fn get_color(distro_name: &str) -> Option<Color> {
    for (distro,(r,g,b)) in &COLORS {
        if distro == &distro_name {
            return Some(Color::new(*r,*g,*b));
        }
    }
    None
}
