#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_parens)]

use iced::window::icon;
use iced::{Alignment, Command};
use iced::{Element, Length};

use iced::widget::{column, row, Button, Column, Container, Text, TextInput};

use crate::app::{self};

use crate::icons;

use iced::alignment;

#[derive(Clone, Debug)]
pub struct Onglets {}

#[derive(Clone, Debug)]
pub enum Message {
    Close,
}

impl Onglets {
    pub fn new() -> Onglets {
        Onglets {}
    }

    pub fn update(&mut self, message: Message) -> iced::Command<app::Message> {
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
