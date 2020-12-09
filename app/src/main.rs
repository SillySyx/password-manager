#![windows_subsystem = "windows"]

mod components;
mod translations;
mod clipboard;
mod datastore;
mod styles;
mod messages;
mod views;
mod passphrase;
mod states;
mod events;

use iced::{Sandbox, Settings};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    components::App::run(Settings::default())?;

    Ok(())
}