use crate::config::{no_colors, no_logo};

use std::{
    path::Path,
    sync::atomic::{AtomicUsize, Ordering},
};

use libscu::util::string::extract_u64;
use regex_lite::Regex;

const TUX: &str = "        ${8}#####
       #######
       ##${7}^${8}#${7}^${8}##
       #${3}#####${8}#
     ##${7}##${3}###${7}##${8}##
    #${7}##########${8}##
   #${7}############${8}##
   #${7}############${8}###
  ${3}##${8}#${7}###########${8}##${3}#
######${8}#${7}#######${8}#${3}######
#######${8}#${7}#####${8}#${3}#######
  ${3}#####${8}#######${3}#####";

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
        let readed = std::fs::read_to_string(path)?.trim_end().to_string();

        Ok(Self(readed))
    }
    pub fn print(&self) {
        if no_logo() {
            return;
        }

        let mut logo = self.0.clone();

        let should_use_colors = !no_colors();
        let color_re = Regex::new(r"\$\{(\d+)\}").unwrap();
        if should_use_colors {
            logo = color_re
                .replace_all(&logo, |caps: &regex_lite::Captures| {
                    format!("\x1b[38;5;{color_int}m", color_int = extract_u64(&caps[1]))
                })
                .to_string();
        } else {
            logo = color_re.replace_all(&logo, "").to_string();
        }

        let color_re = Regex::new(r"\x1b\[38;5;\d+m").unwrap();
        LOGO_WIDTH.store(
            logo.lines()
                .map(|line| color_re.replace_all(line, "").chars().count())
                .max()
                .unwrap_or(0),
            Ordering::Relaxed,
        );
        LOGO_HEIGHT.store(logo.lines().count(), Ordering::Relaxed);

        print!("{logo}\x1b[0m");
    }
}
impl Default for Logo {
    fn default() -> Self {
        Self(TUX.to_string())
    }
}
