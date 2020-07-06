mod components;
mod translations;

use iced::{Sandbox, Settings};

use components::App;

pub fn main() {
    App::run(Settings::default())
}