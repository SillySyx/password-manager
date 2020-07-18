use iced::{Column, Container, Element, Length, Text};

use crate::{
    components::create_widget,
    messages::Messages,
    translations::{translate, Languages},
};

pub struct Error {}

impl Error {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&mut self) -> Element<Messages> {
        let header = Text::new(translate(Languages::English, "error.header")).size(26);

        let description = Text::new(translate(Languages::English, "error.description")).size(18);

        let content = Column::new()
            .spacing(20)
            .max_width(500)
            .push(header)
            .push(description);

        let content = create_widget(content);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
