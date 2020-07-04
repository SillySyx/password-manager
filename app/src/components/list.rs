use iced::{Element, Column, Row, Text, Container, Length, Align, Scrollable, scrollable, Button, button};

use crate::components::app::Messages;
use crate::components::password::Password;
use crate::translations::{translate, Languages};

pub struct List {
    pub key: [u8; 32],
    passwords: Vec<Password>,

    scrollable_state: scrollable::State,
    add_button_state: button::State,
}

impl List {
    pub fn new() -> List {
        List {
            key: [0u8; 32],
            passwords: vec![],
            scrollable_state: scrollable::State::new(),
            add_button_state: button::State::new(),
        }
    }

    pub fn title(&self) -> String {
        translate(Languages::English, "list.title")
    }

    pub fn view(&mut self) -> Element<Messages> {
        let add_button = Button::new(&mut self.add_button_state, Text::new(translate(Languages::English, "list.add-button")).size(16));
        
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
            .fold(Column::new(), |list, (_, password)| {
                list.push(password.view().map(move |message| {
                    Messages::PasswordMessage(message)
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

    pub fn update_password_list(&mut self) {
        self.passwords.clear();

        for password in list_passwords() {
            self.passwords.push(Password::new(password));
        }
    }
}

fn list_passwords() -> Vec<String> {
    let mut passwords: Vec<String> = Vec::new();

    passwords.push(String::from("pass1"));
    passwords.push(String::from("pass2"));

    passwords.sort();

    passwords
}