use common::key::{load_key};
use crypto::{decrypt, encrypt, generate_iv_from_seed};

pub fn encrypt_password(name: &str, password: &str) -> Result<Vec<u8>, String> {
    let key = load_key()?;
    let iv = generate_iv_from_seed(name)?;
    let bytes = password.as_bytes();
    
    encrypt(&bytes, &key, &iv)
}

pub fn decrypt_password(name: &str, password: &[u8]) -> Result<String, String> {
    let key = load_key()?;
    let iv = generate_iv_from_seed(name)?;
    let decrypted = decrypt(password, &key, &iv)?;

    let value = match std::str::from_utf8(&decrypted) {
        Ok(value) => value,
        Err(_) => return Err(String::from("Failed to convert password to text")),
    };

    Ok(value.to_owned())
}