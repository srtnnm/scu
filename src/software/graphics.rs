use crate::utils::process;
use std::process::Command;
use std::env;
use regex::Regex;

pub fn get_session_type() -> Option<String> {
    return match env::var("XDG_SESSION_TYPE")
        .unwrap_or("".to_string())
        .as_str()
    {
        "wayland" => Some("Wayland".to_string()),
        "x11" => Some("Xorg".to_string()),
        _ => None,
    };
}

fn extract_version(program_output: String) -> Option<String> {
    if program_output.is_empty() { return None; }

    let version = match Regex::new(r"(\d+\.?)+").unwrap().find(program_output.as_str()) {
        Some(_str) => { _str.as_str() },
        _ => { "" }
    };

    if !version.is_empty() {
        return Some(version.to_string());
    }
    None
}

pub fn detect_de() -> Option<String> {
    for proc in process::list_process() {
        let de = match proc.command.as_str() {
            "gnome-shell" => "GNOME",
            "plasmashell" => "KDE Plasma",
            "xfce4-session" => "XFCE4",
            "cinnamon-sessio" => "Cinnamon",
            _ => "",
        };

        if !de.is_empty() {
            return Some(de.to_string());
        }
    }

    None
}

fn get_wm_version(proc_command: &str) -> Option<String> {
    let mut arg: String = "--version".to_string();

    if proc_command.is_empty() || proc_command == "Hyprland" {
        return None;
    }

    let binary = match proc_command {
        "mutter-x11-fram" => { "mutter" },
        "dwm" => { arg = "-v".to_string(); "dwm" },
        _ => { proc_command }
    };

    let output = match Command::new(binary)
        .arg(arg.as_str())
        .output() {
            Ok(output) => String::from_utf8(if binary == "dwm" { output.stderr } else { output.stdout }).unwrap(),
            Err(_) => String::from("")
        };

    extract_version(output)
}

pub fn detect_wm() -> Option<String> {
    for proc in process::list_process() {
        let mut wm = match proc.command.as_str() {
            "mutter-x11-fram" => "Mutter", // max /proc/x/comm content lenght is 16 (irl 15)
            "kwin_x11" | "kwin_wayland" => "KWin",
            "xfwm4" => "XFWM4",
            "openbox" => "Openbox",
            "sway" => "Sway",
            "i3" => "i3",
            "dwm" => "DWM",
            "Hyprland" => "Hyprland",
            _ => "",
        }.to_string();

        if !wm.is_empty() {
            let version = match get_wm_version(proc.command.as_str()) {
                Some(version) => version,
                _ => "".to_string()
            };
            if !version.is_empty() {
                wm = format!("{} v{}", wm, version);
            }
            return Some(wm);
        }
    }

    None
}
