use iced::{button, Element, Button, Container, Text, Row, Space, Length, Align};

use crate::{
    messages::Messages,
    styles::{CategoryStyle, ActiveCategoryStyle},
};

pub struct Category {
    pub name: String,
    pub value: Option<String>,
    select_state: button::State,
}

impl Category {
    pub fn new(name: String, value: Option<String>) -> Self {
        Self {
            name,
            value,
            select_state: button::State::new(),
        }
    }

    pub fn view(&mut self, active: bool) -> Element<Messages> {
        let mut border = Container::new(Space::new(Length::Fill, Length::Fill))
            .width(Length::from(3))
            .height(Length::from(40))
            .style(CategoryStyle);

        if active {
            border = border.style(ActiveCategoryStyle);
        }

        let text = Text::new(&self.name)
            .size(18);

        let text_container = Container::new(text)
            .padding(10);

        let row = Row::new()
            .align_items(Align::Center)
            .push(border)
            .push(text_container);

        let mut button = Button::new(&mut self.select_state, row)
            .on_press(Messages::SelectCategory { name: self.value.clone() })
            .padding(0)
            .style(CategoryStyle);

        if active {
            button = button.style(ActiveCategoryStyle);
        }

        Container::new(button)
            .into()
    }
}