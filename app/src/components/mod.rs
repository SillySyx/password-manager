mod addpassword;
mod app;
mod edit;
mod error;
mod list;
mod password;
mod unlock;

pub use {
    app::App,
    unlock::Unlock,
    list::List,
    password::Password,
    addpassword::AddPassword,
    error::Error,
    edit::EditPassword,
};

pub fn create_button<'a, T: Clone + 'static>(
    state: &'a mut iced::button::State,
    text: &str,
    icon: &str,
    message: T,
) -> iced::Button<'a, T> {
    let text = iced::Text::new(text).size(16);

    let svg = iced::Svg::from_path(format!("{}/resources/icons/{}", env!("CARGO_MANIFEST_DIR"), icon))
        .width(iced::Length::from(12))
        .height(iced::Length::from(12));

    let container = iced::Row::new()
        .spacing(5)
        .push(svg)
        .push(text);

    let button = iced::Button::new(state, container)
        .on_press(message)
        .padding(10)
        .style(super::styles::MainStyle);

    button
}

pub fn create_widget<'a, T, C>(content: C) -> iced::Container<'a, T>
where
    C: Into<iced::Element<'a, T>>,
{
    iced::Container::new(content)
        .padding(20)
        .style(super::styles::WidgetStyle)
}
