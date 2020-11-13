use std::error::Error;
use event_sourcing::{Event, EventMode};
use crate::states::PasswordsState;

#[derive(Debug)]
pub struct ChangeNameEvent {
    pub name: String,
    pub new_name: String,
}

impl ChangeNameEvent {
    pub fn from_json(json: &serde_json::Value) -> Option<Self> {
        if json["type"] != "ChangeName" {
            return None;
        }
    
        let name = json["name"].as_str()?;
        let new_name = json["new_name"].as_str()?;
    
        Some(Self {
            name: name.to_string(),
            new_name: new_name.to_string(),
        })
    }
}

impl Event<PasswordsState> for ChangeNameEvent {
    fn execute(&self, state: PasswordsState, _: EventMode) -> Result<PasswordsState, Box<dyn Error>> {
        let mut state = state.clone();

        let mut password = match state.passwords.iter_mut().find(|password| password.name == self.name) {
            Some(value) => value,
            None => return Err(Box::from("failed to find password")),
        };

        password.name = self.new_name.clone();

        Ok(state)
    }
}