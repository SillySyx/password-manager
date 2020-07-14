use iced::{Element, Text, Row, Length, button};

use crate::components::{create_widget, create_button};
use crate::translations::{translate, Languages};
use crate::datastore::{load_datastore, save_datastore};
use crate::clipboard::copy_value_to_clipboard;

use glob::Archive;

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

        let copy_button = create_button(
            &mut self.copy_state, 
            &translate(Languages::English, "password.copy-button"),
            PasswordMessages::CopyPassword(self.name.clone())
        );

        let remove_button = create_button(
            &mut self.remove_state, 
            &translate(Languages::English, "password.remove-button"),
            PasswordMessages::RemovePassword(self.name.clone())
        );

        let row = Row::new()
            .push(text)
            .push(copy_button)
            .push(remove_button)
            .spacing(5);

        create_widget(row).into()
    }

    fn copy_password(&self, name: String) {
        let mut archive = load_datastore(&self.key).expect("failed to load data");

        let entry = archive.find_entry(&name).expect("failed to find entry");

        let data = match archive.read_entry_data(&entry) {
            Ok(data) => data,
            Err(_) => return,
        };

        let password = String::from_utf8(data.clone()).expect("failed to read password");

        copy_value_to_clipboard(password).expect("failed to copy password to clipboard");
    }

    fn remove_password(&self, name: String) {
        let mut archive = load_datastore(&self.key).expect("failed to load data");

        let entry = archive.find_entry(&name).expect("failed to find entry");

        archive.remove_entry(&entry).expect("failed to remove entry");

        save_datastore(&self.key, &mut archive).expect("failed to save data");
    }
}