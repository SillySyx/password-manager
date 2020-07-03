use std::path::{Path, PathBuf};

use iced::{Element, Column, Row, Text, Container, Length, Align, Scrollable, scrollable, Button, button};

use crate::components::app::Messages;
use crate::translations::translate;
use crate::config::read_app_path;

pub struct List {
    pub key: [u8; 32],
    passwords: Vec<String>,

    scrollable_state: scrollable::State,
    add_button_state: button::State,
    read_button_state: button::State,
    remove_button_state: button::State,
}

impl List {
    pub fn new() -> List {
        List {
            key: [0u8; 32],
            passwords: vec![],
            scrollable_state: scrollable::State::new(),
            add_button_state: button::State::new(),
            read_button_state: button::State::new(),
            remove_button_state: button::State::new(),
        }
    }

    pub fn title(&self) -> String {
        translate("list.title")
    }

    pub fn view(&mut self) -> Element<Messages> {
        let add_button = Button::new(&mut self.add_button_state, Text::new(translate("list.add-button")).size(16));
        
        let title = Text::new(translate("list.title"))
            .size(28)
            .width(Length::Fill);

        let header_row = Row::<Messages>::new()
            .push(title)
            .push(add_button);

        let list = self.passwords
            .iter()
            .enumerate()
            .fold(Column::new(), |list, (_, password)| {
                list.push(build_password_element(password.clone()))
            });
            
        let content = Scrollable::new(&mut self.scrollable_state)
            .width(Length::Fill)
            .align_items(Align::Center)
            .push(list);

        let layout = Column::new()
            .max_width(500)
            .spacing(20)
            .push(header_row)
            .push(content);

        Container::new(layout)            
            .width(Length::Fill)
            .center_x()
            .into()
    }

    pub fn update_password_list(&mut self) -> Result<(), String> {
        let password_path = password_path()?;
        let passwords = list_passwords(&password_path)?;

        self.passwords = passwords;

        Ok(())
    }
}

fn password_path() -> Result<PathBuf, String> {
    let mut app_path = read_app_path()?;
    app_path.push("passwords");

    if !app_path.exists() {
        match std::fs::create_dir(app_path.clone()) {
            Ok(_) => {},
            Err(_) => return Err(String::from("failed to create password folder")),
        }
    }

    Ok(app_path)
}

fn list_passwords(path: &Path) -> Result<Vec<String>, String> {
    let mut files: Vec<String> = Vec::new();

    let dir = match std::fs::read_dir(path) {
        Ok(dir) => dir,
        Err(_) => return Err(String::from("Failed to read dir")),
    };

    for entry in dir {
        let name = entry.unwrap().file_name().into_string().unwrap();
        files.push(name);
    }

    files.sort();

    Ok(files)
}

fn build_password_element<'a>(password: String) -> Element<'a, Messages> {
    Row::new()
        .push(Text::new(password).width(Length::Fill))
        .into()
}