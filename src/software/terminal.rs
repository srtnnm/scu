use crate::utils::process;
use std::env;

fn bin_to_name(bin_name: String) -> Option<String> {
    let result = String::from(match bin_name.as_str() {
        "alacritty" => "Alacritty",
        "deepin-terminal" => "Deepin Terminal",
        "foot" => "Foot",
        "gnome-terminal" => "GNOME Terminal",
        "konsole" => "Konsole",
        "lxterminal" => "LXTerminal",
        "st" => "ST",
        "xfce4-terminal" => "XFCE4 Terminal",
        "xterm-kitty" => "Kitty",
        _ => "",
    });

    if result != "" {
        Some(result)
    } else {
        None
    }
}

pub fn get_name() -> String {
    let mut result = env::var("TERM").unwrap_or_else(|_| String::from("Unknown"));

    // still have a problem with tmux-256color
    // idk how to fix it
    // result == "tmux-256color" doesn't work
    if result == "xterm-256color" {
        let mut ppid = process::get_ppid(process::get_pid()).unwrap();
        while ppid != 1 {
            let info = process::get_info(ppid).unwrap();
            match bin_to_name(info.command) {
                Some(name) => {
                    result = name.to_string();
                    break;
                }
                None => {
                    ppid = process::get_ppid(ppid).unwrap();
                }
            }
        }
    }

    result
}
