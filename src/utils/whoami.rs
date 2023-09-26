use crate::utils::libc::{getpwuid,getuid,passwd};
use std::ffi::CStr;

pub fn username() -> Option<String> {
    unsafe {
        let uid = getuid();
        let passwd_ptr = getpwuid(uid);

        if passwd_ptr.is_null() {
            return None;
        }

        let username = CStr::from_ptr((*passwd_ptr).pw_name).to_string_lossy().into_owned();

        Some(username)
    }
}
