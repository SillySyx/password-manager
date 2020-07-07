use iced::{Element, Column, Row, Text, Container, Length, Align, Scrollable, scrollable, Button, button};

use crate::components::app::{Messages, Views};
use crate::components::password::{Password, PasswordMessages};
use crate::translations::{translate, Languages};

use crate::datastore::load_datastore;

use std::error::Error;

pub struct List {
    pub key: [u8; 32],
    passwords: Vec<Password>,

    scrollable_state: scrollable::State,
    add_button_state: button::State,
}

impl List {
    pub fn new() -> Self {
        Self {
            key: [0; 32],
            passwords: vec![],
            scrollable_state: scrollable::State::new(),
            add_button_state: button::State::new(),
        }
    }

    pub fn title(&self) -> String {
        translate(Languages::English, "list.title")
    }

    pub fn update(&mut self, index: usize, message: PasswordMessages) {
        self.passwords[index].update(message);

        match self.update_password_list() {
            Ok(_) => {},
            Err(_) => {},
        };
    }

    pub fn view(&mut self) -> Element<Messages> {
        let add_button = Button::new(&mut self.add_button_state, Text::new(translate(Languages::English, "list.add-button")).size(16))
            .on_press(Messages::ChangeView(Views::AddPassword));
        
        let title = Text::new(translate(Languages::English, "list.title"))
            .size(28)
            .width(Length::Fill);

        let header_row = Row::<Messages>::new()
            .push(title)
            .push(add_button)
            .padding(20);

        let list = self.passwords
            .iter_mut()
            .enumerate()
            .fold(Column::new(), |list, (index, password)| {
                list.push(password.view().map(move |message| {
                    Messages::PasswordMessage(index, message)
                }))
            })
            .padding(20)
            .spacing(5);
            
        let content = Scrollable::new(&mut self.scrollable_state)
            .width(Length::Fill)
            .align_items(Align::Center)
            .push(list);

        let layout = Column::new()
            .max_width(500)
            .push(header_row)
            .push(content);

        Container::new(layout)            
            .width(Length::Fill)
            .center_x()
            .into()
    }

    pub fn update_password_list(&mut self) -> Result<(), Box<dyn Error>> {
        self.passwords.clear();

        for password in list_passwords(&self.key)? {
            self.passwords.push(Password::new(password, self.key));
        }

        Ok(())
    }
}

fn list_passwords(key: &[u8]) -> Result<Vec<String>, Box<dyn Error>> {
    let mut passwords: Vec<String> = Vec::new();

    let glob = load_datastore(key)?;

    for entry in glob.entries {
        passwords.push(entry.name);
    }

    passwords.sort();

    Ok(passwords)
}