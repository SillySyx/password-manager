mod components;
mod translations;
mod clipboard;
mod datastore;
mod styles;
mod messages;
mod views;
mod passphrase;

use iced::{Sandbox, Settings};

pub fn main() {
    components::App::run(Settings::default())
}