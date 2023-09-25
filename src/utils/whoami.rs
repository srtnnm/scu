use crate::utils::libc::getlogin;
use std::ffi::CStr;

pub fn username() -> String {
    unsafe {
        let login_ptr = getlogin();
        let login = CStr::from_ptr(login_ptr);
        return String::from(login.to_str().unwrap());
    }
}
