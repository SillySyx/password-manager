use std::path::{PathBuf};
use std::fs::{create_dir};

pub fn read_app_path() -> Result<PathBuf, String> {
    let mut dir = match dirs::config_dir() {
        Some(dir) => dir,
        None => return Err(String::from("Failed to config dir")),
    };

    dir.push("passwordmanager");

    if dir.exists() {
        return Ok(dir);
    }

    match create_dir(&dir) {
        Ok(_) => Ok(dir),
        Err(_) => Err(String::from("Failed to create app folder")),
    }
}