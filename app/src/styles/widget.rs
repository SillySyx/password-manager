use iced::{Color, Background, container};

const WIDGETBACKGROUND: Color = Color::from_rgb(0.975, 0.975, 0.975);

pub struct WidgetStyle;

impl container::StyleSheet for WidgetStyle {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(WIDGETBACKGROUND)),
            border_radius: 10,
            ..container::Style::default()
        }
    }
}