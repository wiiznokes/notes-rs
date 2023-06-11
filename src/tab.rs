#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use iced::{Alignment, Command};
use iced::{Element, Length};
use iced::alignment;
use iced::widget::{Button, column, Column, Container, row, Text, TextInput};
use iced::window::icon;

use crate::app::{self};
use crate::icons;

#[derive(Clone, Debug)]
pub struct Tab {}

#[derive(Clone, Debug)]
pub enum Message {
    Close,
}

impl Tab {
    pub fn new() -> Tab {
        Tab {}
    }

    pub fn update(&mut self, message: Message) -> Command<app::Message> {
        match message {
            Message::Close => todo!(),
        }
    }

    pub fn view(&self, notes: &app::Notes) -> Element<app::Message> {
        let text = Text::new("hello")
            .width(Length::Fill)
            .height(Length::Fill)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center);

        let content = Container::new(text)
            .style(iced::theme::Container::Box)
            .height(Length::Fill)
            .width(Length::Fill);

        Container::new(content)
            .padding(10)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }
}
