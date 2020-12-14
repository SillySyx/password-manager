use iced::{Element, Sandbox};

use crate::{
    clipboard::copy_value_to_clipboard,
    components::{AddPassword, EditPassword, Error, List, Unlock},
    datastore::{append_event_to_eventlog, load_eventlog},
    events::{AddPasswordEvent, ChangeNameEvent, ChangeDescriptionEvent, ChangeCategoryEvent, ChangePasswordEvent, RemovePasswordEvent},
    messages::Messages,
    states::{Password, PasswordsState},
    translations::{translate, Languages},
    views::Views,
};

use crypto::generate_key_from_seed;

pub struct App {
    key: [u8; 32],

    view: Views,

    list_view: List,
    unlock_view: Unlock,
    add_view: AddPassword,
    error_view: Error,
    edit_view: EditPassword,
}

impl Sandbox for App {
    type Message = Messages;

    fn new() -> Self {
        Self {
            key: [0u8; 32],
            view: Views::Unlock,
            list_view: List::new(),
            unlock_view: Unlock::new(),
            add_view: AddPassword::new(),
            error_view: Error::new(),
            edit_view: EditPassword::new(),
        }
    }

    fn title(&self) -> String {
        translate(Languages::English, "app.name")
    }

    fn update(&mut self, message: Messages) {
        match message {
            Messages::ChangeView { view } => self.change_view(view),
            Messages::UnlockApp { key } => self.unlock_app(key),
            Messages::AddPasswordMessage { name, description, category, password } => self.add_password(name, description, category, password),
            Messages::EditPassword { name } => self.edit_password(name),
            Messages::CopyPassword { name } => self.copy_password(name),
            Messages::CopyDescription { name } => self.copy_description(name),
            Messages::RemovePassword { name } => self.remove_password(name),
            Messages::AddViewInputKeyChanged { input, value } => {
                self.add_view.update_input(input, value)
            }
            Messages::EditViewInputKeyChanged { input, value } => {
                self.edit_view.update_input(input, value)
            }
            Messages::UnlockViewInputKeyChanged { value } => self.unlock_view.input_key = value,
            Messages::UpdatePassword { entry, name, description, category, password} => self.update_password(entry, name, description, category, password),
            Messages::GeneratePassphraseForAddView => self.add_view.generate_passphrase(),
            Messages::SelectCategory { name } => self.list_view.select_category(name),
        }
    }

    fn view(&mut self) -> Element<Messages> {
        match self.view {
            Views::Unlock => self.unlock_view.view(),
            Views::List => self.list_view.view(),
            Views::AddPassword => self.add_view.view(),
            Views::Error => self.error_view.view(),
            Views::Edit => self.edit_view.view(),
        }
    }
}

impl App {
    fn change_view(&mut self, view: Views) {
        match view {
            Views::AddPassword => {
                self.add_view.reset();
            }
            _ => {}
        };

        self.view = view;
    }

    fn unlock_app(&mut self, key: String) {
        if key.is_empty() {
            return;
        }

        let key = match generate_key_from_seed(&key) {
            Ok(key) => key,
            Err(_) => {
                self.unlock_view.reset();
                return;
            }
        };

        self.unlock_view.reset();

        self.list_view.key = key;
        self.key = key;

        match self.list_view.update_password_list() {
            Ok(_) => self.change_view(Views::List),
            Err(_) => self.change_view(Views::Error),
        };
    }

    fn add_password(&mut self, name: String, description: String, category: String, password: String) {
        if name.is_empty() {
            return;
        }

        if password.is_empty() {
            return;
        }

        let event = AddPasswordEvent { name, description, category, password };

        match append_event_to_eventlog(event.as_event(), &self.key) {
            Ok(_) => {}
            Err(_) => {
                self.change_view(Views::Error);
                return;
            }
        };

        match self.list_view.update_password_list() {
            Ok(_) => {}
            Err(_) => {
                self.change_view(Views::Error);
                return;
            }
        };

        self.change_view(Views::List);
    }

