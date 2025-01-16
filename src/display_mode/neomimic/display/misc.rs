use crate::modules::Detection;

use super::{super::row::DataRow, Display};

pub struct Separator;

impl Detection for Separator {
    type Result = ();
    const NAME: &'static str = "separator";

    fn fetch() -> std::io::Result<Self::Result> {
        Ok(())
    }
}
impl Display for Separator {
    fn display(_: Self::Result) -> std::io::Result<usize> {
        Ok(DataRow::separator('-'))
    }
}

pub struct Header;

impl Detection for Header {
    type Result = String;
    const NAME: &'static str = "header";

    fn fetch() -> std::io::Result<Self::Result> {
        let username = crate::modules::Username::fetch().unwrap_or("username".into());
        let hostname = crate::modules::Hostname::fetch().unwrap_or("localhost".into());

        Ok(format!("{username}@{hostname}"))
    }
}
impl Display for Header {
    fn display(value: Self::Result) -> std::io::Result<usize> {
        Ok(DataRow::nameless(&value))
    }
}
