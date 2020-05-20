use std::path::{Path, PathBuf};

pub fn read_app_path() -> Result<PathBuf, String> {
    let mut dir = match dirs::config_dir() {
        Some(dir) => dir,
        None => return Err(String::from("Failed to config dir")),
    };

    dir.push("passwordmanager");

    Ok(dir)
}

pub fn ensure_app_folders_exist() -> Result<(), String> {
    let app_path = read_app_path()?;
    create_path_if_not_exists(&app_path)?;
        
    let mut passwords_path = read_app_path()?;
    passwords_path.push("passwords");
    create_path_if_not_exists(&passwords_path)?;

    Ok(())
}

fn create_path_if_not_exists(path: &Path) -> Result<(), String> {
    if path.exists() {
        return Ok(());
    }

    match std::fs::create_dir(path) {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Failed to create folder")),
    }
}