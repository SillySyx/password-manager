mod ui;

use gio::prelude::*;
use gtk::prelude::*;

use gio::{ApplicationFlags};
use gtk::{ApplicationBuilder};

fn main() {
    common::app::ensure_app_folders_exist().expect("failed to ensure app folders exist");
    
    gtk::init().expect("failed to init");

    let app = create_application(|app| {
        let builder = ui::main::build();
        if let Some(window) = builder.get_object::<gtk::Window>("main") {
            window.set_application(Some(app));
            window.show_all();
        }
    });

    app.run(&[]);
}

fn create_application<BuildFunction: Fn(&gtk::Application) + 'static>(build: BuildFunction) -> gtk::Application {
    let app = ApplicationBuilder::new()
        .application_id("com.passwordmanager")
        .flags(ApplicationFlags::FLAGS_NONE)
        .build();

    app.connect_activate(build);

    app
}