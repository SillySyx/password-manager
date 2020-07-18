use iced::{Element, Sandbox};

use crate::{
    clipboard::copy_value_to_clipboard,
    components::{AddPassword, EditPassword, Error, List, Unlock},
    datastore::{load_datastore, save_datastore},
    messages::Messages,
    translations::{translate, Languages},
    views::Views,
};

use crypto::generate_key_from_seed;
use glob::Archive;

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
            Messages::AddPasswordMessage { name, password } => self.add_password(name, password),
            Messages::EditPassword { name } => self.edit_password(name),
            Messages::CopyPassword { name } => self.copy_password(name),
            Messages::RemovePassword { name } => self.remove_password(name),
            Messages::AddViewInputKeyChanged { input, value } => {
                self.add_view.update_input(input, value)
            }
            Messages::EditViewInputKeyChanged { input, value } => {
                self.edit_view.update_input(input, value)
            }
            Messages::UnlockViewInputKeyChanged { value } => self.unlock_view.input_key = value,
            Messages::UpdatePassword { entry, name, password } => self.update_password(entry, name, password),
            Messages::GeneratePassphraseForAddView => self.add_view.generate_passphrase(),
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

    fn add_password(&mut self, name: String, password: String) {
        if name.is_empty() {
            return;
        }

        if password.is_empty() {
            return;
        }

        let mut archive = match load_datastore(&self.key) {
            Ok(archive) => archive,
            Err(_) => {
                self.change_view(Views::Error);
                return;
            }
        };

        match archive.add_entry(&name, password.as_bytes()) {
            Ok(_) => {}
            Err(_) => {
                self.change_view(Views::Error);
                return;
            }
        }

        match save_datastore(&self.key, &mut archive) {
            Ok(_) => {}
            Err(_) => {
                self.change_view(Views::Error);
                return;
            }
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

    fn edit_password(&mut self, name: String) {
        self.edit_view.reset();
        self.edit_view.entry = name.clone();
        self.edit_view.name = name.clone();

        let password = match read_password(&self.key, &name) {
            Ok(value) => value,
            Err(_) => {
                self.change_view(Views::Error);
                return;
            }
        };

        self.edit_view.password = password;

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

    fn remove_password(&mut self, name: String) {
        let mut archive = match load_datastore(&self.key) {
            Ok(archive) => archive,
            Err(_) => {
                self.change_view(Views::Error);
                return;
            }
        };

        let entry = match archive.find_entry(&name) {
            Ok(entry) => entry,
            Err(_) => return,
        };

        match archive.remove_entry(&entry) {
            Ok(_) => {}
            Err(_) => return,
        };

        match save_datastore(&self.key, &mut archive) {
            Ok(_) => {}
            Err(_) => return,
        };

        match self.list_view.update_password_list() {
            Ok(_) => {}
            Err(_) => {}
        };

        self.change_view(Views::List);
    }

    fn update_password(&mut self, entry: String, name: String, password: String) {
        let mut archive = match load_datastore(&self.key) {
            Ok(archive) => archive,
            Err(_) => {
                self.change_view(Views::Error);
                return;
            }
        };

        let entry = match archive.find_entry(&entry) {
            Ok(entry) => entry,
            Err(_) => return,
        };

        match archive.replace_entry(&entry, &name, password.as_bytes()) {
            Ok(_) => {}
            Err(_) => return,
        };

        match save_datastore(&self.key, &mut archive) {
            Ok(_) => {}
            Err(_) => return,
        };

        match self.list_view.update_password_list() {
            Ok(_) => {}
            Err(_) => {}
        };

        self.change_view(Views::List);
    }
}

fn read_password(key: &[u8], name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut archive = load_datastore(key)?;
    let entry = archive.find_entry(name)?;

    let data = archive.read_entry_data(&entry)?;

    Ok(String::from_utf8(data)?)
}