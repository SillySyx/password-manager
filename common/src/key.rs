use crate::app::read_app_path;
use crate::filesystem::{save_file, load_file};

pub fn save_key(key: &[u8]) -> Result<(), String> {
    let mut path = read_app_path()?;
    path.push("key");

    save_file(&path, key)
}

pub fn load_key() -> Result<Vec<u8>, String> {
    let mut path = read_app_path()?;
    path.push("key");

    load_file(&path)
}