use std::error::Error;
use event_sourcing::{State, Event};

use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Password {
    pub name: String,
    pub password: String,
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
        name: data["name"].to_string(),
        password: data["password"].to_string(),
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

    password.name = data["new_name"].to_string().clone();

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

    password.password = data["new_password"].to_string().clone();

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

// impl JsonState for PasswordsState {
//     fn from_json(json: String) -> Self {
//         let json: Value = match serde_json::from_str(&json) {
//             Ok(value) => value,
//             Err(_) => return PasswordsState::new(),
//         };

//         let events = match json.as_array() {
//             Some(value) => value,
//             None => return PasswordsState::new(),
//         };

//         let state = events.iter().fold(PasswordsState::new(), |state, event| {
//             if let Some(event) = AddPasswordEvent::from_json(event) {
//                 if let Ok(new_state) = event.execute(state.clone(), EventMode::Replay) {
//                     return new_state;
//                 }
//             }

//             if let Some(event) = ChangeNameEvent::from_json(event) {
//                 if let Ok(new_state) = event.execute(state.clone(), EventMode::Replay) {
//                     return new_state;
//                 }
//             }

//             if let Some(event) = ChangePasswordEvent::from_json(event) {
//                 if let Ok(new_state) = event.execute(state.clone(), EventMode::Replay) {
//                     return new_state;
//                 }
//             }

//             if let Some(event) = RemovePasswordEvent::from_json(event) {
//                 if let Ok(new_state) = event.execute(state.clone(), EventMode::Replay) {
//                     return new_state;
//                 }
//             }

//             state
//         });

//         state
//     }

//     fn to_json(&self) -> String {
//         match serde_json::to_string(self) {
//             Ok(value) => value,
//             Err(_) => String::from(""),
//         }
//     }
// }