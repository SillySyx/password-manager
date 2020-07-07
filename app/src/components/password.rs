use iced::{Element, Text, Row, Length, button, Button};

use crate::translations::{translate, Languages};
use crate::datastore::{load_datastore, save_datastore};
use crate::clipboard::copy_value_to_clipboard;

#[derive(Debug, Clone)]
pub enum PasswordMessages {
    CopyPassword(String),
    RemovePassword(String),
}

pub struct Password {
    pub key: [u8; 32],
    pub name: String,

    copy_state: button::State,
    remove_state: button::State,
}

impl Password {
    pub fn new(name: String, key: [u8; 32]) -> Self {
        Self {
            key,
            name,
            copy_state: button::State::new(),
            remove_state: button::State::new(),
        }
    }

    pub fn update(&mut self, message: PasswordMessages) {
        match message {
            PasswordMessages::CopyPassword(name) => self.copy_password(name),
            PasswordMessages::RemovePassword(name) => self.remove_password(name),
        }
    }

    pub fn view(&mut self) -> Element<PasswordMessages> {
        let text = Text::new(&self.name)
            .width(Length::Fill);

        let copy_button = Button::new(&mut self.copy_state, Text::new(translate(Languages::English, "password.copy-button")).size(16))
            .on_press(PasswordMessages::CopyPassword(self.name.clone()));

        let remove_button = Button::new(&mut self.remove_state, Text::new(translate(Languages::English, "password.remove-button")).size(16))
            .on_press(PasswordMessages::RemovePassword(self.name.clone()));

        Row::new()
            .push(text)
            .push(copy_button)
            .push(remove_button)
            .spacing(5)
            .into()
    }

    fn copy_password(&self, name: String) {
        let glob = load_datastore(&self.key).expect("failed to load data");

        let entry = glob.find(|e| e.name == name).expect("failed to find entry");

        let data = match &entry.data {
            Some(data) => data,
            None => return,
        };

        let password = String::from_utf8(data.clone()).expect("failed to read password");

        copy_value_to_clipboard(password).expect("failed to copy password to clipboard");
    }

    fn remove_password(&self, name: String) {
        let mut glob = load_datastore(&self.key).expect("failed to load data");

        glob.remove(|e| e.name == name).expect("failed to remove entry");

        save_datastore(&self.key, glob).expect("failed to save data");
    }
}