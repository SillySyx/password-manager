use iced::{Color, button};

const TEXTCOLOR: Color = Color::from_rgb(1.0, 1.0, 1.0);

pub struct LinkButtonStyle;

impl button::StyleSheet for LinkButtonStyle {
    fn active(&self) -> button::Style {
        button::Style {
            background: None,
            text_color: TEXTCOLOR,
            border_radius: 0.0,
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