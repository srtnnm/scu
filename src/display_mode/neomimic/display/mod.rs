mod hardware;
mod misc;
pub use misc::*;
mod software;

use crate::display_mode::DisplaySender;

pub type RowSenderT = DisplaySender<usize, String>;
