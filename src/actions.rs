#![allow(dead_code)]
#![allow(unused_variables)]


use iced::{Command};

use iced::Length;


use crate::theme::widget::{Element, Button, Row, Text};

use crate::app::{self};

use crate::theme:: {self};

use iced::widget::{Space};

#[derive(Clone, Debug, Copy)]
pub struct Actions {

    tt: i32,
}


#[derive(Clone, Debug)]
pub enum Message {
    Toggle,
    Settings,
    Push,
    Fetch,
    Edit
}




impl Actions {


    pub fn new () -> Actions {

        Actions { tt: 0 }

    }

    pub fn update(&mut self, message: Message, notes: &mut app::Notes) -> iced::Command<app::Message> {

        match message {
            Message::Toggle => {

                println!("hella!");

                println!("{}", self.tt);
                self.tt += 1;


                notes.test += 1;

                Command::none()
            },
            _ => Command::none()
        }
    }



    pub fn view(&self) -> Element<app::Message, iced::Renderer<theme::Theme>> {


        let left_actions = Row::new()
            .push(
                Button::new(Text::new("Toggle"))
                    .on_press(app::Message::Actions(Message::Toggle)))
            .push(Button::new("Settings"))
            .spacing(10)
            .width(Length::Fill);


        Row::new()
            .push(Space::new(5, 0))
            .push(left_actions)
            .push(
                Row::new()
                    .push(Button::new(Text::new("Toggle")))
                    .push(Button::new("Settings"))
                    .spacing(10)
                    .width(Length::Fill),
            )
            .push(
                Row::new()
                    .push(Button::new("Push"))
                    .push(Button::new("Fetch"))
                    .push(Button::new("Edit"))
                    .spacing(10)
                    .width(Length::Shrink),
            )
            .push(Space::new(5, 0))
            .into()
  

        

        
       
        
    }
}