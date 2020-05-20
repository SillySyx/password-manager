use app::read_app_path;
use filesystem::{save_file, load_file, list_files, remove_file};

pub fn save_password(name: &str, value: &[u8]) -> Result<(), String> {
    let mut path = read_app_path()?;
    path.push("passwords");
    path.push(name);

    save_file(&path, value)
}

pub fn load_password(name: &str) -> Result<Vec<u8>, String> {
    let mut path = read_app_path()?;
    path.push("passwords");
    path.push(name);

    load_file(&path)
}

pub fn list_passwords() -> Result<Vec<String>, String> {
    let mut path = read_app_path()?;
    path.push("passwords");
    
    list_files(&path)
}

pub fn remove_password(name: &str) -> Result<(), String> {
    let mut path = read_app_path()?;
    path.push("passwords");
    path.push(name);

    remove_file(&path)
}