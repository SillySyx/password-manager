mod arguments;
mod password;

use password::{decrypt_password, encrypt_password};

fn main() -> Result<(), String> {
    common::app::ensure_app_folders_exist()?;

    if let Some(key_seed) = arguments::should_save_key() {
        return save_key(key_seed);
    }

    if arguments::should_list_passwords() {
        return list_passwords();
    }

    if arguments::should_generate_passphrase() {
        return generate_passphrase();
    }

    if let Some(name) = arguments::should_load_password() {
        return load_password(name);
    }

    if let Some((name, password)) = arguments::should_save_password() {
        return save_password(name, password);
    }

    return show_help();
}

fn save_key(seed: String) -> Result<(), String> {
    let key = crypto::generate_key_from_seed(&seed)?;
    common::key::save_key(&key)?;
    
    println!("Key saved");
    Ok(())
}

fn list_passwords() -> Result<(), String> {
    let passwords = common::password::list_passwords()?;

    for password in passwords {
        println!("{}", password);
    }
    
    Ok(())
}

fn generate_passphrase() -> Result<(), String> {
    let passphrase = common::passphrase::generate_passphrase()?;
    
    println!("{}", passphrase);
    Ok(())
}

fn load_password(name: String) -> Result<(), String> {
    let password = common::password::load_password(&name)?;
    let decrypted = decrypt_password(&name, &password)?;

    common::clip_board::copy_value_to_clipboard(decrypted)?; 
    
    println!("Password copied to clipboard");

    std::thread::sleep(std::time::Duration::from_millis(100));

    Ok(())
}

fn save_password(name: String, password: String) -> Result<(), String> {
    let encrypted = encrypt_password(&name, &password)?;
    common::password::save_password(&name, &encrypted)?;
    
    println!("Password saved");
    Ok(())
}

fn show_help() -> Result<(), String> {
    println!("Commands:");
    println!("  key [SEED]");
    println!("  list");
    println!("  load [NAME]");
    println!("  save [NAME] [PASSWORD]");
    println!("  passphrase");
    Ok(())
}