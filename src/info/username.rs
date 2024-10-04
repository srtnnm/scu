use libscu::software::users;

pub(super) fn fetch() -> Option<String> {
    let username = users::fetch_current()?.name;

    if username.is_empty() {
        None
    } else {
        Some(username)
    }
}
