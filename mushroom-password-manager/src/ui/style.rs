use iced::{widget::{button, container}, Background, Color};

#[allow(dead_code)]
pub enum ButtonType {
    Primary,
    Secondary,
    Danger,
}

impl button::StyleSheet for ButtonType {
    type Style = ();
    fn active(&self, _style: &Self::Style) -> button::Appearance {
        match self {
            ButtonType::Primary => button::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.2, 0.5, 0.8))),
                border_radius: 4.0,
                text_color: Color::WHITE,
                ..button::Appearance::default()
            },
            ButtonType::Secondary => button::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.3, 0.3, 0.3))),
                border_radius: 4.0,
                text_color: Color::WHITE,
                ..button::Appearance::default()
            },
            ButtonType::Danger => button::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.8, 0.2, 0.2))),
                border_radius: 4.0,
                text_color: Color::WHITE,
                ..button::Appearance::default()
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);

        button::Appearance {
            background: match self {
                ButtonType::Primary => Some(Background::Color(Color::from_rgb(0.3, 0.6, 0.9))),
                ButtonType::Secondary => Some(Background::Color(Color::from_rgb(0.4, 0.4, 0.4))),
                ButtonType::Danger => Some(Background::Color(Color::from_rgb(0.9, 0.3, 0.3))),
            },
            ..active
        }
    }
}

pub struct ContainerStyle;

impl container::StyleSheet for ContainerStyle {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(Color::from_rgb(0.95, 0.95, 0.95))),
            text_color: Some(Color::from_rgb(0.1, 0.1, 0.1)),
            border_radius: 5.0,
            border_width: 1.0,
            border_color: Color::from_rgb(0.8, 0.8, 0.8),
            ..container::Appearance::default()
        }
    }
}