#![allow(dead_code)]
#![allow(unused_variables)]


use iced::Length;
use iced::{Command};

use crate::theme::widget::{Element, Text, Container};

use crate::app::{self};

use crate::theme:: {self};



use iced::alignment;




#[derive(Clone, Debug)]
pub struct Onglets {


}


#[derive(Clone, Debug)]
pub enum Message {
    Close
}




impl Onglets {


    pub fn new () -> Onglets {

        Onglets {  }

    }


    pub fn update(&mut self, _message: Message) -> iced::Command<app::Message> {

        let ret = Command::none();
        {}
        ret
    }

   

    pub fn view(&self, notes: &app::Notes) -> Element<app::Message, iced::Renderer<theme::Theme>> {


        let onglets = Text::new(format!("{}", notes.test))
            .height(Length::Fill)
            .width(Length::Fill)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center);
       


        let content = Container::new(onglets)
            .style(theme::Container::Bordered)
            .height(Length::Fill)
            .width(Length::Fill);

        Container::new(content)
            .padding(10)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
        
    }
}