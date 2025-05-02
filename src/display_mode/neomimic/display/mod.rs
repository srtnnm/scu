mod hardware;
pub use hardware::*;
mod misc;
pub use misc::*;
mod software;
pub use software::*;

use crate::display_mode::DisplaySender;

use std::sync::atomic::AtomicUsize;

pub type RowSenderT = DisplaySender<usize, String>;
