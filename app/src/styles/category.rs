use iced::{Color, Background, button};

const TEXTCOLOR: Color = Color::from_rgb(0.0, 0.0, 0.0);
const BUTTONBACKGROUND: Color = Color::from_rgb(0.95, 0.95, 0.95);

pub struct CategoryStyle;

impl button::StyleSheet for CategoryStyle {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(BUTTONBACKGROUND)),
            text_color: TEXTCOLOR,
            border_radius: 5.0,
            border_width: 0.0,
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            ..self.active()
        }
    }

    fn pressed(&self) -> button::Style {
        button::Style {
            ..self.active()
        }
    }
}