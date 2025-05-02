pub mod hardware;
pub mod software;

use crate::{data::table::TableEntry, display_mode::DisplaySender, modules::DisplayModule};

pub type DisplaySenderT = DisplaySender<(usize, usize), TableEntry>;
