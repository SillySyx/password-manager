mod addpassword;
mod app;
mod edit;
mod error;
mod list;
mod password;
mod unlock;

use iced::{Column, Length, Row, Element, Container, Align};

pub use {
    app::App,
    unlock::Unlock,
    list::List,
    password::Password,
    addpassword::AddPassword,
    error::Error,
    edit::EditPassword,
};

use super::styles::{HeaderStyle, WidgetStyle, MainStyle};

pub fn create_button<'a, T: Clone + 'static>(
    state: &'a mut iced::button::State,
    text: &str,
    icon: &str,
    message: T,
) -> iced::Button<'a, T> {
    let text = iced::Text::new(text).size(16);

    let svg = iced::Svg::from_path(format!("{}/resources/icons/{}", env!("CARGO_MANIFEST_DIR"), icon))
        .width(iced::Length::from(14))
        .height(iced::Length::from(14));

    let container = iced::Row::new()
        .align_items(Align::Center)
        .spacing(5)
        .push(svg)
        .push(text);

    let button = iced::Button::new(state, container)
        .on_press(message)
        .padding(10)
        .style(MainStyle);

    button
}

pub fn create_widget<'a, T: 'static, C>(content: C) -> iced::Container<'a, T>
where
    C: Into<iced::Element<'a, T>>,
{
    let content = Container::new(content)
        .padding(20)
        .style(WidgetStyle);

    Container::new(content)
        .max_width(500)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
}

pub fn create_layout<'a, T: Clone + 'static>(
    header_left: Option<Element<'a, T>>, 
    header_right: Option<Element<'a, T>>, 
    content: Element<'a, T>
) -> Column<'a, T> {

    let header_left = {
        let content = match header_left {
            Some(element) => element,
            None => iced::Text::new("").into(),
        };

        Container::new(content)
            .width(Length::Fill)
    };

    let header_right = {
        let content = match header_right {
            Some(element) => element,
            None => iced::Text::new("").into(),
        };

        Container::new(content)
            .width(Length::Fill)
            .align_x(Align::End)
    };

    let logo = iced::Svg::from_path(format!("{}/resources/icons/logo.svg", env!("CARGO_MANIFEST_DIR")))
        .width(Length::from(30))
        .height(Length::from(30));

    let header_row = Row::new()
        .align_items(Align::Center)
        .width(Length::Fill)
        .push(header_left)
        .push(logo)
        .push(header_right);

    let header = Container::new(header_row)
        .width(Length::Fill)
        .padding(10)
        .style(HeaderStyle);

    Column::new()
        .align_items(Align::Center)
        .width(Length::Fill)
        .push(header)
        .push(content)
}