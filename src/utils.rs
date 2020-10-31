use std::path::PathBuf;

pub fn path_to_string(path: PathBuf) -> Option<String> {
    path.into_os_string().into_string().ok()
}
