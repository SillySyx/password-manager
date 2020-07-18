use iced::{button, text_input, Column, Container, Element, Length, Row, Text, TextInput};

use crate::{
    components::{create_button, create_widget},
    messages::Messages,
    styles::HeaderStyle,
    translations::{translate, Languages},
};

pub struct Unlock {
    pub input_key: String,

    button_state: button::State,
    input_state: text_input::State,
}

impl Unlock {
    pub fn new() -> Self {
        Self {
            input_key: String::new(),
            button_state: button::State::new(),
            input_state: text_input::State::new(),
        }
    }

    pub fn reset(&mut self) {
        self.input_key = String::new();
    }

    pub fn view(&mut self) -> Element<Messages> {
        let header_title = Text::new(translate(Languages::English, "unlock.header"))
            .width(Length::Fill)
            .vertical_alignment(iced::VerticalAlignment::Center)
            .size(26);

        let header_row = Row::new()
            .max_width(500)
            .height(Length::Units(35))
            .push(header_title);

        let header_container = Container::new(header_row)
            .padding(10)
            .width(Length::Fill)
            .center_x()
            .style(HeaderStyle);

        let description = Text::new(translate(Languages::English, "unlock.description")).size(16);

        let input = TextInput::new(
            &mut self.input_state,
            &translate(Languages::English, "unlock.key-placeholder"),
            &self.input_key,
            |value| Messages::UnlockViewInputKeyChanged { value }
        )
        .padding(10)
        .on_submit(Messages::UnlockApp { key: self.input_key.clone() })
        .password();

        let button = create_button(
            &mut self.button_state,
            &translate(Languages::English, "unlock.unlock-button"),
            Messages::UnlockApp { key: self.input_key.clone() }
        );

        let content_column = Column::new()
            .spacing(20)
            .push(description)
            .push(input)
            .push(button);

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
