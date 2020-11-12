use std::error::Error;
use event_sourcing::{Event, EventMode};
use crate::states::PasswordsState;

#[derive(Debug)]
pub struct ChangePasswordEvent {
    name: String,
    new_password: Vec<u8>,
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