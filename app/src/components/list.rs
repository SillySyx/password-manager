use std::error::Error;
use iced::{Column, Container, Element, Length, Row, Scrollable, button, scrollable, TextInput, text_input};

use crate::{
    components::{create_link_button, create_layout, Password, Category},
    datastore::load_eventlog,
    messages::Messages,
    translations::{translate, Languages},
    views::Views,
    states::PasswordsState,
};

pub struct List {
    pub key: [u8; 32],
    passwords: Vec<Password>,
    categories: Vec<Category>,
    selected_category: Option<String>,
    search_text: String,

    search_text_state: text_input::State,
    categories_state: scrollable::State,
    passwords_state: scrollable::State,
    add_button_state: button::State,
}

impl List {
    pub fn new() -> Self {
        Self {
            key: [0; 32],
            passwords: vec![],
            categories: vec![],
            selected_category: None,
            search_text: String::from(""),
            search_text_state: text_input::State::new(),
            categories_state: scrollable::State::new(),
            passwords_state: scrollable::State::new(),
            add_button_state: button::State::new(),
        }
    }

    pub fn view(&mut self) -> Element<Messages> {
        let add_button = create_link_button(
            &mut self.add_button_state,
            Some(&translate(Languages::English, "list.add-password")),
            Some("add.svg"),
            Messages::ChangeView { view: Views::AddPassword }
        );

        let selected_category = self.selected_category.clone();

        let categories_container = self.categories
            .iter_mut()
            .fold(Scrollable::new(&mut self.categories_state), |container, category| {
                container.push(category.view(selected_category == category.value.clone()))
            })
            .padding(15);

        let passwords_container = Scrollable::new(&mut self.passwords_state);

        let passwords_container = self.passwords
            .iter_mut()
            .enumerate()
            .fold(passwords_container, |container, (index, password)| {
                let alternated_row = index % 2 == 0;
                container.push(password.view(alternated_row))
            });

        let search_box = TextInput::new(
            &mut self.search_text_state, 
            &translate(Languages::English, "list.search"), 
            &self.search_text, 
            |value| Messages::ListViewInputKeyChanged { input: "search", value }
        )
        .padding(10);

        let content = Column::new()
            .push(search_box)
            .push(passwords_container);

        let content = Container::new(content)
            .padding(5);

        let content_container = Row::new()
            .height(Length::Fill)
            .push(categories_container)
            .push(content);

        create_layout(None, Some(add_button.into()), content_container.into()).into()
    }

    pub fn update_password_list(&mut self) -> Result<(), Box<dyn Error>> {
        let passwords = list_passwords(self.key, self.search_text.clone())?;
        
        self.categories = read_categories_from_passwords(&passwords);

        self.passwords = filter_passwords_based_on_selected_category(passwords, self.selected_category.clone());

        Ok(())
    }

    pub fn select_category(&mut self, name: Option<String>) {
        if name == self.selected_category {
            self.selected_category = None;
            self.update_password_list().expect("failed to update list");
            return;
        }

        self.selected_category = name;
        self.update_password_list().expect("failed to update list");
    }

    pub fn update_input(&mut self, name: &str, value: String) {
        match name {
            "search" => self.search_text = value,
            _ => {}
        };
    }
}

fn list_passwords(key: [u8; 32], search_text: String) -> Result<Vec<Password>, Box<dyn Error>> {
    let eventlog = load_eventlog(&key)?;

    let initial_state = PasswordsState::new();
    let state = eventlog.project(initial_state);

    let mut passwords: Vec<Password> = state.passwords
        .iter()
        .map(|password| Password::new(password.name.clone(), password.description.clone(), password.category.clone()))
        .filter(|password| {
            if search_text.is_empty() {
                return true;
            }

            password.name.to_lowercase().contains(search_text.to_lowercase().as_str())
        })
        .collect();

    passwords.sort_by_key(|password| password.name.to_lowercase());

    Ok(passwords)
}

fn read_categories_from_passwords(passwords: &[Password]) -> Vec<Category> {
    let mut categories: Vec<Category> = passwords
        .iter()
        .filter(|password| !password.category.is_empty())
        .map(|password| Category::new(password.category.clone(), Some(password.category.clone())))
        .collect();

    categories.sort_by_key(|category| category.name.to_lowercase());
    categories.dedup_by_key(|category| category.name.clone());

    categories
}

fn filter_passwords_based_on_selected_category(passwords: Vec<Password>, selected_category: Option<String>) -> Vec<Password> {
    if let Some(selected_category) = selected_category {
        return passwords
            .into_iter()
            .filter(|password| password.category == selected_category)
            .collect();
    }

    passwords
}
