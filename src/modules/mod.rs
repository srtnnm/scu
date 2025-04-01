macro_rules! export_modules {
    ( $( $x:ident ),* ) => {
        $(
            mod $x;
            pub use $x::*;
        )*
    };
}

export_modules!(
    arch,
    battery,
    brightness,
    cpu,
    de,
    device,
    disks,
    display_server,
    gpu,
    hostname,
    init,
    kernel,
    locale,
    os,
    packages,
    ram,
    rootfs,
    shell,
    terminal,
    uptime,
    username,
    wm
);

pub fn get_option<T>(variable_name: &str, variable: &Option<T>) -> std::io::Result<T>
where
    T: Clone,
{
    let Some(var) = variable.clone() else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("failed to determine {variable_name}"),
        ));
    };
    Ok(var)
}

pub trait Detection {
    type Result;
    const NAME: &'static str;

    fn fetch(&self) -> std::io::Result<Self::Result>;
}

macro_rules! generate_modules_and_string_representation {
    ($enum_name:tt, $($module:tt => $string:expr,)*) => {
        const NUMBER_OF_MODULES: usize = {
            let mut count = 0;
            $(
                let _ = stringify!($module);
                count += 1;
            )*
            count
        };
        #[derive(Debug, PartialEq)]
        pub enum $enum_name {
            $($module,)*
        }
        const MODULE_STRING_REPRESENTATION: [($enum_name, &str); NUMBER_OF_MODULES] = [
            $(
                ($enum_name::$module, $string),
            )*
        ];
    };
}
generate_modules_and_string_representation!(
    Module,

    Arch => "arch",
    Battery => "battery",
    Brightness => "brightness",
    CPU => "cpu",
    DE => "de",
    Device => "device",
    Disks => "disks",
    DisplayServer => "display_server",
    GPU => "gpu",
    Header => "header",
    Hostname => "hostname",
    Init => "init",
    Kernel => "kernel",
    Locale => "locale",
    Memory => "memory",
    OS => "os",
    Packages => "packages",
    RootFS => "rootfs",
    Separator => "separator",
    Shell => "shell",
    Terminal => "terminal",
    Uptime => "uptime",
    Username => "username",
    WM => "wm",
);

impl Module {
    pub fn from_str(name: &str) -> Option<Self> {
        for (module, string_representation) in MODULE_STRING_REPRESENTATION {
            if string_representation.starts_with(name) {
                return Some(module);
            }
        }
        None
    }
    pub fn to_str(&self) -> &'static str {
        for (module, string_representation) in MODULE_STRING_REPRESENTATION.iter() {
            if module == self {
                return string_representation;
            }
        }
        panic!("string representation for {self:?} not found in MODULE_STRING_REPRESENT")
    }
}
