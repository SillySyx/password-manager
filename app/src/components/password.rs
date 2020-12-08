use iced::{button, Element, Length, Row, Container, Align};

use crate::{
    components::create_link_button,
    messages::Messages,
    styles::PasswordStyle,
};

#[derive(Clone)]
pub struct Password {
    pub name: String,
    pub description: String,
    pub category: String,

    text_state: button::State,
    edit_state: button::State,
    copy_state: button::State,
}

impl Password {
    pub fn new(name: String, description: String, category: String) -> Self {
        Self {
            name,
            description,
            category,
            text_state: button::State::new(),
            edit_state: button::State::new(),
            copy_state: button::State::new(),
        }
    }

    pub fn view(&mut self) -> Element<Messages> {
        let text_button = create_link_button(
            &mut self.text_state,
            Some(&self.name),
            None,
            Messages::EditPassword { name: self.name.clone() }
        )
        .style(PasswordStyle);

        let text_container = Container::new(text_button)
            .width(Length::Fill);

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
            .push(text_container)
            .push(copy_button)
            .push(edit_button)
            .spacing(5)
            .align_items(Align::Center);

        Container::new(row)
            .padding(5)
            .style(PasswordStyle)
            .into()
    }
}
