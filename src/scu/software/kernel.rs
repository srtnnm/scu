#![cfg(feature = "kernel")]

use crate::utils::libc::{uname, UtsName};

pub fn get_version() -> String {
    unsafe {
        let mut uts: UtsName = std::mem::zeroed();
        uname(&mut uts);

        return String::from_utf8_lossy(&uts.release).trim().to_string();
    }
}
