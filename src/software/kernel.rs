use crate::utils::libc::{uname, utsname};

pub fn get_version() -> String {
    unsafe {
        let mut uts: utsname = std::mem::zeroed();
        uname(&mut uts);

        return String::from_utf8_lossy(&uts.release).trim().to_string();
    }
}
