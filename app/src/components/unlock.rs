use iced::{Element, Text, Button, button, TextInput, text_input, Container, Column, Length};

use super::app::{Messages};

pub struct Unlock {
    bs: button::State,
    is: text_input::State,
}

impl Unlock {
    pub fn new() -> Unlock {
        Unlock { 
            bs: button::State::new(),
            is: text_input::State::new(),
        }
    }

    pub fn title(&self) -> String {
        String::from("Unlock")
    }

    pub fn view(&mut self) -> Element<Messages> {
        let text = Text::new("Unlock view bru!");

        let input = TextInput::new(&mut self.is, "Enter Password", "", Messages::UnlockKeyChanged);

        let button = Button::new(&mut self.bs, Text::new("Ok"))
            .width(Length::Fill)
            .on_press(Messages::Unlock);

        let content = Column::new()
            .spacing(20)
            .max_width(300)
            .push(text)
            .push(input)
            .push(button);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}