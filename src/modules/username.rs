use super::Detection;

use libscu::software::users;

pub struct Username;

impl Detection for Username {
    type Result = String;
    const NAME: &'static str = "username";

    fn fetch(&self) -> std::io::Result<Self::Result> {
        users::fetch_current().map(|userinfo| userinfo.name)
    }
}
