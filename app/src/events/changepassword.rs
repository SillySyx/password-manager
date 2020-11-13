use std::error::Error;
use event_sourcing::{Event, EventMode};
use crate::states::PasswordsState;

#[derive(Debug)]
pub struct ChangePasswordEvent {
    pub name: String,
    pub new_password: String,
}

impl ChangePasswordEvent {
    pub fn from_json(json: &serde_json::Value) -> Option<Self> {
        if json["type"] != "ChangePassword" {
            return None;
        }
    
        let name = json["name"].as_str()?;
        let password = json["new_password"].as_str()?;
    
        Some(Self {
            name: name.to_string(),
            new_password: password.to_string(),
        })
    }
}

impl Event<PasswordsState> for ChangePasswordEvent {
    fn execute(&self, state: PasswordsState, _: EventMode) -> Result<PasswordsState, Box<dyn Error>> {
        let mut state = state.clone();

        let mut password = match state.passwords.iter_mut().find(|password| password.name == self.name) {
            Some(value) => value,
            None => return Err(Box::from("failed to find password")),
        };

        password.password = self.new_password.clone();

        Ok(state)
    }
}