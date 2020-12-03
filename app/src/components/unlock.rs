use iced::{button, text_input, Column, Element, Text, TextInput};

use crate::{
    components::{create_button, create_widget, create_layout},
    messages::Messages,
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
            input_state: {
                let mut state = text_input::State::new();
                state.focus();
                state
            },
        }
    }

    pub fn reset(&mut self) {
        self.input_key = String::new();
    }

    pub fn view(&mut self) -> Element<Messages> {
        let description = Text::new(translate(Languages::English, "unlock.description")).size(18);

        let input = TextInput::new(
            &mut self.input_state,
            "",
            &self.input_key,
            |value| Messages::UnlockViewInputKeyChanged { value }
        )
        .padding(10)
        .on_submit(Messages::UnlockApp { key: self.input_key.clone() })
        .password();

        let button = create_button(
            &mut self.button_state,
            Some(&translate(Languages::English, "unlock.unlock-button")),
            Some("unlock.svg"),
            Messages::UnlockApp { key: self.input_key.clone() }
        );

        let content_column = Column::new()
            .spacing(20)
            .push(description)
            .push(input)
            .push(button);

        let content = create_widget(content_column);

        create_layout(None, None, content.into()).into()
    }
}
