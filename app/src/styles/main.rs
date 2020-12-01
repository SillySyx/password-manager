use iced::{Color, Background, container, button, text_input};

const TEXTCOLOR: Color = Color::from_rgb(1.0, 1.0, 1.0);
const ACCENTCOLOR: Color = Color::from_rgb(0.5, 0.2, 0.6);
const BUTTONBACKGROUND: Color = Color::from_rgb(0.6, 0.6, 0.6);

pub struct MainStyle;

impl container::StyleSheet for MainStyle {
    fn style(&self) -> container::Style {
        container::Style {
            ..container::Style::default()
        }
    }
}

impl button::StyleSheet for MainStyle {
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
            background: Some(Background::Color(ACCENTCOLOR)),
            ..self.active()
        }
    }

    fn pressed(&self) -> button::Style {
        button::Style {
            ..self.active()
        }
    }
}

impl text_input::StyleSheet for MainStyle {
    fn active(&self) -> text_input::Style {
        text_input::Style {
            ..text_input::Style::default()
        }
    }

    fn focused(&self) -> text_input::Style {
        text_input::Style {
            ..self.active()
        }
    }

    fn hovered(&self) -> text_input::Style {
        text_input::Style {
            ..self.focused()
        }
    }

    fn placeholder_color(&self) -> Color {
        TEXTCOLOR
    }

    fn value_color(&self) -> Color {
        TEXTCOLOR
    }

    fn selection_color(&self) -> Color {
        TEXTCOLOR
    }
}