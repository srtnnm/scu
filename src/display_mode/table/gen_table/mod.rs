pub mod hardware;
pub mod software;

use crate::{data::table::Table, modules::Detection};

pub trait GenerateTableEntries: Detection {
    fn gen_entries(&self, entry_buf: &mut Table) -> std::io::Result<()> {
        Self::display(self.fetch()?, entry_buf);
        Ok(())
    }
    fn display(data: Self::Result, table: &mut Table);
}

// macro_rules! generate_table_entries_run {
//     ($($struct:tt)*) => {
//         $(
//             impl GenerateTableEntries for $struct {

//             }
//         )*
//     }
// }
