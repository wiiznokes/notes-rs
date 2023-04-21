
use iced::{application, color};

use iced::widget::{button}; 




#[derive(Debug, Clone, Copy, Default)]
pub struct Theme;





impl application::StyleSheet for Theme {

    type Style = ();

    fn appearance(&self, style: &Self::Style) -> application::Appearance {
        
        application::Appearance {
            background_color: color![0x45, 0x85, 0x88],
            text_color: color![0x45, 0x85, 0x88],
        }
    }
}




impl button::StyleSheet for Theme {
    type Style = ();

    

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: color!(0x28, 0x28, 0x28).into(),
            border_radius: 4.0,
            border_width: 1.0,
            border_color: color!(0x45, 0x85, 0x88),
            ..Default::default()
        }
    }
}

// Always import widget types from this module since it
// uses our custom theme instead of the built-in iced::Theme.
// Otherwise you will get compilation errors since iced::Element
// expects use of iced::Theme by default.
pub mod widget {
    #![allow(dead_code)]
    use crate::theme::Theme;

    pub type Renderer = iced::Renderer<Theme>;
    pub type Element<'a, Message> = iced::Element<'a, Message, Renderer>;
    pub type Container<'a, Message> = iced::widget::Container<'a, Message, Renderer>;
    pub type Button<'a, Message> = iced::widget::Button<'a, Message, Renderer>;
    pub type Column<'a, Message> = iced::widget::Column<'a, Message, Renderer>;
    pub type Row<'a, Message> = iced::widget::Row<'a, Message, Renderer>;

}
