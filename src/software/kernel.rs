extern crate libc;
use std::ffi::CStr;

pub fn get_version() -> String {
    unsafe {
        let mut uts: libc::utsname = std::mem::zeroed();
        libc::uname(&mut uts);

        return CStr::from_ptr(uts.release.as_ptr())
            .to_bytes()
            .iter()
            .map(|&c| c as char)
            .collect::<String>();
    }
}
