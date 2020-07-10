use iced::{Element, Text, Button, button, TextInput, text_input, Container, Column, Length};

use crate::components::app::Messages;
use crate::translations::{translate, Languages};

#[derive(Debug, Clone)]
pub enum UnlockMessages {
    InputKeyChanged(String),
}

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

    pub fn title(&self) -> String {
        translate(Languages::English, "unlock.title")
    }

    pub fn update(&mut self, message: UnlockMessages) {
        match message {
            UnlockMessages::InputKeyChanged(new_key) => self.input_key = new_key,
        }
    }

    pub fn reset(&mut self) {
        self.input_key = String::new();
    }

    pub fn view(&mut self) -> Element<Messages> {
        let header = Text::new(translate(Languages::English, "unlock.header"))
            .size(26);

        let description = Text::new(translate(Languages::English, "unlock.description"))
            .size(18);

        let input = TextInput::new(&mut self.input_state, &translate(Languages::English, "unlock.key-placeholder"), &self.input_key, |message| Messages::UnlockMessage(UnlockMessages::InputKeyChanged(message)))
            .padding(10)
            .on_submit(Messages::UnlockApp)
            .password();

        let button = Button::new(&mut self.button_state, Text::new(translate(Languages::English, "unlock.unlock-button")))
            .on_press(Messages::UnlockApp);

        let content = Column::new()
            .spacing(20)
            .max_width(300)
            .push(header)
            .push(description)
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