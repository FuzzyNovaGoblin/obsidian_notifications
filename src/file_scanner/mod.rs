use std::path::Path;

pub mod sync_conflict;
pub mod time_reminder;

pub fn path_str_no_root(path: impl AsRef<Path>, root: impl AsRef<Path>) -> String {
    path.as_ref()
        .strip_prefix(root)
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned()
}
