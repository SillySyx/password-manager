use glob::MemoryArchive;

use std::error::Error;
use std::fs::OpenOptions;
use std::io::{Read, Write};

pub fn load_datastore(key: &[u8]) -> Result<MemoryArchive, Box<dyn Error>> {
    let iv = crypto::generate_iv_from_seed("silly goose")?;
    
    let bytes = read_file_bytes()?;
    let bytes = crypto::decrypt(&bytes, key, &iv)?;
    
    let archive = MemoryArchive::from(bytes);

    Ok(archive)
}

pub fn save_datastore(key: &[u8], archive: &mut MemoryArchive) -> Result<(), Box<dyn Error>> {
    let iv = crypto::generate_iv_from_seed("silly goose")?;

    let bytes = archive.as_bytes()?;
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