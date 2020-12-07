use iced::{Color, Background, container, button};

const TEXTCOLOR: Color = Color::from_rgb(0.0, 0.0, 0.0);
const BUTTONBACKGROUND: Color = Color::from_rgb(0.95, 0.95, 0.95);

pub struct PasswordStyle;

impl container::StyleSheet for PasswordStyle {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(BUTTONBACKGROUND)),
            text_color: Some(TEXTCOLOR),
            ..container::Style::default()
        }
    }
}

impl button::StyleSheet for PasswordStyle {
    fn active(&self) -> button::Style {
        button::Style {
            text_color: TEXTCOLOR,
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

    fn disabled(&self) -> button::Style {
        button::Style {
            ..self.active()
        }
    }
}