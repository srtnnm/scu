use crate::config::{no_colors, no_logo};

use std::{
    path::Path,
    sync::atomic::{AtomicUsize, Ordering},
};

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

static LOGO_WIDTH: AtomicUsize = AtomicUsize::new(0);
pub fn logo_width() -> usize {
    LOGO_WIDTH.load(Ordering::Relaxed)
}
static LOGO_HEIGHT: AtomicUsize = AtomicUsize::new(0);
pub fn logo_height() -> usize {
    LOGO_HEIGHT.load(Ordering::Relaxed)
}

pub struct Logo(String);
impl Logo {
    pub fn from_path(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let readed = std::fs::read_to_string(path)?;

        Ok(Self(readed))
    }
    pub fn print(&self) {
        if no_logo() {
            return;
        }

        let mut logo = self.0.clone();

        let color_re = Regex::new(r"\$\d").unwrap();
        for color in color_re.find_iter(&logo.clone()) {
            if let Some(color_int) = color
                .as_str()
                .strip_prefix("$")
                .and_then(|color| color.parse::<u64>().ok())
            {
                if no_colors() {
                    logo = logo.replace(color.as_str(), "");
                } else {
                    logo = logo.replace(color.as_str(), &format!("\x1b[38;5;{color_int}m"));
                }
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
