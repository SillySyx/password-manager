use std::error::Error;
use event_sourcing::{State, Event};

use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Password {
    pub name: String,
    pub password: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct PasswordsState {
    pub passwords: Vec<Password>,
}

impl PasswordsState {
    pub fn new() -> Self {
        Self {
            passwords: vec![],
        }
    }
}

impl State for PasswordsState {
    fn execute(&self, event: &Event) -> Self {
        if event.event_type == "AddPassword" {
            return add_password(self, event);
        }

        if event.event_type == "ChangePassword" {
            return change_password(self, event);
        }

        if event.event_type == "ChangeName" {
            return change_name(self, event);
        }

        if event.event_type == "RemovePassword" {
            return remove_password(self, event);
        }

        self.clone()
    }
}

fn convert_event_data_to_json(event: &Event) -> Result<Value, Box<dyn Error>> {
    let data = String::from_utf8(event.data.clone())?;

    let json: Value = serde_json::from_str(&data)?;

    Ok(json)
}

fn add_password(state: &PasswordsState, event: &Event) -> PasswordsState {
    let mut state = state.clone();

    let data = match convert_event_data_to_json(event) {
        Ok(value) => value,
        Err(_) => return state,
    };

    let password = Password {
        name: data["name"].as_str().unwrap().to_string(),
        description: String::from("ree"),
        password: data["password"].as_str().unwrap().to_string(),
    };

    state.passwords.push(password);

    state
}

fn change_name(state: &PasswordsState, event: &Event) -> PasswordsState {
    let mut state = state.clone();

    let data = match convert_event_data_to_json(event) {
        Ok(value) => value,
        Err(_) => return state,
    };

    let mut password = match state.passwords.iter_mut().find(|password| password.name == data["name"]) {
        Some(value) => value,
        None => return state,
    };

    password.name = data["new_name"].as_str().unwrap().to_string();

    state
}

fn change_password(state: &PasswordsState, event: &Event) -> PasswordsState {
    let mut state = state.clone();

    let data = match convert_event_data_to_json(event) {
        Ok(value) => value,
        Err(_) => return state,
    };

    let mut password = match state.passwords.iter_mut().find(|password| password.name == data["name"]) {
        Some(value) => value,
        None => return state,
    };

    password.password = data["new_password"].as_str().unwrap().to_string();

    state
}

fn remove_password(state: &PasswordsState, event: &Event) -> PasswordsState {
    let mut state = state.clone();

    let data = match convert_event_data_to_json(event) {
        Ok(value) => value,
        Err(_) => return state,
    };
        
    let index = match state.passwords.iter().position(|password| password.name == data["name"]) {
        Some(value) => value,
        None => return state,
    };

    state.passwords.remove(index);

    state
}
