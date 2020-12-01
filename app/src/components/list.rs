use std::error::Error;
use iced::{button, scrollable, Align, Column, Container, Element, Length, Row, Scrollable, Text};

use crate::{
    components::{create_button, Password},
    datastore::load_eventlog,
    messages::Messages,
    styles::HeaderStyle,
    translations::{translate, Languages},
    views::Views,
    states::PasswordsState,
};

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

    pub fn view(&mut self) -> Element<Messages> {
        let header_title = Text::new(translate(Languages::English, "list.header"))
            .width(Length::Fill)
            .vertical_alignment(iced::VerticalAlignment::Center)
            .size(26);

        let add_button = create_button(
            &mut self.add_button_state,
            &translate(Languages::English, "list.add-button"),
            Messages::ChangeView { view: Views::AddPassword }
        );

        let header_row = Row::new()
            .max_width(500)
            .height(Length::Units(35))
            .push(header_title)
            .push(add_button);

        let header_container = Container::new(header_row)
            .padding(10)
            .width(Length::Fill)
            .center_x()
            .style(HeaderStyle);

        let content = self
            .passwords
            .iter_mut()
            .enumerate()
            .fold(Column::new(), |list, (_, password)| {
                list.push(password.view())
            })
            .spacing(5)
            .max_width(500);

        let content_scroller = Scrollable::new(&mut self.scrollable_state)
            .push(content)
            .width(Length::Fill)
            .align_items(Align::Center);

        let content_container = Container::new(content_scroller)
            .height(Length::Fill);

        Column::new()
            .align_items(Align::Center)
            .spacing(5)
            .push(header_container)
            .push(content_container)
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
    let eventlog = load_eventlog(key)?;

    let initial_state = PasswordsState::new();
    let state = eventlog.project(initial_state);

    let mut passwords = state
        .passwords
        .iter()
        .fold(vec![], |mut passwords, password| {
            passwords.push(password.name.clone());

            passwords
        });

    passwords.sort();

    Ok(passwords)
}
