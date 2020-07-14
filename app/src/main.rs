mod components;
mod translations;
mod clipboard;
mod datastore;
mod styles;

use iced::{Sandbox, Settings};

use components::App;

pub fn main() {
    App::run(Settings::default())
}