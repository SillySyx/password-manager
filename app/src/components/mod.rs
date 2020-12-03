mod addpassword;
mod app;
mod edit;
mod error;
mod list;
mod password;
mod unlock;

use iced::{Column, Length, Row, Element, Container, Align, Text, Svg, Button, Space};

pub use {
    app::App,
    unlock::Unlock,
    list::List,
    password::Password,
    addpassword::AddPassword,
    error::Error,
    edit::EditPassword,
};

use super::styles::{HeaderStyle, WidgetStyle, MainStyle, LinkButtonStyle};

pub fn create_button<'a, T: Clone + 'static>(
    state: &'a mut iced::button::State,
    text: Option<&str>,
    icon: Option<&str>,
    message: T,
) -> Button<'a, T> {
    let container = Row::new()
        .align_items(Align::Center)
        .spacing(5);

    let container = match icon {
        Some(icon) => {
            let svg = Svg::from_path(format!("{}/resources/icons/{}", env!("CARGO_MANIFEST_DIR"), icon))
                .width(Length::from(14))
                .height(Length::from(14));
            container.push(svg)
        },
        None => container,
    };

    let container = match text {
        Some(text) => {
            let text = Text::new(text).size(16);
            container.push(text)
        },
        None => container,
    };

    Button::new(state, container)
        .on_press(message)
        .padding(10)
        .style(MainStyle)
}


pub fn create_link_button<'a, T: Clone + 'static>(
    state: &'a mut iced::button::State,
    text: Option<&str>,
    icon: Option<&str>,
    message: T,
) -> Button<'a, T> {
    let container = Row::new()
        .align_items(Align::Center)
        .spacing(5);

    let container = match icon {
        Some(icon) => {
            let svg = Svg::from_path(format!("{}/resources/icons/{}", env!("CARGO_MANIFEST_DIR"), icon))
                .width(Length::from(14))
                .height(Length::from(14));
            container.push(svg)
        },
        None => container,
    };

    let container = match text {
        Some(text) => {
            let text = Text::new(text).size(16);
            container.push(text)
        },
        None => container,
    };

    Button::new(state, container)
        .on_press(message)
        .style(LinkButtonStyle)
}

pub fn create_widget<'a, T: 'static, C>(content: C) -> Container<'a, T>
where
    C: Into<Element<'a, T>>,
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
            None => Space::with_width(Length::Fill).into(),
        };

        Container::new(content)
            .width(Length::Fill)
    };

    let header_right = {
        let content = match header_right {
            Some(element) => element,
            None => Space::with_width(Length::Fill).into(),
        };

        Container::new(content)
            .width(Length::Fill)
            .align_x(Align::End)
    };

    let logo = Svg::from_path(format!("{}/resources/icons/logo.svg", env!("CARGO_MANIFEST_DIR")))
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