#![cfg(feature = "kernel")]

use crate::utils::libc::{uname, UtsName};

pub fn get_version() -> String {
    unsafe {
        let mut uts: UtsName = std::mem::zeroed();
        uname(&mut uts);

        return String::from_utf8(uts.release.to_vec())
            .unwrap_or("".to_string())
            .replace("\0", "")
            .trim()
            .to_string();
    }
}
