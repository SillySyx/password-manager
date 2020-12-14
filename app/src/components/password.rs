use iced::{button, Element, Length, Row, Container, Align};

use crate::{
    components::create_link_button,
    messages::Messages,
    styles::{PasswordStyle, AlternatedPasswordStyle},
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

    pub fn view(&mut self, alternated_row: bool) -> Element<Messages> {
        let text_button = create_link_button(
            &mut self.text_state,
            Some(&self.name),
            None,
            Messages::EditPassword { name: self.name.clone() }
        )
        .style(PasswordStyle);

        let desciption = iced::Text::new(self.description.clone())
            .color(iced::Color::from_rgb(0.3, 0.3, 0.3))
            .size(16);

        let description_row = Row::new()
            .push(iced::Space::with_width(Length::from(5)))
            .push(desciption);

        let text_container = iced::Column::new()
            .width(Length::Fill)
            .push(text_button)
            .push(description_row);

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

        let container = Container::new(row)
            .padding(15);

        if alternated_row {
            return container.style(AlternatedPasswordStyle).into();
        }

        container.style(PasswordStyle).into()
    }
}
