use super::Detection;

use libscu::software::users;

pub(super) fn fetch() -> Option<String> {
    let username = users::fetch_current().ok()?.name;

    if username.is_empty() {
        None
    } else {
        Some(username)
    }
}

pub struct Username;

impl Detection for Username {
    type Result = String;
    const NAME: &'static str = "username";

    fn fetch() -> std::io::Result<Self::Result> {
        users::fetch_current().map(|userinfo| userinfo.name)
    }
}
