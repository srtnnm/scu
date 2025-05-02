use std::io;

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
    (enum_name:$enum_name:tt, $($module:tt = $string:expr,)*) => {
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum $enum_name {
            $($module,)*
        }
        const MODULE_STRING_REPRESENTATION: &[($enum_name, &str)] = &[
            $(
                ($enum_name::$module, $string),
            )*
        ];
    };
}
generate_modules_and_string_representation!(
    enum_name:Module,
    Arch = "arch",
    Battery = "battery",
    Brightness = "brightness",
    CPU = "cpu",
    DE = "de",
    Device = "device",
    Disks = "disks",
    DisplayServer = "display_server",
    GPU = "gpu",
    Header = "header",
    Hostname = "hostname",
    Init = "init",
    Kernel = "kernel",
    Locale = "locale",
    Memory = "memory",
    OS = "os",
    Packages = "packages",
    RootFS = "rootfs",
    Separator = "separator",
    Shell = "shell",
    Terminal = "terminal",
    Uptime = "uptime",
    Username = "username",
    WM = "wm",
);

impl Module {
    pub fn from_str(name: &str) -> Option<Self> {
        for (module, string_representation) in MODULE_STRING_REPRESENTATION {
            if *string_representation == name {
                return Some(*module);
            }
        }
        None
    }
}

pub trait DisplayModule<DisplaySenderT>: Detection {
    fn run(&self, display_sender: &DisplaySenderT) -> io::Result<()> {
        Self::display(self.fetch()?, display_sender);

        Ok(())
    }
    fn display(result: Self::Result, sender: &DisplaySenderT);
}
