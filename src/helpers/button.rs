#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use iced::{
    alignment,
    widget::{button, text},
    Color, Element, Length,
};

pub fn base_button<'a, T>(
    content: impl Into<Element<'a, T, iced::Renderer>>,
    msg: T,
) -> button::Button<'a, T, iced::Renderer> {
    button(content)
        .padding([4, 8])
        .style(iced::theme::Button::Custom(Box::new(ButtonStyle {})))
        .on_press(msg)
}

pub fn labeled_button<'a, T>(label: &str, msg: T) -> button::Button<'a, T, iced::Renderer> {
    base_button(
        text(label)
            .width(Length::Fill)
            .height(Length::Fill)
            .vertical_alignment(alignment::Vertical::Center),
        msg,
    )
}

struct ButtonStyle;

impl button::StyleSheet for ButtonStyle {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            text_color: style.extended_palette().background.base.text,
            border_radius: 4.0,
            background: Some(Color::TRANSPARENT.into()),
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let plt = style.extended_palette();

        button::Appearance {
            background: Some(plt.primary.weak.color.into()),
            text_color: plt.primary.weak.text,
            ..self.active(style)
        }
    }
}
