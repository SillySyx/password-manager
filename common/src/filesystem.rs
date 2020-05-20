use std::fs::File;
use std::io::{BufReader, BufRead};
use std::io::prelude::*;

use std::path::{Path};

pub fn save_file(path: &Path, value: &[u8]) -> Result<(), String> {
    let mut file = match File::create(&path) {
        Ok(file) => file,
        Err(_) => return Err(String::from("Failed to create file")),
    };

    match file.write_all(value) {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Failed to save file")),
    }
}

pub fn load_file(path: &Path) -> Result<Vec<u8>, String> {
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => return Err(String::from("Failed to open file")),
    };

    let mut data: Vec<u8> = Vec::new();
    match file.read_to_end(&mut data) {
        Ok(_) => Ok(data),
        Err(_) => Err(String::from("Failed to read key")),
    }
}

pub fn remove_file(path: &Path) -> Result<(), String> {
    match std::fs::remove_file(path) {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Failed to remove file")),
    }
}

pub fn read_lines_in_file(path: &Path) -> Result<Vec<String>, String> {
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => return Err(format!("Failed to open file: {}", path.to_str().unwrap())),
    };

    let buffered = BufReader::new(file);

    let lines: Vec<String> = buffered
        .lines()
        .map(|l| l.unwrap())
        .collect();

    Ok(lines)
}

pub fn list_files(path: &Path) -> Result<Vec<String>, String> {
    let mut files: Vec<String> = Vec::new();

    let dir = match std::fs::read_dir(path) {
        Ok(dir) => dir,
        Err(_) => return Err(String::from("Failed to read dir")),
    };

    for entry in dir {
        let name = entry.unwrap().file_name().into_string().unwrap();
        files.push(name);
    }

    files.sort();

    Ok(files)
}