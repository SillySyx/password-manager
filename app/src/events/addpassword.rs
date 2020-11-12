use std::error::Error;
use event_sourcing::{Event, EventMode};
use crate::states::{PasswordsState, Password};

#[derive(Debug)]
pub struct AddPasswordEvent {
    name: String,
    password: Vec<u8>,
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