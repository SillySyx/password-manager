use std::error::Error;
use event_sourcing::{Event, EventMode};
use crate::states::PasswordsState;

#[derive(Debug)]
pub struct RemovePasswordEvent {
    pub name: String,
}

impl RemovePasswordEvent {
    pub fn from_json(json: &serde_json::Value) -> Option<Self> {
        if json["type"] != "RemovePassword" {
            return None;
        }
    
        let name = json["name"].as_str()?;
    
        Some(Self {
            name: name.to_string(),
        })
    }
}

impl Event<PasswordsState> for RemovePasswordEvent {
    fn execute(&self, state: PasswordsState, _: EventMode) -> Result<PasswordsState, Box<dyn Error>> {
        let mut state = state.clone();
        
        let index = match state.passwords.iter().position(|password| password.name == self.name) {
            Some(value) => value,
            None => return Err(Box::from("failed to find password")),
        };

        state.passwords.remove(index);

        Ok(state)
    }
}