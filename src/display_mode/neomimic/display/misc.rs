use crate::modules::{Detection, DisplayModule};

use super::{super::row::DataRow, RowSenderT};

pub struct Separator;

impl Detection for Separator {
    type Result = ();
    const NAME: &'static str = "separator";

    fn fetch(&self) -> std::io::Result<Self::Result> {
        Ok(())
    }
}
impl DisplayModule<RowSenderT> for Separator {
    fn display(_: Self::Result, _: &RowSenderT) {}
}

pub struct Header;

impl Detection for Header {
    type Result = String;
    const NAME: &'static str = "header";

    fn fetch(&self) -> std::io::Result<Self::Result> {
        let username = crate::modules::Username
            .fetch()
            .unwrap_or("username".into());
        let hostname = crate::modules::Hostname
            .fetch()
            .unwrap_or("localhost".into());

        Ok(format!("{username}@{hostname}"))
    }
}
impl DisplayModule<RowSenderT> for Header {
    fn display(value: Self::Result, sender: &RowSenderT) {
        DataRow::nameless(&value, sender);
    }
}
