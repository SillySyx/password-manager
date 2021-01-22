#![windows_subsystem = "windows"]

mod components;
mod translations;
mod styles;
mod messages;
mod views;

use iced::{Sandbox, Settings};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    components::App::run(Settings::default())?;

    Ok(())
}