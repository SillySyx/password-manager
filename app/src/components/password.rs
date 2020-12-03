use iced::{button, Element, Length, Column, Row, Text};

use crate::{
    components::create_link_button,
    messages::Messages,
};

pub struct Password {
    pub key: [u8; 32],
    pub name: String,
    pub description: String,

    edit_state: button::State,
    copy_state: button::State,
}

impl Password {
    pub fn new(name: String, description: String, key: [u8; 32]) -> Self {
        Self {
            key,
            name,
            description,
            edit_state: button::State::new(),
            copy_state: button::State::new(),
        }
    }

    pub fn view(&mut self) -> Element<Messages> {
        let text = Text::new(&self.name);

        let description = Text::new(&self.description);

        let column = Column::new()
            .width(Length::Fill)
            .push(text)
            .push(description);

        let edit_button = create_link_button(
            &mut self.edit_state,
            None,
            Some("cog.svg"),
            Messages::EditPassword { name: self.name.clone() }
        );

        let copy_button = create_link_button(
            &mut self.copy_state,
            None,
            Some("key.svg"),
            Messages::CopyPassword { name: self.name.clone() }
        );

        let row = Row::new()
            .padding(10)
            .push(column)
            .push(copy_button)
            .push(edit_button)
            .spacing(5)
            .align_items(iced::Align::Center);

        row.into()
    }
}
