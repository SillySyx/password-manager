use iced::{Element, Text};

use super::app::{Messages};

pub struct List {
}

impl List {
    pub fn new() -> List {
        List {
        }
    }

    pub fn title(&self) -> String {
        String::from("List")
    }

    pub fn view(&mut self) -> Element<Messages> {
        Text::new("List view bru!").into()
    }
}