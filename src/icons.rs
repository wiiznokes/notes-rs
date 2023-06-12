#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use iced::alignment;
use iced::Font::{self};
use iced::widget::{Text, text};

// Fonts
const ICONS: Font = Font::External {
    name: "Icons",
    bytes: include_bytes!("../fonts/icons.ttf"),
};

fn icon(unicode: char) -> Text<'static> {
    text(unicode.to_string())
        .font(ICONS)
        .width(15)
        .horizontal_alignment(alignment::Horizontal::Center)
        .size(15)
}


pub fn file_icon() -> Text<'static> {
    icon('\u{e802}')
}

pub fn folder_icon() -> Text<'static> {
    icon('\u{e803}')
}

pub fn chevron_right_icon() -> Text<'static> {
    icon('\u{e801}')
}

pub fn chevron_down_icon() -> Text<'static> {
    icon('\u{e800}')
}