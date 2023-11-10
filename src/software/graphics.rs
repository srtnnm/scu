use crate::utils;
use std::env;

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

pub fn detect_de() -> Option<String> {
    for proc in utils::process::list_process() {
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

pub fn detect_wm() -> Option<String> {
    for proc in utils::process::list_process() {
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
        }
        .to_string();

        if !wm.is_empty() {
            if wm != "Hyprland" {
                match utils::get_version(match wm.as_str() {
                    "Mutter" => "mutter",
                    _ => proc.command.as_str(),
                }) {
                    Some(version) => wm.push_str(&format!(" v{version}")),
                    _ => {}
                };
            }
            return Some(wm);
        }
    }

    None
}
