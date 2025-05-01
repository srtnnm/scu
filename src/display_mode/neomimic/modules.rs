use super::display::{Display, Header, RowSenderT, Separator};

use crate::modules::*;

impl Module {
    pub fn run_neomimic(&self, sender: RowSenderT) -> std::io::Result<()> {
        match self {
            Self::Battery => Battery.run(sender),
            Self::CPU => CPU.run(sender),
            Self::DE => DE.run(sender),
            Self::GPU => GPU.run(sender),
            Self::Header => Header.run(sender),
            Self::Device => Device.run(sender),
            Self::Init => Init.run(sender),
            Self::Kernel => Kernel.run(sender),
            Self::Locale => Locale.run(sender),
            Self::Memory => Memory.run(sender),
            Self::OS => OS.run(sender),
            Self::Packages => Packages.run(sender),
            Self::Separator => Separator.run(sender),
            Self::Shell => Shell.run(sender),
            Self::Terminal => Terminal.run(sender),
            Self::Uptime => Uptime.run(sender),
            Self::WM => WM.run(sender),
            Self::Arch => Arch.run(sender),
            Self::Brightness => Brightness.run(sender),
            Self::Disks => Disks.run(sender),
            Self::RootFS => RootFS.run(sender),
            Self::Hostname => Hostname.run(sender),
            Self::Username => Username.run(sender),
            Self::DisplayServer => DisplayServer.run(sender),
        }
    }
}

// TODO: show possible errors for debugging
pub fn run_module(module: &Module, sender: RowSenderT) -> Option<()> {
    module.run_neomimic(sender).ok()
}
