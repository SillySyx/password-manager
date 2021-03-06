use std::error::Error;
use event_sourcing::{State, Event};

use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Password {
    pub name: String,
    pub password: String,
    pub description: String,
    pub category: String,
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

        if event.event_type == "ChangeDescription" {
            return change_description(self, event);
        }

        if event.event_type == "ChangeCategory" {
            return change_category(self, event);
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

    let name = match data["name"].as_str() {
        Some(name) => name.to_string(),
        None => String::new(),
    };

    let description = match data["description"].as_str() {
        Some(description) => description.to_string(),
        None => String::new(),
    };

    let password = match data["password"].as_str() {
        Some(password) => password.to_string(),
        None => String::new(),
    };

    let category = match data["category"].as_str() {
        Some(category) => category.to_string(),
        None => String::new(),
    };

    state.passwords.push(Password {
        name,
        description,
        password,
        category,
    });

    state
}

fn change_name(state: &PasswordsState, event: &Event) -> PasswordsState {
    let mut state = state.clone();

    let data = match convert_event_data_to_json(event) {
        Ok(value) => value,
        Err(_) => return state,
    };

    let new_name = match data["new_name"].as_str() {
        Some(name) => name.to_string(),
        None => return state,
    };

    let mut password = match state.passwords.iter_mut().find(|password| password.name == data["name"]) {
        Some(value) => value,
        None => return state,
    };

    password.name = new_name;

    state
}

fn change_description(state: &PasswordsState, event: &Event) -> PasswordsState {
    let mut state = state.clone();

    let data = match convert_event_data_to_json(event) {
        Ok(value) => value,
        Err(_) => return state,
    };

    let new_description = match data["new_description"].as_str() {
        Some(description) => description.to_string(),
        None => return state,
    };

    let mut password = match state.passwords.iter_mut().find(|password| password.name == data["name"]) {
        Some(value) => value,
        None => return state,
    };

    password.description = new_description;

    state
}

fn change_category(state: &PasswordsState, event: &Event) -> PasswordsState {
    let mut state = state.clone();

    let data = match convert_event_data_to_json(event) {
        Ok(value) => value,
        Err(_) => return state,
    };

    let new_category = match data["new_category"].as_str() {
        Some(category) => category.to_string(),
        None => return state,
    };

    let mut password = match state.passwords.iter_mut().find(|password| password.name == data["name"]) {
        Some(value) => value,
        None => return state,
    };

    password.category = new_category;

    state
}

fn change_password(state: &PasswordsState, event: &Event) -> PasswordsState {
    let mut state = state.clone();

    let data = match convert_event_data_to_json(event) {
        Ok(value) => value,
        Err(_) => return state,
    };

    let new_password = match data["new_password"].as_str() {
        Some(password) => password.to_string(),
        None => return state,
    };

    let mut password = match state.passwords.iter_mut().find(|password| password.name == data["name"]) {
        Some(value) => value,
        None => return state,
    };

    password.password = new_password;

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
