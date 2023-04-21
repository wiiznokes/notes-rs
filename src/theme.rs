

use iced::{application, color, Color};

use iced::widget::{button, text, container}; 




#[derive(Debug, Clone, Copy, Default)]
pub struct Theme;


#[derive(Debug, Clone, Copy, Default)]
struct Colors {
    red: Color,
    dark_grey: Color,
    grey: Color,
    blue_green: Color,
    green: Color,
    light_grey: Color
}

impl Colors {


    fn new() -> Self {
        Self {
            red: color!(0xFF, 0x00, 0x00),
            dark_grey: color!(0x1A, 0x1A, 0x1A),
            grey: color!(0x80, 0x80, 0x80),
            blue_green: color![0x45, 0x85, 0x88],
            green: color![0x45, 0x85, 0x88],
            light_grey: color!(0x28, 0x28, 0x28),
        }
    }
}



impl application::StyleSheet for Theme {

    type Style = ();

    fn appearance(&self, style: &Self::Style) -> application::Appearance {
        let colors = Colors::new();
        application::Appearance {
            background_color: colors.dark_grey,
            text_color: colors.red,
        }
    }
}




impl button::StyleSheet for Theme {
    type Style = ();

    

    fn active(&self, style: &Self::Style) -> button::Appearance {
        let colors = Colors::new();
        button::Appearance {
            background: colors.light_grey.into(),
            border_radius: 4.0,
            border_width: 1.0,
            border_color: colors.green,
            ..Default::default()
        }
    }
}


impl text::StyleSheet for Theme {
    type Style = ();

    
    fn appearance(&self, style: Self::Style) -> text::Appearance {
        let colors = Colors::new();
        text::Appearance {
            color: colors.green.into()
            
        }
    }
}



#[derive(Debug, Clone, Copy, Default)]
pub enum Container {
    #[default]
    Default,
    Bordered,
}


impl container::StyleSheet for Theme {
    type Style = Container;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        let colors = Colors::new();
        match style {
            Container::Default => container::Appearance::default(),
            Container::Bordered => container::Appearance {
                border_color: colors.green,
                border_width: 1f32,
                border_radius: 10f32,
                ..Default::default()
            },
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
    pub type Element<'a, Message, Renderer> = iced::Element<'a, Message, Renderer>;
    pub type Container<'a, Message, Renderer> = iced::widget::Container<'a, Message, Renderer>;
    pub type Button<'a, Message, Renderer> = iced::widget::Button<'a, Message, Renderer>;
    pub type Column<'a, Message, Renderer> = iced::widget::Column<'a, Message, Renderer>;
    pub type Row<'a, Message, Renderer> = iced::widget::Row<'a, Message, Renderer>;
    pub type Text<'a, Message> = iced::widget::Text<'a, Message>;

}
