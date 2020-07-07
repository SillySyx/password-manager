use iced::{Element, Sandbox};

use crate::components::unlock::{Unlock, UnlockMessages};
use crate::components::password::PasswordMessages;
use crate::components::list::List;
use crate::components::error::Error;
use crate::components::addpassword::AddPassword;
use crate::translations::{translate, Languages};

use crypto::generate_key_from_seed;

pub struct App {
    view: Views,

    list_view: List,
    unlock_view: Unlock,
    add_view: AddPassword,
    error_view: Error,
}

#[derive(Debug, Clone)]
pub enum Views {
    Unlock,
    List,
    AddPassword,
    Error,
}

#[derive(Debug, Clone)]
pub enum Messages {
    ChangeView(Views),
    UnlockApp,
    AddPasswordMessage,
    AddPasswordUpdateInputMessage(&'static str, String),
    UnlockMessage(UnlockMessages),
    PasswordMessage(usize, PasswordMessages),
}

impl Sandbox for App {
    type Message = Messages;

    fn new() -> Self {
        Self {
            view: Views::Unlock,
            list_view: List::new(),
            unlock_view: Unlock::new(),
            add_view: AddPassword::new(),
            error_view: Error::new(),
        }
    }

    fn title(&self) -> String {
        let title = match self.view {
            Views::Unlock => self.unlock_view.title(),
            Views::List => self.list_view.title(),
            Views::AddPassword => self.add_view.title(),
            Views::Error => self.error_view.title(),
        };

        format!("{} - {}", title, translate(Languages::English, "app.name"))
    }

    fn update(&mut self, message: Messages) {
        match message {
            Messages::ChangeView(view) => self.change_view(view),
            Messages::UnlockApp => self.unlock_app(),
            Messages::UnlockMessage(message) => self.unlock_view.update(message),
            Messages::PasswordMessage(index, password_message) => self.list_view.update(index, password_message),
            Messages::AddPasswordMessage => self.add_password(),
            Messages::AddPasswordUpdateInputMessage(name, value) => self.add_view.update_input(name, value),
        }
    }

    fn view(&mut self) -> Element<Messages> {
        match self.view {
            Views::Unlock => self.unlock_view.view(),
            Views::List => self.list_view.view(),
            Views::AddPassword => self.add_view.view(),
            Views::Error => self.error_view.view(),
        }
    }
}

impl App {
    fn change_view(&mut self, view: Views) {
        match view {
            Views::AddPassword => {
                self.add_view.reset();
            },
            _ => {},
        };

        self.view = view;
    }

    fn unlock_app(&mut self) {
        if self.unlock_view.input_key.is_empty() {
            return;
        }

        let key = match generate_key_from_seed(&self.unlock_view.input_key) {
            Ok(key) => key,
            Err(_) => {
                self.unlock_view.reset();
                return;
            },
        };

        self.unlock_view.reset();

        self.list_view.key = key;
        self.add_view.key = key;

        match self.list_view.update_password_list() {
            Ok(_) => self.change_view(Views::List),
            Err(_) => self.change_view(Views::Error),
        };
    }

    fn add_password(&mut self) {
        if self.add_view.name.is_empty() {
            return;
        }
        
        if self.add_view.password.is_empty() {
            return;
        }

        match self.add_view.add_new_password(self.add_view.name.clone(), self.add_view.password.clone()) {
            Ok(_) => {},
            Err(_) => self.change_view(Views::Error),
        };

        match self.list_view.update_password_list() {
            Ok(_) => self.change_view(Views::List),
            Err(_) => self.change_view(Views::Error),
        };
    }
}