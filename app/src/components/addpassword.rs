use iced::{button, text_input, Column, Container, Element, Length, Row, Text, TextInput};

use crate::{
    components::{create_button, create_widget},
    messages::Messages,
    styles::HeaderStyle,
    translations::{translate, Languages},
    views::Views,
    passphrase::generate_passphrase,
};

pub struct AddPassword {
    pub name: String,
    pub password: String,

    add_button_state: button::State,
    back_button_state: button::State,
    passphrase_button_state: button::State,
    name_state: text_input::State,
    password_state: text_input::State,
}

impl AddPassword {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            password: String::new(),

            add_button_state: button::State::new(),
            back_button_state: button::State::new(),
            passphrase_button_state: button::State::new(),
            name_state: {
                let mut state = text_input::State::new();
                state.focus();
                state
            },
            password_state: text_input::State::new(),
        }
    }

    pub fn update_input(&mut self, name: &str, value: String) {
        match name {
            "name" => self.name = value,
            "password" => self.password = value,
            _ => {}
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
            Messages::ChangeView { view: Views::List },
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

        let name_input = TextInput::new(
            &mut self.name_state,
            &translate(Languages::English, "add.name-placeholder"),
            &self.name,
            |value| Messages::AddViewInputKeyChanged { input: "name", value },
        )
        .padding(10);

        let password_input = TextInput::new(
            &mut self.password_state,
            &translate(Languages::English, "add.password-placeholder"),
            &self.password,
            |value| Messages::AddViewInputKeyChanged{ input: "password", value },
        )
        .padding(10);

        let add_button = create_button(
            &mut self.add_button_state,
            &translate(Languages::English, "add.add-button"),
            Messages::AddPasswordMessage { name: self.name.clone(), password: self.password.clone() },
        );

        let generate_passphrase_button = create_button(
            &mut self.passphrase_button_state,
            &translate(Languages::English, "add.generate-button"),
            Messages::GeneratePassphraseForAddView,
        );

        let buttons = Row::new()
            .push(add_button)
            .push(iced::Space::with_width(Length::Fill))
            .push(generate_passphrase_button);

        let content_column = Column::new()
            .spacing(20)
            .push(name_input)
            .push(password_input)
            .push(buttons);

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

    pub fn generate_passphrase(&mut self) {
        if let Ok(passphrase) = generate_passphrase() {
            self.password = passphrase;
        }
    }
}