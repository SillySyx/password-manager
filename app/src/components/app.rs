use iced::{Element, Sandbox};

use super::unlock::{Unlock, UnlockMessages};
use super::password::PasswordMessages;
use super::list::List;
use super::super::translations::translate;

use crypto::generate_key_from_seed;

pub struct App {
    view: Views,

    list_view: List,
    unlock_view: Unlock,
}

#[derive(Debug, Clone)]
pub enum Views {
    Unlock,
    List,
}

#[derive(Debug, Clone)]
pub enum Messages {
    UnlockApp,
    UnlockMessage(UnlockMessages),
}

impl Sandbox for App {
    type Message = Messages;

    fn new() -> App {
        App {
            view: Views::Unlock,
            list_view: List::new(),
            unlock_view: Unlock::new(),
        }
    }

    fn title(&self) -> String {
        let title = match self.view {
            Views::Unlock => self.unlock_view.title(),
            Views::List => self.list_view.title(),
        };

        format!("{} - {}", title, translate("app.name"))
    }

    fn update(&mut self, message: Messages) {
        match message {
            Messages::UnlockApp => { self.unlock_app().unwrap_or_default(); },
            Messages::UnlockMessage(unlock_message) => self.unlock_view.update(unlock_message),
        }
    }

    fn view(&mut self) -> Element<Messages> {
        match self.view {
            Views::Unlock => self.unlock_view.view(),
            Views::List => self.list_view.view(),
        }
    }
}

impl App {
    fn unlock_app(&mut self) -> Result<(), String> {
        if self.unlock_view.input_key.is_empty() {
            return Err(String::from("No key specified"));
        }

        let key = generate_key_from_seed(&self.unlock_view.input_key)?;

        self.unlock_view.input_key = String::new();
        self.list_view.key = key;

        self.list_view.update_password_list()?;

        self.view = Views::List;

        Ok(())
    }
}