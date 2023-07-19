use crate::utils::converter::Size2D;
use crate::utils::process;
use std::process::Command;

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

    // still have a problem with tmux-256color
    // idk how to fix it
    // result == "tmux-256color" doesn't work
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

pub fn get_size() -> Size2D {
    let lines = String::from_utf8(
        Command::new("tput")
            .args(["lines"])
            .output()
            .unwrap()
            .stdout
            .to_ascii_lowercase(),
    )
    .unwrap();
    let lines = lines.trim().parse::<u32>().unwrap();
    let columns = String::from_utf8(
        Command::new("tput")
            .args(["cols"])
            .output()
            .unwrap()
            .stdout
            .to_ascii_lowercase(),
    )
    .unwrap();
    let columns = columns.trim().parse::<u32>().unwrap();

    Size2D::new(columns, lines)
}
