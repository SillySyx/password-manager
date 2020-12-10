mod addpassword;
mod app;
mod edit;
mod error;
mod list;
mod password;
mod category;
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
    category::Category,
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
            let svg = create_svg(icon)
                .width(Length::from(16))
                .height(Length::from(16));

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

fn load_icon_bytes(name: &str) -> Option<Vec<u8>> {
    match name {
        "add.svg" => Some(include_bytes!("../../resources/icons/add.svg").to_vec()),
        "back.svg" => Some(include_bytes!("../../resources/icons/back.svg").to_vec()),
        "cog.svg" => Some(include_bytes!("../../resources/icons/cog.svg").to_vec()),
        "copy.svg" => Some(include_bytes!("../../resources/icons/copy.svg").to_vec()),
        "generate.svg" => Some(include_bytes!("../../resources/icons/generate.svg").to_vec()),
        "key.svg" => Some(include_bytes!("../../resources/icons/key.svg").to_vec()),
        "logo.svg" => Some(include_bytes!("../../resources/icons/logo.svg").to_vec()),
        "trash.svg" => Some(include_bytes!("../../resources/icons/trash.svg").to_vec()),
        "unlock.svg" => Some(include_bytes!("../../resources/icons/unlock.svg").to_vec()),
        _ => None,
    }
}

fn create_svg(icon: &str) -> Svg {
    let icon_bytes = load_icon_bytes(icon).expect("failed to load aaaah");

    Svg::new(iced::svg::Handle::from_memory(icon_bytes))
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
            let svg = create_svg(icon)
                .width(Length::from(16))
                .height(Length::from(16));
            container.push(svg)
        },
        None => container,
    };

    let container = match text {
        Some(text) => {
            let text = Text::new(text).size(18);
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

    let logo = create_svg("logo.svg")
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