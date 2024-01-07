#![cfg(feature = "whoami")]

use crate::utils::libc::{getpwuid, getuid};
use std::ffi::CStr;

pub fn userid() -> u32 {
    unsafe { getuid() }
}

pub fn username() -> Option<String> {
    unsafe {
        let uid = userid();
        let passwd_ptr = getpwuid(uid);

        if passwd_ptr.is_null() {
            return None;
        }

        let username = CStr::from_ptr((*passwd_ptr).pw_name)
            .to_string_lossy()
            .into_owned();

        Some(username)
    }
}

pub fn home_dir(uid: u32) -> Option<std::path::PathBuf> {
    unsafe {
        let passwd_ptr = getpwuid(uid);

        if passwd_ptr.is_null() {
            return None;
        }

        let dir = CStr::from_ptr((*passwd_ptr).pw_dir)
            .to_string_lossy()
            .into_owned();

        if dir.is_empty() {
            return None;
        }

        Some(std::path::PathBuf::from(dir))
    }
}