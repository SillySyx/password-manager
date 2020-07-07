use iced::{Element, Text, Button, button, TextInput, text_input, Container, Column, Length};

use crate::components::app::{Messages, Views};
use crate::translations::{translate, Languages};

use glob::GlobEntry;
use crate::datastore::{load_datastore, save_datastore};

use std::error::Error;

pub struct AddPassword {
    pub key: [u8; 32],
    pub name: String,
    pub password: String,

    add_button_state: button::State,
    back_button_state: button::State,
    name_state: text_input::State,
    password_state: text_input::State,
}

impl AddPassword {
    pub fn new() -> Self {
        Self {
            key: [0; 32],
            name: String::new(),
            password: String::new(),

            add_button_state: button::State::new(),
            back_button_state: button::State::new(),
            name_state: text_input::State::new(),
            password_state: text_input::State::new(),
        }
    }

    pub fn title(&self) -> String {
        translate(Languages::English, "add.title")
    }

    pub fn update_input(&mut self, name: &str, value: String) {
        match name {
            "name" => self.name = value,
            "password" => self.password = value,
            _ => {},
        };
    }

    pub fn reset(&mut self) {
        self.name = String::new();
        self.password = String::new();
    }

    pub fn view(&mut self) -> Element<Messages> {
        let header = Text::new(translate(Languages::English, "add.header"))
            .size(26);

        let description = Text::new(translate(Languages::English, "add.description"))
            .size(18);

        let name_input = TextInput::new(&mut self.name_state, &translate(Languages::English, "add.name-placeholder"), &self.name, |value| Messages::AddPasswordUpdateInputMessage("name", value))
            .padding(10);

        let password_input = TextInput::new(&mut self.password_state, &translate(Languages::English, "add.password-placeholder"), &self.password, |value| Messages::AddPasswordUpdateInputMessage("password", value))
            .padding(10);

        let add_button = Button::new(&mut self.add_button_state, Text::new(translate(Languages::English, "add.add-button")))
            .on_press(Messages::AddPasswordMessage);

        let back_button = Button::new(&mut self.back_button_state, Text::new(translate(Languages::English, "add.back-button")))
            .on_press(Messages::ChangeView(Views::List));

        let content = Column::new()
            .spacing(20)
            .max_width(300)
            .push(header)
            .push(description)
            .push(name_input)
            .push(password_input)
            .push(add_button)
            .push(back_button);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    pub fn add_new_password(&self, name: String, password: String) -> Result<(), Box<dyn Error>> {
        let mut glob = load_datastore(&self.key)?;

        glob.add(GlobEntry {
            name, 
            data: Some(password.as_bytes().to_owned()),
        });

        save_datastore(&self.key, glob)?;

        Ok(())
    }
}

