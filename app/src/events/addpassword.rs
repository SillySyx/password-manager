use std::error::Error;
use event_sourcing::{Event, EventMode};
use crate::states::{PasswordsState, Password};

#[derive(Debug)]
pub struct AddPasswordEvent {
    pub name: String,
    pub password: String,
}

impl AddPasswordEvent {
    pub fn from_json(json: &serde_json::Value) -> Option<Self> {
        if json["type"] != "AddPassword" {
            return None;
        }
    
        let name = json["name"].as_str()?;
        let password = json["password"].as_str()?;
    
        Some(Self {
            name: name.to_string(),
            password: password.to_string(),
        })
    }
}

impl Event<PasswordsState> for AddPasswordEvent {
    fn execute(&self, state: PasswordsState, _: EventMode) -> Result<PasswordsState, Box<dyn Error>> {
        let mut state = state.clone();

        let password = Password {
            name: self.name.clone(),
            password: self.password.clone(),
        };

        state.passwords.push(password);

        Ok(state)
    }
}