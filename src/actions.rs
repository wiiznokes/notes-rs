#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_parens)]

use iced::Command;

use iced::{Element, Length};

use iced::widget::{Button, Row, Text};

use crate::app;

use iced::widget::Space;

#[derive(Clone, Debug)]
pub struct Actions {
    tt: i32,
}

#[derive(Clone, Debug)]
pub enum Message {
    Toggle,
    Settings,
    Push,
    Fetch,
    Edit,
}

impl Actions {
    pub fn new() -> Actions {
        Actions { tt: 0 }
    }

    pub fn update(&mut self, message: Message) -> iced::Command<app::Message> {
        match message {
            Message::Toggle => todo!(),
            Message::Settings => todo!(),
            Message::Push => todo!(),
            Message::Fetch => todo!(),
            Message::Edit => todo!(),
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
