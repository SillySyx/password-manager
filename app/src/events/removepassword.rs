use std::error::Error;
use event_sourcing::{Event, EventMode};
use crate::states::PasswordsState;

#[derive(Debug)]
pub struct RemovePasswordEvent {
    name: String,
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