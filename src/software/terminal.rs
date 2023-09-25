use crate::utils::converter::Size2D;
use crate::utils::process;
use crate::utils::libc::{ioctl, winsize, STDOUT_FILENO, TIOCGWINSZ};
use std::path::Path;

fn bin_to_name(bin_name: String) -> String {
    String::from(match bin_name.as_str() {
        "alacritty" => "Alacritty",
        "deepin-terminal" => "Deepin Terminal",
        "foot" => "Foot",
        "gnome-terminal-" => "GNOME Terminal",
        "konsole" => "Konsole",
        "lxterminal" => "LXTerminal",
        "st" => "ST",
        "xfce4-terminal" => "XFCE4 Terminal",
        "kitty" => "Kitty",
        _ => "",
    })
}

pub fn get_name() -> String {
    let mut result = String::from("Linux");

    // still doesn't work from tmux
    let mut ppid = process::get_ppid(process::get_pid()).unwrap();
    while ppid != 1 {
        let info = process::get_info(ppid);
        if info.is_err() {
            break;
        }
        let info = info.unwrap();
        let got_name = bin_to_name(info.command);
        if !got_name.is_empty() {
            return got_name;
        } else {
            ppid = process::get_ppid(ppid).unwrap();
        }
    }

    if Path::new("/data/data/com.termux/files/home/.termux").exists() {
        result = String::from("Termux");
    }

    result
}

pub fn get_size() -> Option<Size2D> {
    let mut nix_size = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };

    if unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ.into(), &mut nix_size) } == 0 {
        return Some(Size2D {
            width: nix_size.ws_col as usize,
            height: nix_size.ws_row as usize,
        });
    }

    None
}