    fn edit_password(&mut self, name: String) {
        self.edit_view.reset();

        let password = match find_password(&self.key, &name) {
            Some(value) => value,
            None => {
                self.change_view(Views::Error);
                return;
            }
        };

        self.edit_view.entry = name.clone();
        self.edit_view.name = password.name.clone();
        self.edit_view.description = password.description.clone();
        self.edit_view.category = password.category.clone();
        self.edit_view.password = password.password.clone();

        self.change_view(Views::Edit);
    }

    fn copy_password(&mut self, name: String) {
        let password = match read_password(&self.key, &name) {
            Ok(value) => value,
            Err(_) => {
                self.change_view(Views::Error);
                return;
            }
        };

        match copy_value_to_clipboard(password) {
            Ok(_) => {}
            Err(_) => {}
        };
    }

    fn copy_description(&mut self, name: String) {
        let entry = match find_password(&self.key, &name) {
            Some(value) => value,
            None => {
                self.change_view(Views::Error);
                return;
            },
        };

        match copy_value_to_clipboard(entry.description) {
            Ok(_) => {}
            Err(_) => {}
        };
    }

    fn remove_password(&mut self, name: String) {
        let event = RemovePasswordEvent { name };

        match append_event_to_eventlog(event.as_event(), &self.key) {
            Ok(_) => {}
            Err(_) => {
                self.change_view(Views::Error);
                return;
            }
        };

        match self.list_view.update_password_list() {
            Ok(_) => {}
            Err(_) => {
                self.change_view(Views::Error);
                return;
            }
        };

        self.change_view(Views::List);
    }

    fn update_password(&mut self, entry: String, name: String, description: String, category: String, password: String) {
        let entry = match find_password(&self.key, &entry) {
            Some(value) => value,
            None => return,
        };

        if entry.name != name {
            let event = ChangeNameEvent {
                name: entry.name.clone(),
                new_name: name.clone(),
            };

            match append_event_to_eventlog(event.as_event(), &self.key) {
                Ok(_) => {}
                Err(_) => {
                    self.change_view(Views::Error);
                    return;
                }
            };
        }

        if entry.description != description {
            let event = ChangeDescriptionEvent {
                name: entry.name.clone(),
                new_description: description.clone(),
            };

            match append_event_to_eventlog(event.as_event(), &self.key) {
                Ok(_) => {}
                Err(_) => {
                    self.change_view(Views::Error);
                    return;
                }
            };
        }

        if entry.category != category {
            let event = ChangeCategoryEvent {
                name: entry.name.clone(),
                new_category: category.clone(),
            };

            match append_event_to_eventlog(event.as_event(), &self.key) {
                Ok(_) => {}
                Err(_) => {
                    self.change_view(Views::Error);
                    return;
                }
            };
        }

        if entry.password != password {
            let event = ChangePasswordEvent {
                name: entry.name.clone(),
                new_password: password,
            };
    
            match append_event_to_eventlog(event.as_event(), &self.key) {
                Ok(_) => {}
                Err(_) => {
                    self.change_view(Views::Error);
                    return;
                }
            };
        }

        match self.list_view.update_password_list() {
            Ok(_) => {}
            Err(_) => {
                self.change_view(Views::Error);
                return;
            }
        };

        self.change_view(Views::List);
    }
}

fn find_password(key: &[u8], name: &str) -> Option<Password> {
    let eventlog = match load_eventlog(key) {
        Ok(value) => value,
        Err(_) => return None,
    };

    let initial_state = PasswordsState::new();
    let state = eventlog.project(initial_state);

    let password = state
        .passwords
        .iter()
        .find(|password| password.name == name)?;

    Some(password.clone())
}

fn read_password(key: &[u8], name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let password = match find_password(key, name) {
        Some(value) => value,
        None => return Err(Box::from("Failed to find password")),
    };

    Ok(password.password)
}