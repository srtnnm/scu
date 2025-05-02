pub mod hardware;
pub mod software;

use crate::{data::table::TableEntry, display_mode::DisplaySender, modules::Detection};

pub trait GenerateTableEntries: Detection {
    fn run(&self, sender: DisplaySenderT) -> std::io::Result<()> {
        Self::display(self.fetch()?, sender);
        Ok(())
    }
    fn display(data: Self::Result, sender: DisplaySenderT);
}

pub type DisplaySenderT = DisplaySender<(usize, usize), TableEntry>;
