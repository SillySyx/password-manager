use gtk::{Builder, Window, Widget, Button, Entry};

use gtk::prelude::*;

use common::key::{load_key};
use crypto::{encrypt, generate_iv_from_seed};

pub fn build<F: Fn(&'static str) + 'static>(parent_window: &Widget, event_handler: F) -> Builder {
    let source = include_str!("new_password.glade");
    let builder = Builder::new_from_string(source);

    if let Some(window) = builder.get_object::<Window>("main") {
        setup_close_button(&builder, &window);
        setup_add_button(&builder, &window, move || event_handler("passwords"));

        window.set_attached_to(Some(parent_window));
        window.show_all();
    }

    builder
}

fn setup_close_button(builder: &Builder, window: &Window) {
    let button = match builder.get_object::<Button>("close_button") {
        Some(button) => button,
        None => return,
    };

    let window_clone = window.clone();
    button.connect_clicked(move |_| {
        window_clone.close();
    });
}

fn setup_add_button<F: Fn() + 'static>(builder: &Builder, window: &Window, f: F) {
    let button = match builder.get_object::<Button>("add_button") {
        Some(button) => button,
        None => return,
    };

    let window_clone = window.clone();
    let builder_clone = builder.clone();
    button.connect_clicked(move |_| {
        let name = match builder_clone.get_object::<Entry>("name_entry") {
            Some(name) => name,
            None => return,
        };
        
        let pass = match builder_clone.get_object::<Entry>("password_entry") {
            Some(pass) => pass,
            None => return,
        };

        let name_text = name.get_text().unwrap().to_string();
        let pass_text = pass.get_text().unwrap().to_string();

        let encrypted = encrypt_password(&name_text, &pass_text).unwrap();
        common::password::save_password(&name_text, &encrypted).unwrap();

        window_clone.close();
        
        f();
    });
}

fn encrypt_password(name: &str, password: &str) -> Result<Vec<u8>, String> {
    let key = load_key()?;
    let iv = generate_iv_from_seed(name)?;
    let bytes = password.as_bytes();
    
    encrypt(&bytes, &key, &iv)
}