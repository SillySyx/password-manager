use iced::{Element, Text, Row, Length, button, Button};

use crate::translations::{translate, Languages};

#[derive(Debug, Clone)]
pub enum PasswordMessages {
    CopyPassword(String),
    RemovePassword(String),
}

pub struct Password {
    pub name: String,

    copy_state: button::State,
    remove_state: button::State,
}

impl Password {
    pub fn new(name: String) -> Password {
        Password {
            name,
            copy_state: button::State::new(),
            remove_state: button::State::new(),
        }
    }

    pub fn update(&mut self, message: PasswordMessages) {
        match message {
            PasswordMessages::CopyPassword(password) => { println!("copy {}", password) },
            PasswordMessages::RemovePassword(password) => { println!("remove {}", password) },
        }
    }

    pub fn view(&mut self) -> Element<PasswordMessages> {
        let text = Text::new(&self.name)
            .width(Length::Fill);

        let copy_button = Button::new(&mut self.copy_state, Text::new(translate(Languages::English, "password.copy-button")).size(16))
            .on_press(PasswordMessages::CopyPassword(self.name.clone()));

        let remove_button = Button::new(&mut self.remove_state, Text::new(translate(Languages::English, "password.remove-button")).size(16))
            .on_press(PasswordMessages::CopyPassword(self.name.clone()));

        Row::new()
            .push(text)
            .push(copy_button)
            .push(remove_button)
            .spacing(5)
            .into()
    }
}