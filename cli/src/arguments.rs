pub fn should_save_key() -> Option<String> {
    let args: Vec<String> = std::env::args().collect();

    let command = match args.get(1) {
        Some(value) => value,
        None => return None,
    };

    if command != "key" {
        return None;
    }

    let key = match args.get(2) {
        Some(value) => value,
        None => return None,
    };

    Some(key.to_string())
}

pub fn should_list_passwords() -> bool {
    let args: Vec<String> = std::env::args().collect();

    let command = match args.get(1) {
        Some(value) => value,
        None => return false,
    };

    command == "list"
}

pub fn should_generate_passphrase() -> bool {
    let args: Vec<String> = std::env::args().collect();

    let command = match args.get(1) {
        Some(value) => value,
        None => return false,
    };

    command == "passphrase"
}

pub fn should_load_password() -> Option<String> {
    let args: Vec<String> = std::env::args().collect();

    let command = match args.get(1) {
        Some(value) => value,
        None => return None,
    };

    if command != "load" {
        return None;
    }

    let name = match args.get(2) {
        Some(value) => value,
        None => return None,
    };

    Some(name.to_string())
}

pub fn should_save_password() -> Option<(String, String)> {
    let args: Vec<String> = std::env::args().collect();

    let command = match args.get(1) {
        Some(value) => value,
        None => return None,
    };

    if command != "save" {
        return None;
    }

    let name = match args.get(2) {
        Some(value) => value,
        None => return None,
    };

    let password = match args.get(3) {
        Some(value) => value,
        None => return None,
    };

    Some((name.to_string(), password.to_string()))
}