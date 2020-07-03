use iced::{Element, Text};

use super::app::{Messages};

#[derive(Debug, Clone)]
pub enum PasswordMessages {
    CopyPassword(String),
}

pub struct Password {
    pub name: String,
}

impl Password {
    pub fn new(name: String) -> Password {
        Password {
            name,
        }
    }

    pub fn update(&mut self, message: PasswordMessages) {
        match message {
            PasswordMessages::CopyPassword(name) => {},
        }
    }

    pub fn view(&mut self) -> Element<Messages> {
        Text::new(self.name.clone())
            .into()
    }
}