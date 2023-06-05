#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_parens)]

use iced::Command;

use iced::{Element, Length};

use iced::widget::{Button, Row, Text};

use crate::app;

use iced::widget::Space;

use iced::widget::column as col;
use iced::widget::{
    button, checkbox, container, horizontal_space, pick_list, row, slider, svg, text, text_input,
    toggler, vertical_slider,
};
use iced::{alignment, theme, Application, Color};

use iced_aw::menu::{CloseCondition, ItemHeight, ItemWidth, MenuTree, PathHighlight};
use iced_aw::quad;
//use iced_aw::{helpers::menu_tree, menu_bar, menu_tree};


#[derive(Clone, Debug)]
pub struct Actions {
    tt: i32,
}

#[derive(Clone, Debug)]
pub enum Message {
    Toggle,
    Search,
    Push,
    Fetch,
}

impl Actions {
    pub fn new() -> Actions {
        Actions { tt: 0 }
    }

    pub fn update(&mut self, message: Message) -> iced::Command<app::Message> {
        match message {
            Message::Toggle => todo!(),
            Message::Push => todo!(),
            Message::Fetch => todo!(),

            _ => todo!()
        }
    }

    pub fn view(&self) -> Element<app::Message> {
        let left_actions = Row::new()
            .push(Button::new(Text::new("Toggle")).on_press(app::Message::Actions(Message::Toggle)))
            .push(Button::new("Settings"))
            .spacing(10)
            .width(Length::Fill);

        let right_actions = Row::new()
            .push(Button::new("Push"))
            .push(Button::new("Fetch"))
            .push(Button::new("Edit"))
            .spacing(10)
            .width(Length::Shrink);

        Row::new()
            .push(Space::new(5, 0))
            .push(left_actions)
            .push(right_actions)
            .push(Space::new(5, 0))
            .into()
    }
}



fn files_menu() {

}
