use iced::{Element, Sandbox};

use super::unlock::Unlock;
use super::list::List;

pub struct App {
    key: String,

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
    Unlock,
    UnlockKeyChanged(String),
}

impl Sandbox for App {
    type Message = Messages;

    fn new() -> App {
        App {
            key: String::new(),
            view: Views::Unlock,
            list_view: List::new(),
            unlock_view: Unlock::new(),
        }
    }

    fn title(&self) -> String {
        match self.view {
            Views::Unlock => self.unlock_view.title(),
            Views::List => self.list_view.title(),
        }
    }

    fn update(&mut self, message: Messages) {
        match message {
            Messages::Unlock => self.view = Views::List,
            Messages::UnlockKeyChanged(key) => self.key = key,
        }
    }

    fn view(&mut self) -> Element<Messages> {
        match self.view {
            Views::Unlock => self.unlock_view.view(),
            Views::List => self.list_view.view(),
        }
    }
}