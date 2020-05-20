use gtk::{Builder, Button, Entry};

use gtk::prelude::*;

pub fn build() -> Builder {
    let source = include_str!("settings.glade");
    let builder = Builder::new_from_string(source);

    setup_save_button(&builder);

    builder
}

fn setup_save_button(builder: &Builder) {
    let button = match builder.get_object::<Button>("save_button") {
        Some(button) => button,
        None => return,
    };

    let entry = match builder.get_object::<Entry>("key_entry") {
        Some(entry) => entry,
        None => return,
    };

    button.connect_clicked(move |_| {
        let text = entry.get_text().unwrap().to_string();

        if let Ok(key) = crypto::generate_key_from_seed(&text) {
            common::key::save_key(&key).expect("failed to save key");
            entry.set_text("");
        }
    });
}