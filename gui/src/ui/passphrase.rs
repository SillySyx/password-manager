use gtk::{Builder, Button, Label};

use gtk::prelude::*;

pub fn build() -> Builder {
    let source = include_str!("passphrase.glade");
    let builder = Builder::new_from_string(source);

    setup_generate_button(&builder);

    builder
}

fn setup_generate_button(builder: &Builder) {
    let button = match builder.get_object::<Button>("generate_button") {
        Some(button) => button,
        None => return,
    };

    let label = match builder.get_object::<Label>("passphrase_label") {
        Some(label) => label,
        None => return,
    };

    button.connect_clicked(move |_| {
        match common::passphrase::generate_passphrase() {
            Ok(passphrase) => label.set_label(&passphrase),
            Err(error) => label.set_label(&error),
        };
    });
}