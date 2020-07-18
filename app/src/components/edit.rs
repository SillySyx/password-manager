use iced::{button, text_input, Column, Container, Element, Length, Row, Text, TextInput};

use crate::{
    components::{create_button, create_widget},
    messages::Messages,
    styles::HeaderStyle,
    translations::{translate, Languages},
    views::Views,
};

pub struct EditPassword {
    pub entry: String,
    pub name: String,
    pub password: String,

    save_button_state: button::State,
    remove_button_state: button::State,
    back_button_state: button::State,

    name_state: text_input::State,
    password_state: text_input::State,
}

impl EditPassword {
    pub fn new() -> Self {
        Self {
            entry: String::new(),
            name: String::new(),
            password: String::new(),

            save_button_state: button::State::new(),
            remove_button_state: button::State::new(),
            back_button_state: button::State::new(),

            name_state: text_input::State::new(),
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
        self.entry = String::new();
        self.name = String::new();
        self.password = String::new();
    }

    pub fn view(&mut self) -> Element<Messages> {
        let header_title = Text::new(translate(Languages::English, "edit.header"))
            .width(Length::Fill)
            .vertical_alignment(iced::VerticalAlignment::Center)
            .size(26);

        let back_button = create_button(
            &mut self.back_button_state,
            &translate(Languages::English, "edit.back-button"),
            Messages::ChangeView { view: Views::List }
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
            &translate(Languages::English, "edit.name-placeholder"),
            &self.name,
            |value| Messages::EditViewInputKeyChanged { input: "name", value }
        )
        .padding(10);

        let password_input = TextInput::new(
            &mut self.password_state,
            &translate(Languages::English, "edit.password-placeholder"),
            &self.password,
            |value| Messages::EditViewInputKeyChanged { input: "password", value }
        )
        .padding(10)
        .password();

        let save_button = create_button(
            &mut self.save_button_state,
            &translate(Languages::English, "edit.save-button"),
            Messages::UpdatePassword { entry: self.entry.clone(), name: self.name.clone(), password: self.password.clone() }
        );

        let remove_button = create_button(
            &mut self.remove_button_state,
            &translate(Languages::English, "edit.remove-button"),
            Messages::RemovePassword { name: self.name.clone() }
        );

        let content_column = Column::new()
            .spacing(20)
            .push(name_input)
            .push(password_input)
            .push(save_button)
            .push(remove_button);

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
}
