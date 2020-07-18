use iced::{Color, Background, container};

const TEXTCOLOR: Color = Color::from_rgb(1.0, 1.0, 1.0);
const LAYOUTBACKGROUND: Color = Color::from_rgb(0.1, 0.1, 0.1);

pub struct HeaderStyle;

impl container::StyleSheet for HeaderStyle {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(LAYOUTBACKGROUND)),
            text_color: Some(TEXTCOLOR),
            ..container::Style::default()
        }
    }
}