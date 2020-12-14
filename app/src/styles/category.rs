use iced::{Color, Background, button, container};

const TEXTCOLOR: Color = Color::from_rgb(0.0, 0.0, 0.0);
const BACKGROUNDCOLOR: Color = Color::from_rgb(0.95, 0.95, 0.95);
const ACCENTCOLOR: Color = Color::from_rgb(0.9294, 0.5686, 0.1294);

pub struct CategoryStyle;
pub struct ActiveCategoryStyle;

impl container::StyleSheet for CategoryStyle {
    fn style(&self) -> container::Style {
        container::Style {
            ..container::Style::default()
        }
    }
}

impl container::StyleSheet for ActiveCategoryStyle {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(ACCENTCOLOR)),
            ..container::Style::default()
        }
    }
}

impl button::StyleSheet for CategoryStyle {
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

impl button::StyleSheet for ActiveCategoryStyle {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(BACKGROUNDCOLOR)),
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