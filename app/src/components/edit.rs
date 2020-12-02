use iced::{button, text_input, Column, Element, TextInput};

use crate::{
    components::{create_button, create_widget, create_layout},
    messages::Messages,
    translations::{translate, Languages},
    views::Views,
};

pub struct EditPassword {
    pub entry: String,
    pub name: String,
    pub password: String,

    save_button_state: button::State,
    remove_button_state: button::State,
    back_button_state: button::State,

    name_state: text_input::State,
    password_state: text_input::State,
}

impl EditPassword {
    pub fn new() -> Self {
        Self {
            entry: String::new(),
            name: String::new(),
            password: String::new(),

            save_button_state: button::State::new(),
            remove_button_state: button::State::new(),
            back_button_state: button::State::new(),

            name_state: text_input::State::new(),
            password_state: text_input::State::new(),
        }
    }

    pub fn update_input(&mut self, name: &str, value: String) {
        match name {
            "name" => self.name = value,
            "password" => self.password = value,
            _ => {}
        };
    }

    pub fn reset(&mut self) {
        self.entry = String::new();
        self.name = String::new();
        self.password = String::new();
    }

    pub fn view(&mut self) -> Element<Messages> {
        let back_button = create_button(
            &mut self.back_button_state,
            &translate(Languages::English, "edit.back-button"),
            "back.svg",
            Messages::ChangeView { view: Views::List }
        )
        .padding(5);

        let name_input = TextInput::new(
            &mut self.name_state,
            &translate(Languages::English, "edit.name-placeholder"),
            &self.name,
            |value| Messages::EditViewInputKeyChanged { input: "name", value }
        )
        .padding(10);

        let password_input = TextInput::new(
            &mut self.password_state,
            &translate(Languages::English, "edit.password-placeholder"),
            &self.password,
            |value| Messages::EditViewInputKeyChanged { input: "password", value }
        )
        .padding(10)
        .password();

        let save_button = create_button(
            &mut self.save_button_state,
            &translate(Languages::English, "edit.save-button"),
            "add.svg",
            Messages::UpdatePassword { entry: self.entry.clone(), name: self.name.clone(), password: self.password.clone() }
        );

        let remove_button = create_button(
            &mut self.remove_button_state,
            &translate(Languages::English, "edit.remove-button"),
            "trash.svg",
            Messages::RemovePassword { name: self.name.clone() }
        );

        let content_column = Column::new()
            .spacing(20)
            .push(name_input)
            .push(password_input)
            .push(save_button)
            .push(remove_button);

        let content = create_widget(content_column);

        create_layout(None, Some(back_button.into()), content.into()).into()
    }
}
