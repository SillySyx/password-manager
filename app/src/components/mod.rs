mod app;
mod unlock;
mod list;
mod password;
mod addpassword;
mod error;

pub use app::App;

pub fn create_button<'a, T>(state: &'a mut iced::button::State, text: &str, message: T) -> iced::Button<'a, T> {
    let text = iced::Text::new(text)
        .size(16);

    iced::Button::new(state, text)
        .on_press(message)
        .padding(10)
        .style(super::styles::MainStyle)
}

pub fn create_widget<'a, T, C>(content: C) -> iced::Container<'a, T> where C : Into<iced::Element<'a, T>> {
    iced::Container::new(content)
        .padding(20)
        .style(super::styles::WidgetStyle)
}