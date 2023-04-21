


use iced::Length;


use crate::theme::widget::{Element, Button, Row, Text};

use crate::app::{self};

use crate::theme:: {self};

use iced::widget::{Space};

#[derive(Clone, Debug, Copy)]
pub struct Actions {


}


#[derive(Clone, Debug)]
pub enum Message {
    Toggle(bool),
    Settings,
    Push,
    Fetch,
    Edit
}




impl Actions {


    pub fn new () -> Actions {

        Actions {  }

    }


    pub fn view(&self) -> Element<app::Message, iced::Renderer<theme::Theme>> {

        Row::new()
            .push(Space::new(5, 0))
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