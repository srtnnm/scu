pub mod hardware;
pub mod software;

use super::table::TableEntry;
use crate::{display_mode::DisplaySender, modules::DisplayModule};

pub type DisplaySenderT = DisplaySender<(usize, usize), TableEntry>;
