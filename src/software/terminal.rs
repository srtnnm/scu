use crate::utils::converter::Size2D;
use crate::utils::process;
use libc::{c_ushort, ioctl, STDOUT_FILENO, TIOCGWINSZ};

fn bin_to_name(bin_name: String) -> String {
    String::from(match bin_name.as_str() {
        "alacritty" => "Alacritty",
        "deepin-terminal" => "Deepin Terminal",
        "foot" => "Foot",
        "gnome-terminal" => "GNOME Terminal",
        "konsole" => "Konsole",
        "lxterminal" => "LXTerminal",
        "st" => "ST",
        "xfce4-terminal" => "XFCE4 Terminal",
        "kitty" => "Kitty",
        _ => "",
    })
}

pub fn get_name() -> String {
    let mut result = String::from("Unknown");

    // still doesn't work from tmux
    let mut ppid = process::get_ppid(process::get_pid()).unwrap();
    while ppid != 1 {
        let info = process::get_info(ppid).unwrap();
        let got_name = bin_to_name(info.command);
        if !got_name.is_empty() {
            result = got_name;
            break;
        } else {
            ppid = process::get_ppid(ppid).unwrap();
        }
    }

    result
}

#[repr(C)]
#[derive(Debug)]
pub struct UnixSize {
    pub rows: c_ushort,
    pub cols: c_ushort,
    x: c_ushort,
    y: c_ushort,
}

pub fn get_size() -> Option<Size2D> {
    let nix_size = UnixSize {
        rows: 0,
        cols: 0,
        x: 0,
        y: 0,
    };

    if unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ.into(), &nix_size) } == 0 {
        return Some(Size2D {
            width: nix_size.cols as u32,
            height: nix_size.rows as u32,
        });
    }

    None
}
