use gtk::{Builder, Button, ListBox, Viewport, Widget};

use gtk::prelude::*;

pub fn build() -> Builder {
    let source = include_str!("main.glade");
    let builder = Builder::new_from_string(source);

    let builder_clone = builder.clone();
    setup_new_password_button(&builder, move |event_name| handle_events(&builder_clone, event_name));
    
    let builder_clone = builder.clone();
    setup_menu(&builder, move |event_name| handle_events(&builder_clone, event_name));

    builder
}

fn handle_events(builder: &Builder, event_name: &'static str) {
    match event_name {
        "new_password" => show_new_password_dialog(&builder),
        "passwords" => show_passwords_ui(&builder),
        "passphrase" => show_passphrase_ui(&builder),
        "settings" => show_settings_ui(&builder),
        _ => {}
    };
}

fn setup_new_password_button<F: Fn(&'static str) + 'static>(builder: &Builder, event_handler: F) {
    let button = match builder.get_object::<Button>("new_password_button") {
        Some(button) => button,
        None => return,
    };

    button.connect_clicked(move |_| {
        event_handler("new_password");
    });
}

fn setup_menu<F: Fn(&'static str) + 'static>(builder: &Builder, event_handler: F) {
    let menu = match builder.get_object::<ListBox>("menu") {
        Some(menu) => menu,
        None => return,
    };

    menu.connect_row_selected(move |_, listbox_row| {
        let row = match listbox_row {
            Some(row) => row,
            None => return,
        };

        let index = row.get_index();
        if index == 0 {
            event_handler("passwords");
        }
        if index == 1 {
            event_handler("passphrase");
        }
        if index == 2 {
            event_handler("settings");
        }
    });
}

fn change_viewport_content(builder: &Builder, content: &Builder) {
    let container = match builder.get_object::<Viewport>("content") {
        Some(container) => container,
        None => return,
    };

    remove_children(&container);

    let widget = match content.get_object::<Widget>("main") {
        Some(widget) => widget,
        None => return,
    };

    container.add(&widget);
}

fn remove_children<T: IsA<gtk::Container>>(container: &T) {
    for child in container.get_children() {
        container.remove(&child);
    }
}

fn show_new_password_dialog(builder: &Builder) {
    let window = match builder.get_object::<Widget>("main") {
        Some(window) => window,
        None => return,
    };

    let builder_clone = builder.clone();
    super::new_password::build(&window, move |event_name| handle_events(&builder_clone, event_name));
}

fn show_passwords_ui(builder: &Builder) {
    let passwords_ui = super::passwords::build(builder, move |builder| {
        handle_events(builder, "passwords");
    });
    change_viewport_content(&builder, &passwords_ui);
}

fn show_passphrase_ui(builder: &Builder) {
    let passphrase_ui = super::passphrase::build();
    change_viewport_content(&builder, &passphrase_ui);
}

fn show_settings_ui(builder: &Builder) {
    let settings_ui = super::settings::build();
    change_viewport_content(&builder, &settings_ui);
}