use super::{DataRow, ModuleTrait};

pub struct Separator;

impl ModuleTrait for Separator {
    const NAME: &'static str = "separator";

    fn run(_info: &crate::info::SystemInformation) -> std::io::Result<usize> {
        Ok(DataRow::separator('-'))
    }
}
