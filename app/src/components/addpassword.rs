use iced::{Element, Text, button, TextInput, text_input, Container, Column, Row, Length};

use crate::styles::HeaderStyle;
use crate::components::{create_widget, create_button};
use crate::components::app::{Messages, Views};
use crate::translations::{translate, Languages};

use crate::datastore::{load_datastore, save_datastore};

use glob::Archive;

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
        let header_title = Text::new(translate(Languages::English, "add.header"))
            .width(Length::Fill)
            .vertical_alignment(iced::VerticalAlignment::Center)
            .size(26);

        let back_button = create_button(
            &mut self.back_button_state, 
            &translate(Languages::English, "add.back-button"), 
            Messages::ChangeView(Views::List)
        );

        let header_row = Row::new()
            .max_width(500)
            .height(Length::Units(35))
            .push(header_title)
            .push(back_button);

        let header_container = Container::new(header_row)
            .padding(10)
            .width(Length::Fill)
            .center_x()
            .style(HeaderStyle);

        let description = Text::new(translate(Languages::English, "add.description"))
            .size(16);

        let name_input = TextInput::new(&mut self.name_state, &translate(Languages::English, "add.name-placeholder"), &self.name, |value| Messages::AddPasswordUpdateInputMessage("name", value))
            .padding(10);

        let password_input = TextInput::new(&mut self.password_state, &translate(Languages::English, "add.password-placeholder"), &self.password, |value| Messages::AddPasswordUpdateInputMessage("password", value))
            .padding(10);

        let add_button = create_button(
            &mut self.add_button_state, 
            &translate(Languages::English, "add.add-button"), 
            Messages::AddPasswordMessage
        );

        let content_column = Column::new()
            .spacing(20)
            .push(name_input)
            .push(password_input)
            .push(add_button)
            .push(description);

        let content = create_widget(content_column);

        let content_container = Container::new(content)
            .max_width(500)
            .height(Length::Fill)
            .center_y();

        Column::new()
            .align_items(iced::Align::Center)
            .push(header_container)
            .push(content_container)
            .into()
    }

    pub fn add_new_password(&self, name: String, password: String) -> Result<(), Box<dyn Error>> {
        let mut archive = load_datastore(&self.key)?;

        archive.add_entry(&name, password.as_bytes())?;

        save_datastore(&self.key, &mut archive)?;

        Ok(())
    }
}

