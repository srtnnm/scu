mod hardware;
mod software;

use crate::{data::table::Table, modules::Detection};

trait GenerateTable {
    fn gen_table(name: &str) -> Option<Table>;
}

pub trait GenerateTableEntries: Detection {
    fn gen_entries(&self, entry_buf: &mut Table) -> std::io::Result<()> {
        Self::display(self.fetch()?, entry_buf);
        Ok(())
    }
    fn display(data: Self::Result, table: &mut Table);
}
