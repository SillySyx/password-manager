use iced::{button, text_input, Column, Element, Length, Row, Text, TextInput, Space};

use crate::{
    components::{create_button, create_widget, create_layout},
    messages::Messages,
    translations::{translate, Languages},
    views::Views,
    passphrase::generate_passphrase,
};

pub struct AddPassword {
    pub name: String,
    pub description: String,
    pub category: String,
    pub password: String,

    add_button_state: button::State,
    back_button_state: button::State,
    passphrase_button_state: button::State,
    name_state: text_input::State,
    description_state: text_input::State,
    category_state: text_input::State,
    password_state: text_input::State,
}

impl AddPassword {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            category: String::new(),
            password: String::new(),

            add_button_state: button::State::new(),
            back_button_state: button::State::new(),
            passphrase_button_state: button::State::new(),
            name_state: {
                let mut state = text_input::State::new();
                state.focus();
                state
            },
            description_state: text_input::State::new(),
            category_state: text_input::State::new(),
            password_state: text_input::State::new(),
        }
    }

    pub fn update_input(&mut self, name: &str, value: String) {
        match name {
            "name" => self.name = value,
            "password" => self.password = value,
            "description" => self.description = value,
            "category" => self.category = value,
            _ => {}
        };
    }

    pub fn reset(&mut self) {
        self.name = String::new();
        self.description = String::new();
        self.category = String::new();
        self.password = String::new();
        self.name_state.focus();
    }

    pub fn view(&mut self) -> Element<Messages> {
        let header = Text::new(&translate(Languages::English, "add.header"))
            .size(30);
        
        let name_title = Text::new(&translate(Languages::English, "add.name"));

        let name_input = TextInput::new(
            &mut self.name_state,
            "",
            &self.name,
            |value| Messages::AddViewInputKeyChanged { input: "name", value },
        )
        .padding(10);

        let description_title = Text::new(&translate(Languages::English, "add.description"));

        let description_input = TextInput::new(
            &mut self.description_state,
            "",
            &self.description,
            |value| Messages::AddViewInputKeyChanged { input: "description", value },
        )
        .padding(10);

        let category_title = Text::new(&translate(Languages::English, "add.category"));

        let category_input = TextInput::new(
            &mut self.category_state,
            "",
            &self.category,
            |value| Messages::AddViewInputKeyChanged { input: "category", value },
        )
        .padding(10);

        let password_title = Text::new(&translate(Languages::English, "add.password"));

        let password_input = TextInput::new(
            &mut self.password_state,
            "",
            &self.password,
            |value| Messages::AddViewInputKeyChanged{ input: "password", value },
        )
        .padding(10)
        .width(Length::Fill);

        let generate_passphrase_button = create_button(
            &mut self.passphrase_button_state,
            None,
            Some("generate.svg"),
            Messages::GeneratePassphraseForAddView,
        )
        .padding(13);

        let password_row = Row::new()
            .spacing(5)
            .push(password_input)
            .push(generate_passphrase_button);

        let add_button = create_button(
            &mut self.add_button_state,
            Some(&translate(Languages::English, "add.add-button")),
            Some("add.svg"),
            Messages::AddPasswordMessage { 
                name: self.name.clone(), 
                description: self.description.clone(), 
                category: self.category.clone(),
                password: self.password.clone(),
            },
        );

        let back_button = create_button(
            &mut self.back_button_state,
            Some(&translate(Languages::English, "add.back-button")),
            Some("back.svg"),
            Messages::ChangeView { view: Views::List },
        );

        let button_row = Row::new()
            .push(add_button)
            .push(Space::with_width(Length::Fill))
            .push(back_button);

        let content_column = Column::new()
            .spacing(20)
            .push(header)
            .push(name_title)
            .push(name_input)
            .push(description_title)
            .push(description_input)
            .push(category_title)
            .push(category_input)
            .push(password_title)
            .push(password_row)
            .push(button_row);

        let content = create_widget(content_column);

        create_layout(None, None, content.into()).into()
    }

    pub fn generate_passphrase(&mut self) {
        if let Ok(passphrase) = generate_passphrase() {
            self.password = passphrase;
        }
    }
}