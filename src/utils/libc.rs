pub const TIOCGWINSZ: u64 = 0x5413;
pub const STDOUT_FILENO: i32 = 0;
extern "C" {
    pub fn ioctl(fs: i32, requets: u64, argp: *mut winsize) -> i32;
    pub fn uname(uts: *mut utsname) -> i32;
    pub fn getpwuid(uid: u32) -> *mut passwd;
    pub fn getuid() -> u32;
}
#[repr(C)]
pub struct winsize {
    pub ws_row: u16,
    pub ws_col: u16,
    pub ws_xpixel: u16,
    pub ws_ypixel: u16
}
#[repr(C)]
pub struct utsname {
    pub sysname: [u8; 65],
    pub nodename: [u8; 65],
    pub release: [u8; 65],
    pub version: [u8; 65],
    pub machine: [u8; 65],
    pub domainname: [u8; 65],
}
#[repr(C)]
pub struct passwd {
    pub pw_name: *const i8,
}
