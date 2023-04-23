#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_parens)]




use iced::window::icon;
use iced::{Length, Element};
use iced::{Command};

use iced::widget::{Text, Container, TextInput, Column, Button, column, row};

use crate::app::{self};

use crate::icons;

use iced::alignment;




#[derive(Clone, Debug)]
pub struct Onglets {

    value: String

}


#[derive(Clone, Debug)]
pub enum Message {
    Modif(String),
    Close
}




impl Onglets {


    pub fn new () -> Onglets {

        Onglets { 
            value: "".to_string() 
        }

    }


    pub fn update(&mut self, message: Message) -> iced::Command<app::Message> {
        
        match message {
            Message::Modif(value) => { self.value = value; },
            Message::Close => todo!(),
        }

        Command::none()
    }

   

    pub fn view(&self, notes: &app::Notes) -> Element<app::Message> {


        
        
       
        
        let test = 
            TextInput::new("placeholder", &self.value)
            .width(Length::Shrink)
            .size(15)
            .on_input( |value| app::Message::Onglets(Message::Modif(value)));

     
        let i = Button::new(icons::file_icon());

        let c = row![test, i];

        let content = Container::new(c)
            .style(iced::theme::Container::Box)
            .height(Length::Fill)
            .width(Length::Shrink);

        Container::new(content)
            .padding(10)
            .height(Length::Fill)
            .width(Length::Shrink)
            .into()
        
    }
}


