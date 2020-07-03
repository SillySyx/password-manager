mod components;
mod translations;
mod config;

use iced::{Sandbox, Settings};

use components::App;

pub fn main() {
    App::run(Settings::default())
}