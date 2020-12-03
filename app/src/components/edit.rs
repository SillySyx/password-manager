use iced::{button, text_input, Column, Row, Element, TextInput, Text, Length, Space};

use crate::{
    components::{create_button, create_widget, create_layout},
    messages::Messages,
    translations::{translate, Languages},
    views::Views,
};

pub struct EditPassword {
    pub entry: String,
    pub name: String,
    pub description: String,
    pub password: String,

    save_button_state: button::State,
    remove_button_state: button::State,
    back_button_state: button::State,

    name_state: text_input::State,
    description_state: text_input::State,
    password_state: text_input::State,
}

impl EditPassword {
    pub fn new() -> Self {
        Self {
            entry: String::new(),
            name: String::new(),
            description: String::new(),
            password: String::new(),

            save_button_state: button::State::new(),
            remove_button_state: button::State::new(),
            back_button_state: button::State::new(),

            name_state: text_input::State::new(),
            description_state: text_input::State::new(),
            password_state: text_input::State::new(),
        }
    }

    pub fn update_input(&mut self, name: &str, value: String) {
        match name {
            "name" => self.name = value,
            "password" => self.password = value,
            "description" => self.description = value,
            _ => {}
        };
    }

    pub fn reset(&mut self) {
        self.entry = String::new();
        self.name = String::new();
        self.password = String::new();
    }

    pub fn view(&mut self) -> Element<Messages> {
        let header = Text::new(&translate(Languages::English, "edit.header"))
            .size(30);

        let name_title = Text::new(&translate(Languages::English, "edit.name"));

        let name_input = TextInput::new(
            &mut self.name_state,
            "",
            &self.name,
            |value| Messages::EditViewInputKeyChanged { input: "name", value }
        )
        .padding(10);

        let description_title = Text::new(&translate(Languages::English, "edit.description"));

        let description_input = TextInput::new(
            &mut self.description_state,
            "",
            &self.description,
            |value| Messages::EditViewInputKeyChanged { input: "description", value },
        )
        .padding(10);

        let password_title = Text::new(&translate(Languages::English, "edit.password"));

        let password_input = TextInput::new(
            &mut self.password_state,
            "",
            &self.password,
            |value| Messages::EditViewInputKeyChanged { input: "password", value }
        )
        .padding(10)
        .password();

        let save_button = create_button(
            &mut self.save_button_state,
            Some(&translate(Languages::English, "edit.save-button")),
            Some("add.svg"),
            Messages::UpdatePassword { 
                entry: self.entry.clone(), 
                name: self.name.clone(), 
                description: self.description.clone(), 
                password: self.password.clone() 
            }
        );

        let remove_button = create_button(
            &mut self.remove_button_state,
            Some(&translate(Languages::English, "edit.remove-button")),
            Some("trash.svg"),
            Messages::RemovePassword { name: self.name.clone() }
        );

        let back_button = create_button(
            &mut self.back_button_state,
            Some(&translate(Languages::English, "edit.back-button")),
            Some("back.svg"),
            Messages::ChangeView { view: Views::List }
        );

        let button_row = Row::new()
            .push(save_button)
            .push(Space::with_width(Length::Fill))
            .push(back_button);

        let content_column = Column::new()
            .spacing(20)
            .push(header)
            .push(name_title)
            .push(name_input)
            .push(description_title)
            .push(description_input)
            .push(password_title)
            .push(password_input)
            .push(button_row)
            .push(remove_button);

        let content = create_widget(content_column);

        create_layout(None, None, content.into()).into()
    }
}
