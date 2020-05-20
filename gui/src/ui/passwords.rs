use gtk::{Builder, ListBox, ListBoxRow, ListBoxRowBuilder, BoxBuilder, LabelBuilder, ImageBuilder, ButtonBuilder, Align};

use gtk::prelude::*;

use common::key::{load_key};
use crypto::{decrypt, generate_iv_from_seed};

pub fn build<CallbackFn: Fn(&Builder) + Copy + 'static>(main_builder: &Builder, on_remove: CallbackFn) -> Builder {
    let source = include_str!("passwords.glade");
    let builder = Builder::new_from_string(source);

    let passwords = match common::password::list_passwords() {
        Ok(passwords) => passwords,
        Err(_) => return builder,
    };

    let list = match builder.get_object::<ListBox>("main") {
        Some(list) => list,
        None => return builder,
    };

    let builder_clone = main_builder.clone();
    for password in passwords {
        let row = build_listbox_row(password, &builder_clone, move |builder| on_remove(builder));
        list.add(&row);
    }

    list.show_all();

    builder
}

fn build_listbox_row<CallbackFn: Fn(&Builder) + Copy + 'static>(name: String, builder: &Builder, on_remove: CallbackFn) -> ListBoxRow {
    let label = LabelBuilder::new()
        .label(&name)
        .halign(Align::Start)
        .hexpand(true)
        .build();

    let password_image = ImageBuilder::new()
        .icon_name("dialog-password-symbolic")
        .build();

    let password_button = ButtonBuilder::new()
        .child(&password_image)
        .margin_start(5)
        .build();

    let name_clone = name.clone();
    password_button.connect_clicked(move |_| {
        load_password(&name_clone);
    });

    let remove_image = ImageBuilder::new()
        .icon_name("edit-delete-symbolic")
        .build();

    let remove_button = ButtonBuilder::new()
        .child(&remove_image)
        .margin_start(5)
        .build();

    let name_clone = name.clone();
    let builder_clone = builder.clone();
    remove_button.connect_clicked(move |_| {
        remove_password(&name_clone);
        on_remove(&builder_clone);
    });

    let b = BoxBuilder::new()
        .margin(5)
        .build();

    b.add(&label);
    b.add(&password_button);
    b.add(&remove_button);

    let row = ListBoxRowBuilder::new()
        .child(&b)
        .selectable(false)
        .build();

    row
}

fn load_password(name: &str) {
    let password = match common::password::load_password(name) {
        Ok(password) => password,
        Err(_) => return,
    };

    let decrypted = match decrypt_password(&name, &password) {
        Ok(decrypted) => decrypted,
        Err(_) => return,
    };

    common::clip_board::copy_value_to_clipboard(decrypted).unwrap();
}

fn remove_password(name: &str) {
    match common::password::remove_password(name) {
        Ok(_) => {},
        Err(_) => println!("failed to remove password {}", name),
    };
}

fn decrypt_password(name: &str, password: &[u8]) -> Result<String, String> {
    let key = load_key()?;
    let iv = generate_iv_from_seed(name)?;
    let decrypted = decrypt(password, &key, &iv)?;

    let value = match std::str::from_utf8(&decrypted) {
        Ok(value) => value,
        Err(_) => return Err(String::from("Failed to convert password to text")),
    };

    Ok(value.to_owned())
}