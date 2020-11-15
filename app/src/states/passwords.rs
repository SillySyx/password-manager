use crate::events::{AddPasswordEvent, ChangeNameEvent, ChangePasswordEvent, RemovePasswordEvent};
use event_sourcing::{JsonState, Event, EventMode};

use serde_json::Value;
use serde::Serialize;

#[derive(Serialize)]
#[derive(Debug, Clone)]
pub struct Password {
    pub name: String,
    pub password: String,
}

#[derive(Serialize)]
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

impl JsonState for PasswordsState {
    fn from_json(json: String) -> Self {
        let json: Value = match serde_json::from_str(&json) {
            Ok(value) => value,
            Err(_) => return PasswordsState::new(),
        };

        let events = match json.as_array() {
            Some(value) => value,
            None => return PasswordsState::new(),
        };

        let state = events.iter().fold(PasswordsState::new(), |state, event| {
            if let Some(event) = AddPasswordEvent::from_json(event) {
                if let Ok(new_state) = event.execute(state.clone(), EventMode::Replay) {
                    return new_state;
                }
            }

            if let Some(event) = ChangeNameEvent::from_json(event) {
                if let Ok(new_state) = event.execute(state.clone(), EventMode::Replay) {
                    return new_state;
                }
            }

            if let Some(event) = ChangePasswordEvent::from_json(event) {
                if let Ok(new_state) = event.execute(state.clone(), EventMode::Replay) {
                    return new_state;
                }
            }

            if let Some(event) = RemovePasswordEvent::from_json(event) {
                if let Ok(new_state) = event.execute(state.clone(), EventMode::Replay) {
                    return new_state;
                }
            }

            state
        });

        state
    }

    fn to_json(&self) -> String {
        match serde_json::to_string(self) {
            Ok(value) => value,
            Err(_) => String::from(""),
        }
    }
}