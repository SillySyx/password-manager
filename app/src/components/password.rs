use iced::{button, Element, Length, Row, Text};

use crate::{
    components::{create_button, create_widget},
    messages::Messages,
    translations::{translate, Languages},
};

pub struct Password {
    pub key: [u8; 32],
    pub name: String,

    edit_state: button::State,
    copy_state: button::State,
}

impl Password {
    pub fn new(name: String, key: [u8; 32]) -> Self {
        Self {
            key,
            name,
            edit_state: button::State::new(),
            copy_state: button::State::new(),
        }
    }

    pub fn view(&mut self) -> Element<Messages> {
        let text = Text::new(&self.name).width(Length::Fill);

        let edit_button = create_button(
            &mut self.edit_state,
            &translate(Languages::English, "password.edit-button"),
            "cog.svg",
            Messages::EditPassword { name: self.name.clone() }
        )
        .padding(5);

        let copy_button = create_button(
            &mut self.copy_state,
            &translate(Languages::English, "password.copy-button"),
            "key.svg",
            Messages::CopyPassword { name: self.name.clone() }
        )
        .padding(5);

        let row = Row::new()
            .push(text)
            .push(edit_button)
            .push(copy_button)
            .spacing(5)
            .align_items(iced::Align::Center);

        create_widget(row)
            .padding(10)
            .into()
    }
}
