use std::sync::atomic::{AtomicU16, AtomicUsize, Ordering};

use regex_lite::Regex;

const TUX: &str = "        $8#####
       $8#######
       $8##$7^$8#$7^$8##
       $8#$3#####$8#
     $8##$7##$3###$7##$8##
    $8#$7##########$8##
   $8#$7############$8##
   $8#$7############$8###
  $3##$8#$7###########$8##$3#
$3######$8#$7#######$8#$3######
$3#######$8#$7#####$8#$3#######
  $3#####$8#######$3#####";
// pub const TUX_WIDTH: usize = 21;
// pub const TUX_HEIGHT: usize = 12;

static LOGO_WIDTH: AtomicUsize = AtomicUsize::new(0);
pub fn logo_width() -> usize {
    LOGO_WIDTH.load(Ordering::Relaxed)
}
static LOGO_HEIGHT: AtomicUsize = AtomicUsize::new(0);
pub fn logo_height() -> usize {
    LOGO_HEIGHT.load(Ordering::Relaxed)
}

pub fn print_logo() {
    let mut linux_logo = TUX.to_string();

    let color_re = Regex::new(r"\$\d").unwrap();
    for color in color_re.find_iter(&linux_logo.clone()) {
        if let Some(color_int) = color
            .as_str()
            .strip_prefix("$")
            .and_then(|color| color.parse::<u64>().ok())
        {
            linux_logo = linux_logo.replace(color.as_str(), &format!("\x1b[38;5;{color_int}m"));
        }
    }

    print!("{linux_logo}");
    println!("\x1b[0m");
}

pub struct Logo(String);
impl Logo {
    pub fn print(&self) {
        let mut logo = self.0.clone();

        let color_re = Regex::new(r"\$\d").unwrap();
        for color in color_re.find_iter(&logo.clone()) {
            if let Some(color_int) = color
                .as_str()
                .strip_prefix("$")
                .and_then(|color| color.parse::<u64>().ok())
            {
                logo = logo.replace(color.as_str(), &format!("\x1b[38;5;{color_int}m"));
            }
        }

        let color_re = Regex::new(r"\x1b\[38;5;\dm").unwrap();
        LOGO_WIDTH.store(
            logo.lines()
                .map(|line| color_re.replace_all(line, ""))
                .map(|line| line.chars().count())
                .max()
                .unwrap_or(0),
            Ordering::Relaxed,
        );
        LOGO_HEIGHT.store(logo.lines().count(), Ordering::Relaxed);

        print!("{logo}");
        print!("\x1b[0m");
    }
}
impl Default for Logo {
    fn default() -> Self {
        Self(TUX.to_string())
    }
}
