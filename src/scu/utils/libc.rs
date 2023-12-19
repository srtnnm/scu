pub const TIOCGWINSZ: u64 = 0x5413;
pub const STDIN_FILENO: i32 = 0;
pub const STDOUT_FILENO: i32 = 1;
extern "C" {
    pub fn ioctl(fs: i32, requets: u64, argp: *mut WinSize) -> i32;
    pub fn uname(uts: *mut UtsName) -> i32;
    pub fn getpwuid(uid: u32) -> *mut Passwd;
    pub fn getuid() -> u32;
    pub fn isatty(fd: i32) -> i8;
}
#[repr(C)]
pub struct WinSize {
    pub ws_row: u16,
    pub ws_col: u16,
    pub ws_xpixel: u16,
    pub ws_ypixel: u16,
}
#[repr(C)]
pub struct UtsName {
    pub sysname: [u8; 65],
    pub nodename: [u8; 65],
    pub release: [u8; 65],
    pub version: [u8; 65],
    pub machine: [u8; 65],
    pub domainname: [u8; 65],
}
#[repr(C)]
pub struct Passwd {
    pub pw_name: *const std::os::raw::c_char,
}
