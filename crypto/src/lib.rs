extern crate rand;
extern crate c2_chacha;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use rand::prelude::*;
use rand::{SeedableRng};

use c2_chacha::stream_cipher::{NewStreamCipher, SyncStreamCipher};
use c2_chacha::{ChaCha20};

pub fn generate_key_from_seed(seed: &str) -> Result<Vec<u8>, String> {
    let hash = hash_value(seed);

    let mut buffer = [0u8; 32];
    
    let mut rng: StdRng = SeedableRng::seed_from_u64(hash);
    rng.fill_bytes(&mut buffer);

    Ok(buffer.to_vec())
}

pub fn generate_iv_from_seed(seed: &str) -> Result<Vec<u8>, String> {
    let hash = hash_value(seed);

    let mut buffer = [0u8; 8];
    
    let mut rng: StdRng = SeedableRng::seed_from_u64(hash);
    rng.fill_bytes(&mut buffer);

    Ok(buffer.to_vec())
}

pub fn encrypt(value: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, String> {
    let mut buffer = value.to_owned();

    let mut cipher = match ChaCha20::new_var(key, iv) {
        Ok(value) => value,
        Err(_) => return Err(String::from("Failed to encrypt value")),
    };

    cipher.apply_keystream(&mut buffer);

    Ok(buffer)
}

pub fn decrypt(value: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, String> {
    let mut buffer = value.to_owned();

    let mut cipher = match ChaCha20::new_var(key, iv) {
        Ok(value) => value,
        Err(_) => return Err(String::from("Failed to encrypt value")),
    };

    cipher.apply_keystream(&mut buffer);

    Ok(buffer)
}

fn hash_value(seed: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    seed.hash(&mut hasher);
    hasher.finish()
}