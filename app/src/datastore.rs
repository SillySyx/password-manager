use glob::Glob;

use std::error::Error;
use std::fs::OpenOptions;
use std::io::{Read, Write};

pub fn load_datastore(key: &[u8]) -> Result<Glob, Box<dyn Error>> {
    let iv = crypto::generate_iv_from_seed("silly goose")?;
    
    let bytes = read_file_bytes()?;
    let bytes = crypto::decrypt(&bytes, key, &iv)?;
    
    let glob = Glob::from(&bytes)?;

    Ok(glob)
}

pub fn save_datastore(key: &[u8], glob: Glob) -> Result<(), Box<dyn Error>> {
    let iv = crypto::generate_iv_from_seed("silly goose")?;

    let bytes = glob.as_bytes()?;
    let bytes = crypto::encrypt(&bytes, key, &iv)?;

    save_file_bytes(&bytes)?;

    Ok(())
}

fn read_file_bytes() -> Result<Vec<u8>, Box<dyn Error>> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("./passwords")?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}

fn save_file_bytes(bytes: &[u8]) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open("./passwords")?;

    file.write(bytes)?;

    Ok(())
}