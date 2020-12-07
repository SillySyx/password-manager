use iced::{button, Element, Button, Container, Text };

use crate::{
    messages::Messages,
};

pub struct Category {
    pub name: String,
    select_state: button::State,
}

impl Category {
    pub fn new(name: String) -> Self {
        Self {
            name,
            select_state: button::State::new(),
        }
    }

    pub fn view(&mut self) -> Element<Messages> {
        let text = Text::new(&self.name);

        let button = Button::new(&mut self.select_state, text)
            .on_press(Messages::ToggleCategory { name: self.name.clone() });

        let container = Container::new(button);

        container.into()
    }
}
